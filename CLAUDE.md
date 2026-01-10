# CLAUDE.md - prompt-line-rs

## Project Overview

prompt-line-rs is a Tauri 2 application providing a floating text input tool for Windows, inspired by [prompt-line](https://github.com/nkmr-jp/prompt-line).

**Purpose**: Reduce stress when entering text in:
- CLI-based AI coding agents (Claude Code, Gemini CLI, etc.)
- Chat apps where Enter sends messages at unintended times
- Text editors with slow input response

## Core Features

1. **Quick Launch**: Global hotkey (default: Alt+Space) to instantly show floating window
2. **Quick Paste**: Ctrl+Enter to copy text and paste to the previously focused application
3. **Customizable Shortcuts**: Alt+Space, Win+Shift+Space, Ctrl+Shift+Space
4. **History**: Save and search previous inputs with Ctrl+J/K navigation
5. **Settings UI**: Configure shortcuts and behavior
6. **System Tray**: Minimize to tray with context menu

## Architecture

```
prompt-line-rs/
├── src-tauri/                # Rust backend (Tauri 2)
│   ├── src/
│   │   ├── main.rs           # Entry point
│   │   ├── lib.rs            # Tauri commands and app setup
│   │   ├── config.rs         # Settings management (toml + serde)
│   │   ├── clipboard.rs      # Clipboard + keyboard simulation
│   │   └── history.rs        # History persistence (JSONL)
│   ├── capabilities/         # Tauri permissions
│   ├── icons/                # App icons
│   ├── nsis/                 # Windows installer customization
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src-frontend/             # TypeScript frontend
│   ├── main.ts               # Main window logic
│   ├── settings.ts           # Settings window logic
│   ├── styles.css            # Main window styles
│   └── settings.css          # Settings window styles
├── index.html                # Main window entry point
├── settings.html             # Settings window entry point
├── dist/                     # Vite build output (auto-generated)
├── vite.config.ts            # Vite multi-page config
├── package.json
└── tsconfig.json
```

## Key Dependencies

### Rust (src-tauri/Cargo.toml)
| Crate | Purpose |
|-------|---------|
| `tauri` | Application framework with tray-icon support |
| `tauri-plugin-global-shortcut` | System-wide hotkey registration |
| `arboard` | Cross-platform clipboard access |
| `serde` + `toml` | Configuration file parsing |
| `directories` | Platform-specific config paths |
| `chrono` | Timestamps for history |
| `windows` | Win32 API for keyboard simulation |

### Frontend (package.json)
| Package | Purpose |
|---------|---------|
| `@tauri-apps/api` | Tauri JavaScript API |
| `@tauri-apps/plugin-global-shortcut` | Shortcut plugin frontend |
| `vite` | Build tool |
| `typescript` | Type checking |

## Build Commands

### Development
```bash
npm run tauri dev     # Start dev server with hot reload
```

### Production Build
```bash
npm run tauri build   # Build for Windows (creates installer)
```

### Rust-only Commands
```bash
cd src-tauri
cargo check           # Syntax check
cargo clippy          # Lint
cargo fmt             # Format
```

### Output Location
- Dev: `src-tauri/target/debug/prompt-line-rs.exe`
- Release: `src-tauri/target/release/prompt-line-rs.exe`
- Installer: `src-tauri/target/release/bundle/nsis/`

## Configuration

Config file location: `%APPDATA%\prompt-line-rs\config.toml`

```toml
[shortcuts]
launch = "Alt+Space"

[behavior]
readline_bindings = true  # Ctrl+A/E for line start/end
```

## Coding Conventions

### Rust
- Use `rustfmt` defaults
- Error handling: Use `Result` with descriptive error types
- No `unwrap()` in production code - use `expect()` with context or proper error handling
- Prefer explicit types over inference when it aids readability

### TypeScript
- Use TypeScript strict mode
- Async/await for Tauri invoke calls
- Event-driven communication with backend

## Implementation Notes

### Global Hotkey Flow
1. Register hotkey on app startup via `tauri-plugin-global-shortcut`
2. Listen for hotkey events
3. On trigger: show window, focus text input
4. On paste (Ctrl+Enter): copy to clipboard, hide window, simulate Ctrl+V

### Windows-specific
- Use `windows` crate for `SendInput` to simulate keyboard events
- Window is always-on-top and captures focus immediately
- System tray with context menu for settings/quit

### Multi-window Setup
- Main window: Text input with history
- Settings window: Created on demand via `WebviewWindowBuilder`
