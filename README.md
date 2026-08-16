# Kaomoji Picker

A standalone kaomoji picker built with **Rust** and **Qt6** via [cxx-qt](https://github.com/KDAB/cxx-qt).

![Kaomoji Picker](https://img.shields.io/badge/Rust-cxx--qt-orange) ![Qt6](https://img.shields.io/badge/Qt6-QML-green) ![Platform](https://img.shields.io/badge/platform-Linux-blue)

## Features

- **~180 built-in kaomoji** across 11 categories (Happy, Sad, Angry, Confused, Love, Animals, People, Hands, Tables, Misc)
- **Search** — fuzzy filter by kaomoji characters or description
- **Category browsing** — click category chips to filter the grid
- **Click to copy** — click any kaomoji to copy it to the system clipboard
- **Custom kaomoji** — add your own via the built-in editor panel
- **Import/Export** — share custom kaomoji collections as JSON files
- **Persistent storage** — custom kaomoji saved to `~/.local/share/kaomoji-picker/custom.json`
- **Always-on-top window** — quick access, similar to KDE Plasma's emoji picker (Super + .)

## Screenshots

```
┌─────────────────────────────────┐
│ 🔍 Search kaomoji...            │
├─────────────────────────────────┤
│ All │ Happy │ Sad │ Angry │ ... │
├─────────────────────────────────┤
│ (◕‿◕)  (╯°□°)╯  ┻━┻  ʕ•ᴥ•ʔ    │
│ (ﾉ◕ヮ◕)  (╥_╥)  ( ˘ω˘ )  ¯\_(ツ)_/¯ │
│   ...                           │
├─────────────────────────────────┤
│ 142 kaomoji    +Add  Import Export│
└─────────────────────────────────┘
```

## Prerequisites

### Rust

Install via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Qt6

**Arch Linux / CachyOS:**

```bash
sudo pacman -S qt6-base qt6-declarative qt6-tools
```

**Ubuntu / Debian:**

```bash
sudo apt install qt6-base-dev qt6-declarative-dev qt6-tools-dev
```

**Fedora:**

```bash
sudo dnf install qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qttools-devel
```

### Build tools

A C++ compiler (`gcc` or `clang`) and `pkg-config` are also required.

## Building

```bash
git clone https://github.com/Alekisan/kaomoji-picker.git
cd kaomoji-picker
cargo build --release
```

## Running

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/kaomoji-picker
```

## Keyboard Shortcut Setup (KDE Plasma)

To launch the picker with a keyboard shortcut (e.g. `Super + K`):

1. Open **System Settings** → **Shortcuts**
2. Click **Add New** → **Command**
3. Set the command to the full path of the binary:
   ```
   /home/<user>/Projects/kaomoji-picker/target/release/kaomoji-picker
   ```
4. Assign a shortcut key (e.g. `Meta + K`)
5. Click **Apply**

## Project Structure

```
kaomoji-picker/
├── Cargo.toml              # Dependencies: cxx-qt, cxx-qt-lib, dirs
├── build.rs                # cxx-qt build configuration & QML module registration
├── cpp/
│   ├── clipboard.h         # C++ header for Qt clipboard helper
│   └── clipboard.cpp       # QGuiApplication::clipboard()->setText() wrapper
├── src/
│   ├── main.rs             # Application entry point
│   └── kaomoji/
│       ├── mod.rs          # KaomojiEntry struct definition
│       ├── model.rs        # cxx-qt bridge: KaomojiController QObject
│       ├── builtin.rs      # ~180 built-in kaomoji entries
│       └── storage.rs      # JSON load/save for custom kaomoji
└── qml/
    ├── main.qml            # Main ApplicationWindow (search, categories, grid)
    ├── KaomojiGrid.qml     # GridView container
    ├── KaomojiCard.qml     # Individual clickable kaomoji card
    ├── CategoryBar.qml     # Horizontal category filter chips
    ├── EditorPanel.qml     # Add custom kaomoji form
    └── Toast.qml           # "Copied!" notification label
```

## How It Works

### Rust Side

- **`KaomojiController`** — a cxx-qt `#[qobject]` exposed to QML with:
  - `filtered_count` property (notifies QML when the filter results change)
  - Invokable methods: `loadAll()`, `setSearchText()`, `setCategory()`, `getChars()`, `getDescription()`, `getCategories()`, `addKaomoji()`, `removeKaomoji()`, `importJson()`, `exportJson()`, `copyToClipboard()`
  - Internal state stored in `RefCell<ControllerState>` for interior mutability through `Pin<&mut Self>`

### QML Side

- QML calls `controller.loadAll()` on startup, then `root.refresh()` to populate a `ListModel`
- The `GridView` delegate renders each kaomoji as a `KaomojiCard`
- Search and category changes call `controller.setSearchText()` / `controller.setCategory()`, then `root.refresh()` rebuilds the list

### Clipboard

Uses Qt's native `QClipboard` API via a small C++ helper (`cpp/clipboard.cpp`), ensuring compatibility with both Wayland and X11 without external dependencies.

## Custom Kaomoji JSON Format

```json
[
  {
    "chars": "(◕‿◕)",
    "description": "happy face",
    "categories": ["happy", "misc"]
  },
  {
    "chars": "¯\\_(ツ)_/¯",
    "description": "shrug",
    "categories": ["misc"]
  }
]
```

## Tech Stack

| Component | Technology |
|-----------|-----------|
| Language | Rust 1.77+ |
| GUI Framework | Qt6 (QML / Qt Quick) |
| Rust↔Qt Bridge | [cxx-qt 0.9](https://github.com/KDAB/cxx-qt) |
| Storage | JSON file (`~/.local/share/kaomoji-picker/custom.json`) |
| Clipboard | Qt6 QClipboard (via C++ helper) |

## License

This project is open source. Feel free to use, modify, and distribute.
