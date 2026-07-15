@echo off
cd /d "%~dp0"
echo Building frontend...
call npm run build
if %errorlevel% neq 0 exit /b %errorlevel%
cd src-tauri
echo Building Rust release...
call cargo build --release
if %errorlevel% neq 0 exit /b %errorlevel%
echo Done! Binary: src-tauri\target\release\tauri-app.exe