# Code Style and Conventions

## Rust

### Formatting
- Use `rustfmt` defaults (no custom rustfmt.toml)
- Run `cargo fmt` before committing

### Error Handling
- Use `Result` with descriptive error types
- **No `unwrap()` in production code** - use `expect()` with context or proper error handling
- Return `Result<T, String>` from Tauri commands for frontend error handling

### Documentation
- Document public APIs with rustdoc comments (`///`)
- Use `//!` for module-level documentation
- Example from codebase:
  ```rust
  /// Get history entries, optionally filtered by query
  #[tauri::command]
  fn get_history(query: String, state: tauri::State<'_, AppState>) -> Vec<history::HistoryEntry> {
      // ...
  }
  ```

### Type Annotations
- Prefer explicit types over inference when it aids readability
- Always annotate function signatures

### Tauri Commands
- Use `#[tauri::command]` attribute
- Access app state via `tauri::State<'_, AppState>`
- Return `Result<T, String>` for error handling

## TypeScript

### General
- Use TypeScript 5.6+
- Use ES modules (`"type": "module"` in package.json)

### Imports
- Use `@tauri-apps/api` and plugin packages for Tauri integration

## Project Patterns

### State Management (Rust)
- Use `Mutex<T>` for shared mutable state
- Store in `AppState` struct managed by Tauri

### Configuration
- TOML format for config files
- Serde for serialization/deserialization
- Default values with fallback

### History Storage
- JSONL (JSON Lines) format
- Includes timestamps (chrono)
