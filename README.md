# AutoStarter

<!-- language-switcher -->
**English** · [Русский](docs/README.ru.md) · [中文](docs/README.zh.md) · [Español](docs/README.es.md) · [العربية](docs/README.ar.md) · [Português](docs/README.pt.md) · [Français](docs/README.fr.md) · [Deutsch](docs/README.de.md) · [日本語](docs/README.ja.md) · [हिन्दी](docs/README.hi.md)

![AutoStarter — your programs, your startup order. A Windows manager built with Tauri and Rust.](docs/images/hero.svg)

**A startup manager for Windows with window behaviour options and system tray support.**

AutoStarter (AutoStart Manager in the interface) launches the applications you select whenever it starts. Add the programs you need, configure their behaviour and enable autostart for the manager itself — your set of applications will then start when you sign in to Windows.

The project is built with **Tauri 1, Rust, TypeScript and Vite 5**.

> **Windows only.** The application uses the registry and the Win32 API; running and building the desktop version on Linux and macOS is not supported in the current implementation.

## Contents

- [Features](#features)
- [Quick start](#quick-start)
- [Program launch modes](#program-launch-modes)
- [Manager settings](#manager-settings)
- [Installing and building from source](#installing-and-building-from-source)
- [Data storage and Windows autostart](#data-storage-and-windows-autostart)
- [FAQ](#faq)
- [Project structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Your own program list:** add `.exe` files, `.lnk` shortcuts and `.bat` / `.cmd` scripts through the file picker dialog.
- **Independent toggles:** temporarily exclude a program from startup without removing it from the list.
- **Window control:** normal, minimised or hidden launch, plus sending a close command once the process is detected.
- **Start at Windows sign-in:** autostart of the manager for the current user.
- **Tray support:** hide the main window, restore it and quit through the icon's context menu.
- **Headless mode:** launch the enabled programs and quit the manager after 30 seconds.
- **Local settings:** the program list and options are stored in JSON files.
- **10 interface languages:** Russian, English, Chinese, Spanish, Arabic, Portuguese, French, German, Japanese and Hindi.

## Quick start

![Three steps: add programs and enable their cards, configure modes and the manager's autostart, sign in to Windows. Starting the manager manually also launches the enabled entries.](docs/images/workflow.svg)

If the application has not been built yet, follow the [build instructions](#installing-and-building-from-source) first.

1. Run `tauri-app.exe` — that is the current name of the project's executable.
2. Open the settings with the button in the bottom right corner and pick **Language → English**.
3. Click **"+ Add"** and select a program or script.
4. If needed, tick **"Minimized"**, **"Hidden"** or **"Close"** on the program card.
5. Switch the card's toggle to **"On"**. New entries are disabled by default.
6. In the settings enable **"Autostart"** if you want the manager and the selected programs to start when you sign in to Windows.

Changes are saved automatically. The delete button only removes the entry from the list — the program file stays on disk.

> **Important:** enabled programs are launched on every new start of the AutoStarter process, including manual ones. Adding an entry or switching it to "On" does not launch it immediately. To verify your setup, fully quit the manager via **Quit** in the tray and open it again. Applications that are already running may be started a second time.

## Program launch modes

| Mode on the card | Behaviour |
| --- | --- |
| No extra options | Normal launch through `cmd /c start`. |
| **Minimized** | Launch through `cmd /c start /min`, requesting a minimised window. |
| **Hidden** | Launch through PowerShell `Start-Process -WindowStyle Hidden`. This is a request to hide the window, not a guarantee that the program will appear in the tray. |
| **Close** | Normal launch, then a search for the process by file name for roughly 30 seconds. Once detected, the manager waits another 0.5 seconds and sends the system close command to the process's visible windows. |
| **Off / On** | Excludes or includes the entry in the launch list on the manager's next start. |

**Notes and limitations:**

- "Minimized" and "Hidden" are mutually exclusive in the interface.
- "Close" takes priority over both modes and does not mean force-terminating the process. The target application may close, minimise to its own tray, show a dialog or ignore the request.
- The search for "Close" is performed by file name, not by full path or by the identifier of the launched instance. The command may therefore affect an already running process with the same name. For shortcuts and scripts the selected file's name usually differs from the real process name.
- Some applications manage their own windows and may ignore a hidden or minimised launch.
- Configuring command-line arguments, the working directory and individual delays is not available in the interface.

Only add programs and scripts you trust. Do not use "Close" for applications with unsaved data without testing it first.

## Manager settings

| Setting | What it does |
| --- | --- |
| **Language** | Changes the interface language; English is selected by default. |
| **Start in tray** | Hides the main window when the regular version starts and shows an icon in the notification area. |
| **Autostart** | Adds AutoStarter to Windows autostart for the current user. Disabled by default. |
| **Close after run** | On Windows autostart adds the `--close-after-30` flag: the manager launches the enabled programs, waits 30 seconds and quits without creating the interface or the tray icon. Available only when "Autostart" is enabled. |

**"Close" on a card and "Close after run" in the settings are different features:** the first one applies to the windows of the selected program, the second one to the manager itself. The 30-second interval is fixed and does not mean waiting for all launched applications to finish.

A regular manual start without the flag opens the manager according to the "Start in tray" setting, even if "Close after run" is enabled.

### Tray control

- **The window's close button** hides the manager in the tray instead of quitting it.
- **Clicking the icon** or the **Show** item brings the main window back.
- **Right click → Quit** fully terminates the manager.

The tray menu item names are currently displayed in English regardless of the interface language.

## Installing and building from source

### Requirements

The graphical version requires the **Microsoft Edge WebView2 Runtime**. A practical build environment is Windows 10/11 with the following installed:

- [Node.js](https://nodejs.org/) **20 or newer** and npm; the current LTS release is recommended.
- [Rust](https://rustup.rs/) — a current stable toolchain for Windows MSVC. `Cargo.toml` declares a minimum of 1.70, but dependencies from the lock file may require a newer version.
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the **Desktop development with C++** workload, MSVC and the Windows SDK.
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/).
- [Git](https://git-scm.com/) to clone the repository.

More details: [setting up the Tauri 1 environment for Windows](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows).

### Getting the sources

Run the following in PowerShell:

```powershell
git clone https://github.com/BORGERone/AutoStarter.git
cd AutoStarter
npm ci
```

`npm ci` installs dependencies according to `package-lock.json`. The first build also needs network access to download Rust dependencies and packaging tools.

### Development mode

```powershell
npm run tauri-dev
```

Or run `dev.bat` from Explorer. Tauri automatically starts Vite on port **5174** and opens the desktop window.

> `npm run dev` starts only the web interface. Tauri features are unavailable in a regular browser: the native file picker, saving settings, tray and registry control. Use `npm run tauri-dev` for a full check of the application.

### Build with installers

```powershell
npm run tauri-build
```

The command builds the interface, the Rust application and the packages defined by the Tauri configuration. The results are located in:

- `src-tauri/target/release/tauri-app.exe` — the executable;
- `src-tauri/target/release/bundle/` — the installation packages created by the bundler.

### Build without an installer

```powershell
.\build.bat
```

The script runs `npm run build` and then `cargo build --release` in the `src-tauri` directory. The result is `src-tauri/target/release/tauri-app.exe`. The graphical interface still requires the WebView2 Runtime.

Running the headless mode manually:

```powershell
.\src-tauri\target\release\tauri-app.exe --close-after-30
```

### Checking the interface

```powershell
npm run build
```

The command runs the TypeScript check and the Vite production build. It does not check the Rust code or the Win32 API behaviour. There is currently no separate automated test script in `package.json`.

## Data storage and Windows autostart

The current user's settings are stored in the directory:

```text
%APPDATA%\tauri-launcher\
├── programs.json    # Program list, paths and launch modes
└── settings.json    # Language and manager settings
```

For a backup, fully quit the manager and copy both files. When moving to another computer, check the program paths. The autostart parameters in the registry are not part of this copy: after moving, enable autostart again from the instance of the application you want.

Enabling "Autostart" creates the value **`AutoStartManager`** in the key:

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
```

It stores the path to the manager's current executable and, if needed, the `--close-after-30` flag. This is a startup entry **for the current user's sign-in**, not a system service. Separate entries are not created for each added program; the manager does not manage the existing autostart entries of other applications.

**Disable "Autostart" before moving or deleting the executable.** After moving it, open the manager from its new location and enable the option again. For a complete reset, first disable autostart, quit the application via Quit, and then delete the `%APPDATA%\tauri-launcher` directory — this removes the saved list and settings.

## FAQ

**A program has been added but does not start.**

Check the "On" toggle on its card and make sure the file exists at the given path. Fully restart the manager: changing the list does not launch anything on its own. For a launch at Windows sign-in, "Autostart" must also be enabled in the manager settings.

**Applications open twice.**

AutoStarter does not check whether a program is already running. Check the program's own autostart setting and the Windows startup list. Restarting the manager itself also relaunches all enabled entries.

**The manager is still running after I clicked the close button.**

This is the expected behaviour. Find the AutoStart Manager icon in the notification area, expand the hidden icons if needed, and choose Quit.

**The application does not appear after I sign in to Windows.**

Check "Start in tray" and "Close after run": in the first case the window is hidden, in the second no interface is created at all. To change the settings, run the executable manually without the `--close-after-30` flag.

**The build cannot find the linker or the Windows SDK.**

Check the installation of the C++ Build Tools and the Rust MSVC toolchain, then restart the terminal. For window display problems, check that the WebView2 Runtime is present.

**Development mode reports that the port is busy.**

Free port 5174, for example by terminating the previous Vite instance. The port is used both in the Vite configuration and in the Tauri configuration.

## Project structure

```text
AutoStarter/
├── index.html                 # Markup of the main window and settings
├── src/
│   ├── main.ts                # Interface, translations and Tauri calls
│   └── styles.css             # Application styles
├── src-tauri/
│   ├── src/main.rs            # Program launching, JSON, registry and Win32 tray
│   ├── Cargo.toml             # Rust dependencies and settings
│   ├── Cargo.lock             # Pinned Rust dependencies
│   ├── build.rs               # Tauri build script
│   └── tauri.conf.json        # Window, permissions and packaging
├── icon.ico                   # Application and tray icon
├── package.json               # npm commands and interface dependencies
├── package-lock.json          # Pinned npm dependencies
├── vite.config.ts             # Vite settings
├── build.bat                  # Release build without an installer
└── dev.bat                    # Start development mode
```

## Contributing

Bug reports and suggestions can be submitted in [Issues](https://github.com/BORGERone/AutoStarter/issues). For a reproducible report, include your Windows version, how you ran or built the application, the steps and the expected behaviour. Remove personal data and private paths before publishing logs or JSON files.

For interface changes run `npm run build`; changes to launching, the registry and the tray should additionally be verified in the desktop version on Windows. A pull request should preferably come with a description of the changes and the checks you performed.

## License

`src-tauri/Cargo.toml` declares the **MIT** license. A separate `LICENSE` file with the full license text is not yet present in the repository.
