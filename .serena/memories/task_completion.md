# Task Completion Checklist

When completing a task, run these steps:

## 1. Format Code
```bash
# Format Rust code
cd src-tauri && cargo fmt
```

## 2. Lint
```bash
# Check for common mistakes and improvements
cd src-tauri && cargo clippy
```

## 3. Check Compilation
```bash
# Verify code compiles
cd src-tauri && cargo check
```

## 4. Build Frontend (if frontend changes)
```bash
npm run build
```

## 5. Test (manual)
```bash
# Run the full app in development mode
npm run tauri dev
```

## 6. Verify
- No compiler warnings/errors
- No clippy warnings
- App launches and basic functionality works

## Notes
- There are no automated tests currently in the project
- Manual testing is required for verification
- Ensure Windows-specific features work (clipboard, keyboard simulation)
