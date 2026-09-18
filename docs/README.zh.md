# AutoStarter

<!-- language-switcher -->
[English](../README.md) · [Русский](README.ru.md) · **中文** · [Español](README.es.md) · [العربية](README.ar.md) · [Português](README.pt.md) · [Français](README.fr.md) · [Deutsch](README.de.md) · [日本語](README.ja.md) · [हिन्दी](README.hi.md)

![AutoStarter — 你的程序，你的启动顺序。基于 Tauri 和 Rust 的 Windows 管理器。](images/hero.svg)

**适用于 Windows 的启动管理器，支持窗口行为设置和系统托盘。**

AutoStarter（界面中显示为 AutoStart Manager）会在自身启动时运行你选择的应用程序。添加所需的程序、配置它们的行为，并为管理器本身启用自启动 —— 这样你的应用集合就会在登录 Windows 时启动。

本项目基于 **Tauri 1、Rust、TypeScript 和 Vite 5** 构建。

> **仅限 Windows。** 应用使用注册表和 Win32 API；当前实现不支持在 Linux 和 macOS 上运行或构建桌面版本。

## 目录

- [功能](#功能)
- [快速开始](#快速开始)
- [程序启动模式](#程序启动模式)
- [管理器设置](#管理器设置)
- [从源码安装与构建](#从源码安装与构建)
- [数据存储与 Windows 自启动](#数据存储与-windows-自启动)
- [常见问题](#常见问题)
- [项目结构](#项目结构)
- [参与开发](#参与开发)
- [许可证](#许可证)

## 功能

- **自定义程序列表：** 通过文件选择对话框添加 `.exe`、`.lnk` 快捷方式以及 `.bat` / `.cmd` 脚本。
- **独立开关：** 临时将某个程序排除在启动之外，而无需将其从列表中删除。
- **窗口控制：** 正常、最小化或隐藏启动，也可在检测到进程后发送关闭命令。
- **登录 Windows 时启动：** 为当前用户自启动管理器。
- **托盘支持：** 通过图标的右键菜单隐藏主窗口、恢复窗口和退出。
- **无界面模式：** 启动已启用的程序，并在 30 秒后关闭管理器。
- **本地设置：** 程序列表和参数保存在 JSON 文件中。
- **10 种界面语言：** 俄语、英语、中文、西班牙语、阿拉伯语、葡萄牙语、法语、德语、日语和印地语。

## 快速开始

![三个步骤：添加程序并启用它们的卡片，配置模式和管理器自启动，登录 Windows。手动启动管理器同样会运行已启用的条目。](images/workflow.svg)

如果应用尚未构建，请先按照[构建说明](#从源码安装与构建)操作。

1. 运行 `tauri-app.exe` —— 这是本项目可执行文件当前的名称。
2. 用右下角的按钮打开设置，选择 **Language → 中文**。
3. 点击 **“+ 添加”** 并选择程序或脚本。
4. 如有需要，在程序卡片上勾选 **“最小化”**、**“隐藏”** 或 **“关闭”**。
5. 将卡片的开关切换到 **“开”**。新条目默认处于关闭状态。
6. 如果希望管理器与所选程序在登录 Windows 时一起启动，请在设置中启用 **“自启动”**。

更改会自动保存。删除按钮只会从列表中移除条目 —— 程序文件仍保留在磁盘上。

> **重要：** 已启用的程序会在 AutoStarter 进程每次重新启动时运行，手动启动也不例外。添加条目或将其切换为“开”并不会立即运行它。若要验证配置，请通过托盘中的 **Quit** 完全退出管理器，然后重新打开。已在运行的应用可能会被再次启动。

## 程序启动模式

| 卡片中的模式 | 行为 |
| --- | --- |
| 不额外勾选 | 通过 `cmd /c start` 正常启动。 |
| **最小化** | 通过 `cmd /c start /min` 启动，请求最小化窗口。 |
| **隐藏** | 通过 PowerShell `Start-Process -WindowStyle Hidden` 启动。这只是隐藏窗口的请求，并不保证程序会出现在托盘中。 |
| **关闭** | 正常启动，然后按文件名搜索进程约 30 秒。检测到后管理器再等待 0.5 秒，并向该进程的可见窗口发送系统关闭命令。 |
| **关 / 开** | 在管理器下次启动时将该条目排除或纳入启动列表。 |

**特性与限制：**

- “最小化”和“隐藏”在界面中互斥。
- “关闭”的优先级高于上述两种模式，且并不意味着强制结束进程。目标应用可能会关闭、最小化到自己的托盘、显示对话框，或忽略该请求。
- “关闭”的搜索按文件名进行，而不是按完整路径或已启动实例的标识符。因此该命令可能影响同名的、已在运行的进程。对于快捷方式和脚本，所选文件的名称通常与真实进程名不一致。
- 某些应用会自行管理窗口，可能忽略隐藏或最小化启动。
- 界面中不提供命令行参数、工作目录和单独延迟的设置。

只添加你信任的程序和脚本。在未经事先验证的情况下，请勿对含有未保存数据的应用使用“关闭”。

## 管理器设置

| 设置 | 作用 |
| --- | --- |
| **语言** | 更改界面语言；默认选择英语。 |
| **启动到托盘** | 在普通版本启动时隐藏主窗口，并在通知区域显示图标。 |
| **自启动** | 为当前用户将 AutoStarter 添加到 Windows 自启动。默认关闭。 |
| **运行后关闭** | 在 Windows 自启动时添加 `--close-after-30` 标志：管理器启动已启用的程序，等待 30 秒后退出，不创建界面和托盘图标。仅在启用“自启动”时可用。 |

**卡片中的“关闭”与设置中的“运行后关闭”是不同的功能：** 前者作用于所选程序的窗口，后者作用于管理器本身。30 秒的间隔是固定的，并不意味着等待所有已启动的应用结束。

即使启用了“运行后关闭”，不带标志的普通手动启动仍会按照“启动到托盘”的设置打开管理器。

### 通过托盘控制

- **窗口的关闭按钮**会将管理器隐藏到托盘，而不是结束它。
- **单击图标**或 **Show** 菜单项可恢复主窗口。
- **右键点击 → Quit** 会完全退出管理器。

托盘菜单项的名称目前始终以英文显示，与界面语言无关。

## 从源码安装与构建

### 环境要求

运行图形版本需要 **Microsoft Edge WebView2 Runtime**。实用的构建环境是安装了以下组件的 Windows 10/11：

- [Node.js](https://nodejs.org/) **20 或更高版本**以及 npm；建议使用当前的 LTS 版本。
- [Rust](https://rustup.rs/) —— 适用于 Windows MSVC 的当前 stable 工具链。`Cargo.toml` 中声明的最低版本为 1.70，但 lock 文件中的依赖可能需要更新的版本。
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)，包含 **Desktop development with C++** 组件、MSVC 和 Windows SDK。
- [Microsoft Edge WebView2 Runtime](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)。
- [Git](https://git-scm.com/)，用于克隆仓库。

更多信息：[为 Windows 准备 Tauri 1 环境](https://v1.tauri.app/v1/guides/getting-started/prerequisites/#setting-up-windows)。

### 获取源码

在 PowerShell 中执行：

```powershell
git clone https://github.com/BORGERone/AutoStarter.git
cd AutoStarter
npm ci
```

`npm ci` 会根据 `package-lock.json` 安装依赖。首次构建还需要网络访问，以下载 Rust 依赖和打包工具。

### 开发模式

```powershell
npm run tauri-dev
```

或者在资源管理器中运行 `dev.bat`。Tauri 会自动在端口 **5174** 上启动 Vite 并打开桌面窗口。

> `npm run dev` 只会启动 Web 界面。在普通浏览器中无法使用 Tauri 功能：原生对话框选择文件、保存设置、托盘和注册表控制。请使用 `npm run tauri-dev` 对应用进行完整验证。

### 构建并生成安装包

```powershell
npm run tauri-build
```

该命令会构建界面、Rust 应用以及 Tauri 配置所定义的软件包。结果位于：

- `src-tauri/target/release/tauri-app.exe` —— 可执行文件；
- `src-tauri/target/release/bundle/` —— 打包器生成的安装包。

### 构建（不生成安装包）

```powershell
.\build.bat
```

该脚本会依次在 `src-tauri` 目录中执行 `npm run build` 和 `cargo build --release`。结果为 `src-tauri/target/release/tauri-app.exe`。图形界面仍然需要 WebView2 Runtime。

手动运行无界面模式：

```powershell
.\src-tauri\target\release\tauri-app.exe --close-after-30
```

### 检查界面

```powershell
npm run build
```

该命令会执行 TypeScript 检查和 Vite 生产构建。它不会检查 Rust 代码和 Win32 API 行为。目前 `package.json` 中没有单独的自动化测试脚本。

## 数据存储与 Windows 自启动

当前用户的设置保存在以下目录：

```text
%APPDATA%\tauri-launcher\
├── programs.json    # 程序列表、路径和启动模式
└── settings.json    # 语言和管理器设置
```

若要备份，请完全退出管理器并复制这两个文件。迁移到另一台计算机时，请检查程序路径。注册表中的自启动参数不包含在此备份中：迁移后请从需要的应用实例重新启用自启动。

启用“自启动”时，会在以下注册表项中创建值 **`AutoStartManager`**：

```text
HKEY_CURRENT_USER\Software\Microsoft\Windows\CurrentVersion\Run
```

其中保存管理器当前可执行文件的路径，必要时还包括 `--close-after-30` 标志。这是**当前用户登录时**的启动项，而不是系统服务。不会为每个添加的程序创建单独的条目；管理器也不会管理其他应用已有的自启动项。

**在移动或删除可执行文件之前，请先关闭“自启动”。** 移动之后，请从新位置打开管理器并重新启用。若要彻底重置，请先关闭自启动，通过 Quit 退出应用，然后删除 `%APPDATA%\tauri-launcher` 目录 —— 这会删除已保存的列表和设置。

## 常见问题

**程序已添加，但没有启动。**

检查其卡片上的“开”开关，以及指定路径下的文件是否存在。完全重启管理器：仅更改列表本身不会启动任何程序。若要在登录 Windows 时启动，管理器设置中的“自启动”也必须开启。

**应用被打开了两次。**

AutoStarter 不会检查程序是否已在运行。请检查程序自身的自启动设置以及 Windows 启动项列表。重新启动管理器本身也会再次运行所有已启用的条目。

**点击关闭按钮后管理器仍在运行。**

这是预期行为。请在通知区域找到 AutoStart Manager 图标（必要时展开隐藏图标），然后选择 Quit。

**登录 Windows 后应用没有出现。**

请检查“启动到托盘”和“运行后关闭”：前者会隐藏窗口，后者则完全不创建界面。若要更改设置，请不带 `--close-after-30` 标志手动运行可执行文件。

**构建时找不到链接器或 Windows SDK。**

请检查 C++ Build Tools 和 Rust MSVC 工具链的安装情况，然后重启终端。若窗口显示有问题，请检查是否安装了 WebView2 Runtime。

**开发模式提示端口被占用。**

请释放端口 5174，例如结束之前的 Vite 实例。该端口同时用于 Vite 配置和 Tauri 配置。

## 项目结构

```text
AutoStarter/
├── index.html                 # 主窗口和设置的标记
├── src/
│   ├── main.ts                # 界面、翻译和 Tauri 调用
│   └── styles.css             # 应用样式
├── src-tauri/
│   ├── src/main.rs            # 程序启动、JSON、注册表和 Win32 托盘
│   ├── Cargo.toml             # Rust 依赖和参数
│   ├── Cargo.lock             # 锁定的 Rust 依赖
│   ├── build.rs               # Tauri 构建脚本
│   └── tauri.conf.json        # 窗口、权限和打包
├── icon.ico                   # 应用和托盘图标
├── package.json               # npm 命令和界面依赖
├── package-lock.json          # 锁定的 npm 依赖
├── vite.config.ts             # Vite 配置
├── build.bat                  # 不含安装包的 Release 构建
└── dev.bat                    # 启动开发模式
```

## 参与开发

错误报告和建议可以提交到 [Issues](https://github.com/BORGERone/AutoStarter/issues)。为了便于复现，请注明 Windows 版本、运行或构建方式、操作步骤以及预期行为。在发布日志或 JSON 文件之前，请删除个人数据和私有路径。

对于界面改动，请执行 `npm run build`；涉及启动、注册表和托盘的改动，还应在 Windows 桌面版本中额外验证。Pull request 最好附带改动说明和已完成的验证。

## 许可证

`src-tauri/Cargo.toml` 中声明的许可证为 **MIT**。仓库中目前还没有包含完整许可证文本的独立 `LICENSE` 文件。
