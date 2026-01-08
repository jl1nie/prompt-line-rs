# CLAUDE.md - prompt-line-rs

## Project Overview

prompt-line-rs is a pure Rust implementation of a floating text input tool for Windows, inspired by [prompt-line](https://github.com/nkmr-jp/prompt-line).

**Purpose**: Reduce stress when entering text in:
- CLI-based AI coding agents (Claude Code, Gemini CLI, etc.)
- Chat apps where Enter sends messages at unintended times
- Text editors with slow input response

## Core Features

1. **Quick Launch**: Global hotkey (default: Alt+Space) to instantly show floating window
2. **Quick Paste**: Ctrl+Enter to copy text and paste to the previously focused application
3. **Customizable Shortcuts**: Alt+Space, Win+Shift+Space, Ctrl+Shift+Space
4. **History**: Save and search previous inputs

## Architecture

```
src/
├── main.rs           # Entry point, egui app setup
├── app.rs            # Application state and logic
├── hotkey.rs         # Global hotkey registration (global-hotkey crate)
├── clipboard.rs      # Clipboard operations (arboard crate)
├── ui/
│   ├── mod.rs
│   ├── input_window.rs  # Floating input window
│   └── history_panel.rs # History search panel
├── config.rs         # Settings management (toml + serde)
└── history.rs        # History persistence (JSONL format)
```

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `eframe`/`egui` | GUI framework - lightweight, pure Rust |
| `global-hotkey` | System-wide hotkey registration |
| `arboard` | Cross-platform clipboard access |
| `serde` + `toml` | Configuration file parsing |
| `directories` | Platform-specific config paths |
| `chrono` | Timestamps for history |
| `windows` | Win32 API for keyboard simulation |

## Build Commands

### Development (Linux/WSL)
```bash
cargo check          # Syntax check
cargo build          # Debug build (Linux)
cargo clippy         # Lint
cargo fmt            # Format
```

### Cross-compile for Windows
```bash
# First time setup
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64

# Build
cargo build --target x86_64-pc-windows-gnu --release
```

### Output location
- Linux: `target/debug/prompt-line-rs`
- Windows: `target/x86_64-pc-windows-gnu/release/prompt-line-rs.exe`

## Configuration

Config file location: `~/.config/prompt-line-rs/config.toml` (or `%APPDATA%\prompt-line-rs\config.toml` on Windows)

```toml
[shortcuts]
launch = "Alt+Space"
paste = "Ctrl+Enter"
close = "Escape"

[history]
max_entries = 1000
```

## Coding Conventions

- Use `rustfmt` defaults
- Error handling: Use `Result` with descriptive error types
- No `unwrap()` in production code - use `expect()` with context or proper error handling
- Prefer explicit types over inference when it aids readability
- Document public APIs with rustdoc comments

## Implementation Notes

### Global Hotkey Flow
1. Register hotkey on app startup via `global-hotkey`
2. Listen for hotkey events in background
3. On trigger: show window, focus text input
4. On paste (Ctrl+Enter): copy to clipboard, hide window, simulate Ctrl+V in previous app

### Windows-specific
- Use `windows` crate for `SendInput` to simulate keyboard events
- Window should be topmost and capture focus immediately
- Consider handling IME for Japanese input

## TODO (Future Implementation)

- [ ] Global hotkey registration and handling
- [ ] Clipboard paste with keyboard simulation
- [ ] System tray icon
- [ ] History persistence and search
- [ ] Settings UI
- [ ] Auto-start on Windows boot
