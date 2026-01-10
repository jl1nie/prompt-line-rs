# prompt-line-rs - Project Overview

## Purpose
A floating text input tool for Windows, implemented in pure Rust using Tauri 2.0. Reduces stress when entering text in:
- CLI-based AI coding agents (Claude Code, Gemini CLI, etc.)
- Chat apps where Enter sends messages at unintended times
- Text editors with slow input response

## Core Features
- **Quick Launch**: Global hotkey (default: Ctrl+Shift+Space) to instantly show floating window
- **Quick Paste**: Ctrl+Enter to copy text and paste to the previously focused application
- **Customizable Shortcuts**: Multiple hotkey options with fallback support
- **History**: Save and search previous inputs (JSONL format)
- **System Tray**: Tray icon for quick access and management

## Tech Stack

### Backend (Rust)
| Crate | Purpose |
|-------|---------|
| `tauri` 2.x | Framework - native app with WebView frontend |
| `tauri-plugin-global-shortcut` 2.x | System-wide hotkey registration |
| `arboard` 3.x | Cross-platform clipboard access |
| `serde` + `toml` | Configuration file parsing |
| `directories` 5.x | Platform-specific config paths |
| `chrono` | Timestamps for history |
| `windows` 0.58 | Win32 API for keyboard simulation |

### Frontend (TypeScript)
| Package | Purpose |
|---------|---------|
| `@tauri-apps/api` 2.x | Tauri API bindings |
| `@tauri-apps/plugin-global-shortcut` 2.x | Shortcut plugin bindings |
| `vite` 6.x | Build tool |
| `typescript` 5.6 | Type safety |

## Project Structure
```
prompt-line-rs/
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── main.rs      # Entry point
│   │   ├── lib.rs       # Tauri app setup, commands, tray
│   │   ├── clipboard.rs # Clipboard operations + paste simulation
│   │   ├── config.rs    # Settings management (TOML)
│   │   └── history.rs   # History persistence (JSONL)
│   ├── Cargo.toml
│   └── tauri.conf.json  # Tauri configuration
├── src-frontend/        # TypeScript frontend
│   ├── main.ts          # UI logic
│   └── styles.css       # Styling
├── package.json         # npm dependencies
├── index.html           # HTML entry point
└── config.example.toml  # Example configuration
```

## Configuration
Config file location: `%APPDATA%\prompt-line-rs\config.toml` on Windows

Available options:
- Shortcuts: launch, paste, close hotkeys
- History: max_entries
- Window: width, height, opacity
