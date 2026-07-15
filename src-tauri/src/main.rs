#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::{Manager, State};
use uuid::Uuid;
use winreg::enums::*;
use winreg::RegKey;
use windows_sys::Win32::Foundation::{BOOL, HWND, LPARAM, LRESULT, POINT, TRUE, WPARAM};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W,
    TH32CS_SNAPPROCESS,
};
use windows_sys::Win32::UI::Shell::{
    NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW, Shell_NotifyIconW,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu,
    DestroyWindow, DispatchMessageW, EnumWindows, GetCursorPos, GetMessageW,
    GetWindowLongPtrW, GetWindowThreadProcessId, IsWindowVisible, LoadImageW,
    PostMessageW, RegisterClassW, SendMessageW,
    SetForegroundWindow, SetWindowLongPtrW, TranslateMessage, GWLP_USERDATA,
    IMAGE_ICON, LR_LOADFROMFILE, MF_STRING, TPM_NONOTIFY, TPM_RETURNCMD,
    TrackPopupMenu, WM_LBUTTONDBLCLK, WM_LBUTTONUP, WM_RBUTTONUP, WM_SYSCOMMAND,
    WM_USER, WNDCLASSW,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Program {
    id: String,
    name: String,
    path: String,
    enabled: bool,
    run_minimized: bool,
    run_hidden_tray: bool,
    auto_close: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AppSettings {
    language: String,
    launch_minimized: bool,
    app_autostart: bool,
    close_after_execution: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "en".to_string(),
            launch_minimized: false,
            app_autostart: false,
            close_after_execution: false,
        }
    }
}

struct AppState {
    programs: Mutex<Vec<Program>>,
    config_dir: PathBuf,
}

const APP_REGISTRY_PATH: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const APP_REGISTRY_KEY: &str = "AutoStartManager";
const SC_CLOSE: usize = 0xF060;

const TRAY_CALLBACK_MSG: u32 = WM_USER + 1000;
const TRAY_ADD_ICON: u32 = WM_USER + 100;
const TRAY_REMOVE_ICON: u32 = WM_USER + 101;
const TRAY_ICON_ID: u32 = 1;
const TRAY_WINDOW_CLASS: &str = "AutoStartTrayWindow";

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();
static TRAY_ACTIVE: AtomicBool = AtomicBool::new(false);
static TRAY_HWND: Mutex<Option<HWND>> = Mutex::new(None);

fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}

fn get_config_dir() -> PathBuf {
    let base = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    let dir = base.join("tauri-launcher");
    let _ = fs::create_dir_all(&dir);
    dir
}

fn load_programs(config_dir: &PathBuf) -> Vec<Program> {
    let path = config_dir.join("programs.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn save_programs_file(config_dir: &PathBuf, programs: &[Program]) {
    let path = config_dir.join("programs.json");
    if let Ok(data) = serde_json::to_string_pretty(programs) {
        let _ = fs::write(path, data);
    }
}

fn load_settings(config_dir: &PathBuf) -> AppSettings {
    let path = config_dir.join("settings.json");
    fs::read_to_string(path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

fn write_settings_file(config_dir: &PathBuf, settings: &AppSettings) {
    let path = config_dir.join("settings.json");
    if let Ok(data) = serde_json::to_string_pretty(settings) {
        let _ = fs::write(path, data);
    }
}

fn extract_name(path: &str) -> String {
    PathBuf::from(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn find_pid_by_name(exe_name: &str) -> Option<u32> {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot as isize == -1 {
            return None;
        }
        let mut entry = PROCESSENTRY32W {
            dwSize: std::mem::size_of::<PROCESSENTRY32W>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0u16; 260],
        };

        if Process32FirstW(snapshot, &mut entry) == TRUE {
            loop {
                let name = String::from_utf16_lossy(&entry.szExeFile[..])
                    .trim_end_matches('\0')
                    .to_lowercase();
                if name == exe_name {
                    return Some(entry.th32ProcessID);
                }
                if Process32NextW(snapshot, &mut entry) != TRUE {
                    break;
                }
            }
        }
        None
    }
}

fn get_exe_name(path: &str) -> String {
    PathBuf::from(path)
        .file_name()
        .map(|s| s.to_string_lossy().to_lowercase())
        .unwrap_or_default()
}

unsafe extern "system" fn close_enum_callback(hwnd: HWND, lparam: isize) -> BOOL {
    let target_pid = lparam as u32;
    let mut pid: u32 = 0;
    GetWindowThreadProcessId(hwnd, &mut pid);
    if pid == target_pid && IsWindowVisible(hwnd) == TRUE {
        SendMessageW(hwnd, WM_SYSCOMMAND, SC_CLOSE, 0);
    }
    TRUE
}

fn send_close_to_process(pid: u32) {
    unsafe {
        EnumWindows(Some(close_enum_callback), pid as isize);
    }
}

fn launch_program(program: &Program) {
    let path = &program.path;

    if program.auto_close {
        let path_owned = program.path.clone();
        let exe_name = get_exe_name(&path_owned);
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "", path])
            .spawn();

        std::thread::spawn(move || {
            for _ in 0..15 {
                std::thread::sleep(Duration::from_secs(2));
                if let Some(pid) = find_pid_by_name(&exe_name) {
                    std::thread::sleep(Duration::from_millis(500));
                    send_close_to_process(pid);
                    break;
                }
            }
        });
    } else if program.run_hidden_tray {
        let _ = std::process::Command::new("powershell")
            .args(["-WindowStyle", "Hidden", "-Command",
                &format!("Start-Process -WindowStyle Hidden '{}'", path.replace('\'', "''"))])
            .spawn();
    } else if program.run_minimized {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "/min", "", path])
            .spawn();
    } else {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", "", path])
            .spawn();
    }
}

#[tauri::command]
fn get_programs(state: State<AppState>) -> Vec<Program> {
    state.programs.lock().unwrap().clone()
}

#[tauri::command]
fn add_program(path: String, state: State<AppState>) -> Result<Vec<Program>, String> {
    let path_trimmed = path.trim().trim_matches('"').to_string();
    if path_trimmed.is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let pb = PathBuf::from(&path_trimmed);
    if !pb.exists() {
        return Err("File does not exist".to_string());
    }

    let program = Program {
        id: Uuid::new_v4().to_string(),
        name: extract_name(&path_trimmed),
        path: path_trimmed,
        enabled: false,
        run_minimized: false,
        run_hidden_tray: false,
        auto_close: false,
    };

    let mut programs = state.programs.lock().unwrap();
    programs.push(program);
    save_programs_file(&state.config_dir, &programs);
    Ok(programs.clone())
}

#[tauri::command]
fn toggle_program(id: String, enabled: bool, state: State<AppState>) -> Result<Vec<Program>, String> {
    let mut programs = state.programs.lock().unwrap();
    if let Some(p) = programs.iter_mut().find(|p| p.id == id) {
        p.enabled = enabled;
        save_programs_file(&state.config_dir, &programs);
        Ok(programs.clone())
    } else {
        Err("Program not found".to_string())
    }
}

#[tauri::command]
fn update_program_options(
    id: String,
    run_minimized: bool,
    run_hidden_tray: bool,
    auto_close: bool,
    state: State<AppState>,
) -> Result<Vec<Program>, String> {
    let mut programs = state.programs.lock().unwrap();
    if let Some(p) = programs.iter_mut().find(|p| p.id == id) {
        p.run_minimized = run_minimized;
        p.run_hidden_tray = run_hidden_tray;
        p.auto_close = auto_close;
        save_programs_file(&state.config_dir, &programs);
        Ok(programs.clone())
    } else {
        Err("Program not found".to_string())
    }
}

#[tauri::command]
fn remove_program(id: String, state: State<AppState>) -> Result<Vec<Program>, String> {
    let mut programs = state.programs.lock().unwrap();
    let len_before = programs.len();
    programs.retain(|p| p.id != id);
    if programs.len() < len_before {
        save_programs_file(&state.config_dir, &programs);
        Ok(programs.clone())
    } else {
        Err("Program not found".to_string())
    }
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> AppSettings {
    load_settings(&state.config_dir)
}

#[tauri::command]
fn save_settings(settings: AppSettings, state: State<AppState>) {
    write_settings_file(&state.config_dir, &settings);
    sync_autostart(&settings);
}

fn sync_autostart(settings: &AppSettings) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if settings.app_autostart {
        if let Ok(key) = hkcu.open_subkey_with_flags(APP_REGISTRY_PATH, KEY_SET_VALUE) {
            if let Ok(exe_path) = std::env::current_exe() {
                let value = if settings.close_after_execution {
                    format!("\"{}\" --close-after-30", exe_path.to_string_lossy())
                } else {
                    format!("\"{}\"", exe_path.to_string_lossy())
                };
                let _ = key.set_value(APP_REGISTRY_KEY, &value);
            }
        }
    } else {
        if let Ok(key) = hkcu.open_subkey_with_flags(APP_REGISTRY_PATH, KEY_SET_VALUE) {
            let _ = key.delete_value(APP_REGISTRY_KEY);
        }
    }
}

#[tauri::command]
fn hide_to_tray(window: tauri::Window) {
    let _ = window.hide();
    show_tray_icon();
}

fn show_tray_icon() {
    if TRAY_ACTIVE.swap(true, Ordering::SeqCst) {
        return;
    }
    let hwnd = TRAY_HWND.lock().unwrap();
    if let Some(hwnd) = *hwnd {
        unsafe {
            PostMessageW(hwnd, TRAY_ADD_ICON, 0, 0);
        }
    } else {
        TRAY_ACTIVE.store(false, Ordering::SeqCst);
    }
}

fn start_tray_thread() {
    let app_handle = APP_HANDLE.get().cloned();
    if app_handle.is_none() {
        return;
    }
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let barrier_clone = barrier.clone();
    std::thread::spawn(move || {
        unsafe {
            let class_name = to_wide(TRAY_WINDOW_CLASS);
            let hinstance = GetModuleHandleW(std::ptr::null());
            let wc = WNDCLASSW {
                style: 0,
                lpfnWndProc: Some(tray_wndproc),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: hinstance,
                hIcon: 0,
                hCursor: 0,
                hbrBackground: 0,
                lpszMenuName: std::ptr::null(),
                lpszClassName: class_name.as_ptr(),
            };
            RegisterClassW(&wc);

            let hwnd = CreateWindowExW(
                0,
                class_name.as_ptr(),
                std::ptr::null(),
                0,
                0, 0, 0, 0,
                0,
                0,
                hinstance,
                std::ptr::null_mut(),
            );

            if hwnd == 0 {
                barrier_clone.wait();
                return;
            }

            let app_ptr = Box::into_raw(Box::new(app_handle.unwrap()));
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, app_ptr as _);

            *TRAY_HWND.lock().unwrap() = Some(hwnd);

            barrier_clone.wait();

            let mut msg = std::mem::zeroed();
            while GetMessageW(&mut msg, 0, 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            *TRAY_HWND.lock().unwrap() = None;
            TRAY_ACTIVE.store(false, Ordering::SeqCst);
            let mut nid_del: NOTIFYICONDATAW = std::mem::zeroed();
            nid_del.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid_del.hWnd = hwnd;
            nid_del.uID = TRAY_ICON_ID;
            Shell_NotifyIconW(NIM_DELETE, &nid_del);
            DestroyWindow(hwnd);
            let _ = Box::from_raw(app_ptr);
        }
    });
    barrier.wait();
}

const ICON_DATA: &[u8] = include_bytes!("../../icon.ico");

unsafe fn load_tray_icon() -> isize {
    let temp_icon = std::env::temp_dir().join("tauri-launcher-icon.ico");
    let _ = std::fs::write(&temp_icon, ICON_DATA);
    let icon_wide: Vec<u16> =
        temp_icon.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
    LoadImageW(
        GetModuleHandleW(std::ptr::null()),
        icon_wide.as_ptr(),
        IMAGE_ICON,
        16,
        16,
        LR_LOADFROMFILE,
    ) as isize
}

unsafe extern "system" fn tray_wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if msg == TRAY_ADD_ICON {
        if TRAY_ACTIVE.load(Ordering::SeqCst) {
            let hicon = load_tray_icon();
            let mut nid: NOTIFYICONDATAW = std::mem::zeroed();
            nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
            nid.hWnd = hwnd;
            nid.uID = TRAY_ICON_ID;
            nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
            nid.uCallbackMessage = TRAY_CALLBACK_MSG;
            nid.hIcon = hicon;
            let tip = to_wide("AutoStart Manager");
            let tip_len = tip.len().min(128);
            let mut i = 0;
            while i < tip_len {
                nid.szTip[i] = tip[i];
                i += 1;
            }
            Shell_NotifyIconW(NIM_ADD, &nid);
        }
        return 0;
    }
    if msg == TRAY_REMOVE_ICON {
        TRAY_ACTIVE.store(false, Ordering::SeqCst);
        let mut nid_del: NOTIFYICONDATAW = std::mem::zeroed();
        nid_del.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
        nid_del.hWnd = hwnd;
        nid_del.uID = TRAY_ICON_ID;
        Shell_NotifyIconW(NIM_DELETE, &nid_del);
        return 0;
    }
    if msg == TRAY_CALLBACK_MSG {
        match lparam as u32 {
            WM_LBUTTONUP | WM_LBUTTONDBLCLK => {
                let app_ptr = (GetWindowLongPtrW(hwnd, GWLP_USERDATA)) as *mut tauri::AppHandle;
                if let Some(app) = app_ptr.as_ref() {
                    if let Some(window) = app.get_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.unminimize();
                    }
                }
                PostMessageW(hwnd, TRAY_REMOVE_ICON, 0, 0);
                return 0;
            }
            WM_RBUTTONUP => {
                let hmenu = CreatePopupMenu();
                if hmenu != 0 {
                    let show_str = to_wide("Show");
                    let quit_str = to_wide("Quit");
                    AppendMenuW(hmenu, MF_STRING, 1, show_str.as_ptr());
                    AppendMenuW(hmenu, MF_STRING, 2, quit_str.as_ptr());
                    let mut pos = POINT { x: 0, y: 0 };
                    GetCursorPos(&mut pos);
                    SetForegroundWindow(hwnd);
                    let cmd = TrackPopupMenu(
                        hmenu,
                        TPM_RETURNCMD | TPM_NONOTIFY,
                        pos.x,
                        pos.y,
                        0,
                        hwnd,
                        std::ptr::null_mut(),
                    );
                    DestroyMenu(hmenu);
                    match cmd {
                        1 => {
                            let app_ptr =
                                (GetWindowLongPtrW(hwnd, GWLP_USERDATA)) as *mut tauri::AppHandle;
                            if let Some(app) = app_ptr.as_ref() {
                                if let Some(window) = app.get_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    let _ = window.unminimize();
                                }
                            }
                            PostMessageW(hwnd, TRAY_REMOVE_ICON, 0, 0);
                        }
                        2 => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                return 0;
            }
            _ => {}
        }
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
}

fn main() {
    let close_after_30 = std::env::args().any(|a| a == "--close-after-30");

    let config_dir = get_config_dir();
    let settings = load_settings(&config_dir);
    let programs = load_programs(&config_dir);

    for p in &programs {
        if p.enabled {
            launch_program(p);
        }
    }

    if close_after_30 {
        std::thread::sleep(Duration::from_secs(30));
        return;
    }

    let app = tauri::Builder::default()
        .manage(AppState {
            programs: Mutex::new(programs),
            config_dir,
        })
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                let _ = event.window().hide();
                show_tray_icon();
                api.prevent_close();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_programs,
            add_program,
            toggle_program,
            update_program_options,
            remove_program,
            get_settings,
            save_settings,
            hide_to_tray,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    APP_HANDLE.set(app.app_handle()).unwrap();
    start_tray_thread();

    if settings.launch_minimized {
        show_tray_icon();
        if let Some(window) = app.get_window("main") {
            let _ = window.hide();
        }
    }

    app.run(|_app_handle, _event| {});
    // After app.run returns, tray thread is still running.
    // Process exits, OS cleans up.
}
