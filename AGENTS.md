# Kaomoji Picker — Project Memory

## Project Overview
A standalone kaomoji picker built with **Rust + cxx-qt 0.9 + Qt6 (QML)**.
- Repo: https://github.com/Alekisan/kaomoji-picker
- Location: `/home/alex/Projects/kaomoji-picker`
- User GitHub: Alekisan
- GitHub PAT stored at: `/home/alex/Documents/github_pat.txt`
- OS: CachyOS (Arch-based), KDE Plasma, Wayland
- Qt version: 6.11.1
- Rust: 1.95.0

## Build & Run
```bash
cd ~/Projects/kaomoji-picker
cargo clean && cargo build --release
cargo run --release
```
- `cargo clean` is important when QML files change — cxx-qt caches QRC resources and stale artifacts can cause the QML to fail silently (no window appears, process runs in background)
- C++ warnings from Qt headers (SFINAE incomplete type) are harmless

## Architecture

### Rust Side
- `src/main.rs` — entry point, creates `QGuiApplication` + `QQmlApplicationEngine`, loads `qrc:/qt/qml/com/kaomoji/picker/qml/main.qml`
- `src/kaomoji/mod.rs` — `KaomojiEntry` struct (chars, description, categories)
- `src/kaomoji/model.rs` — cxx-qt bridge defining `KaomojiController` QObject:
  - `#[qproperty(i32, filtered_count)]` — QML property is snake_case `filtered_count` (NOT camelCase `filteredCount`)
  - Internal state in `RefCell<ControllerState>` for interior mutability through `Pin<&mut Self>` (cxx-qt doesn't allow direct field mutation through Pin)
  - Read-only methods use `self: &Self`, mutable methods use `self: Pin<&mut Self>`
  - Invokable methods: loadAll, setSearchText, setCategory, getChars, getDescription, getCategories, addKaomoji, removeKaomoji, importJson, exportJson, copyToClipboard
- `src/kaomoji/builtin.rs` — ~180 hardcoded kaomoji across 11 categories
- `src/kaomoji/storage.rs` — custom JSON load/save (no serde, manual parser). Saved to `~/.local/share/kaomoji-picker/custom.json`

### C++ Side
- `cpp/clipboard.cpp` + `cpp/clipboard.h` — C++ helper calling `QGuiApplication::clipboard()->setText()` (cxx-qt-lib doesn't expose QClipboard)
- Declared in the cxx bridge via `include!("clipboard.h")` and `.include_dir("cpp")` in build.rs

### QML Side
- `qml/main.qml` — ApplicationWindow with search, CategoryBar, buttons, StackLayout (GridView + EditorPanel), Toast, FileDialogs
- `qml/KaomojiGrid.qml` — GridView container (currently unused — grid is inline in main.qml)
- `qml/KaomojiCard.qml` — clickable card, `signal clicked(string kaomoji)`
- `qml/CategoryBar.qml` — category filter chips
- `qml/EditorPanel.qml` — add custom kaomoji form
- `qml/Toast.qml` — "Copied!" notification (bare Label, no custom properties — don't redeclare `text`)

### build.rs
- Uses `CxxQtBuilder::new_qml_module()` with QmlModule URI `com.kaomoji.picker`
- `.file("src/kaomoji/model.rs")` — cxx-qt bridge file
- `.cpp_file("cpp/clipboard.cpp")` — C++ clipboard helper
- `.include_dir("cpp")` — so `include!("clipboard.h")` works in the bridge

## Key Lessons Learned
1. **cxx-qt property names are snake_case** — `filtered_count` in QML, not `filteredCount`
2. **ListModel delegates use `model.roleName`**, not `modelData.roleName`
3. **Qt6 FileDialog** uses `fileMode: FileDialog.SaveFile`, not `selectExisting: false` (that was Qt5)
4. **`Pin<&mut Self>` field mutation** — cxx-qt doesn't support direct field assignment through Pin; use `RefCell` for interior mutability
5. **`Pin<&Self>` is invalid** — use `&Self` for read-only invokable methods
6. **`cargo clean` is essential** when QML files change to avoid stale QRC resources
7. **QML errors are silent** — use `QT_FORCE_STDERR_LOGGING=1` to see them
8. **Clipboard** — `wl-copy`/`xclip` may not be installed; use Qt's QClipboard via C++ helper instead

## Potential Future Improvements
- Use `QAbstractListModel` instead of ListModel + JS refresh loop for better performance
- Add keyboard navigation (arrow keys, Enter to copy, Escape to close)
- Single-instance via DBus (so shortcut launches raise existing window)
- System tray icon
- More kaomoji entries
- Dark/light theme awareness
- Favorite/recent kaomoji section
- Configurable window position/size persistence
