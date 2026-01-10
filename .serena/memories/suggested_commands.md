# Suggested Commands

## Development

### Frontend (npm/vite)
```bash
# Install dependencies
npm install

# Start dev server (frontend only)
npm run dev

# Build frontend
npm run build
```

### Tauri (full app)
```bash
# Run development mode (starts both frontend and Tauri)
npm run tauri dev

# Build release
npm run tauri build
```

### Rust/Cargo
```bash
# Check syntax (in src-tauri directory)
cd src-tauri && cargo check

# Lint with clippy
cd src-tauri && cargo clippy

# Format code
cd src-tauri && cargo fmt

# Build (debug)
cd src-tauri && cargo build

# Build (release)
cd src-tauri && cargo build --release
```

### Cross-compilation (from Linux/WSL)
```bash
# Add Windows target (first time)
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64

# Build for Windows
cargo build --target x86_64-pc-windows-gnu --release
```

## Output Locations
- Development: `src-tauri/target/debug/`
- Release: `src-tauri/target/release/`
- Installer: `src-tauri/target/release/bundle/`

## Windows Utilities (PowerShell/CMD equivalent)
```powershell
# List directory
dir
Get-ChildItem

# Find files
Get-ChildItem -Recurse -Filter "*.rs"

# Search in files
Select-String -Path "*.rs" -Pattern "pattern"

# Git commands (same as Unix)
git status
git diff
git log
```
