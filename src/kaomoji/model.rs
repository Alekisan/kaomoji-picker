use cxx_qt_lib::QString;
use std::cell::RefCell;
use std::pin::Pin;

use super::builtin;
use super::storage;
use super::KaomojiEntry;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("clipboard.h");
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        #[cxx_name = "kaomoji_copy_to_clipboard"]
        fn kaomoji_copy_to_clipboard(text: &QString);
    }

    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(i32, filtered_count)]
        type KaomojiController = super::KaomojiControllerRust;

        #[qinvokable]
        #[cxx_name = "loadAll"]
        fn load_all(self: Pin<&mut Self>);

        #[qinvokable]
        #[cxx_name = "setSearchText"]
        fn set_search_text(self: Pin<&mut Self>, text: &QString);

        #[qinvokable]
        #[cxx_name = "setCategory"]
        fn set_category(self: Pin<&mut Self>, category: &QString);

        #[qinvokable]
        #[cxx_name = "getChars"]
        fn get_chars(self: &Self, index: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "getDescription"]
        fn get_description(self: &Self, index: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "getCategories"]
        fn get_categories(self: &Self, index: i32) -> QString;

        #[qinvokable]
        #[cxx_name = "addKaomoji"]
        fn add_kaomoji(
            self: Pin<&mut Self>,
            chars: &QString,
            desc: &QString,
            categories: &QString,
        );

        #[qinvokable]
        #[cxx_name = "removeKaomoji"]
        fn remove_kaomoji(self: Pin<&mut Self>, index: i32);

        #[qinvokable]
        #[cxx_name = "importJson"]
        fn import_json(self: Pin<&mut Self>, path: &QString);

        #[qinvokable]
        #[cxx_name = "exportJson"]
        fn export_json(self: &Self, path: &QString) -> QString;

        #[qinvokable]
        #[cxx_name = "copyToClipboard"]
        fn copy_to_clipboard(self: &Self, text: &QString);

        #[qinvokable]
        #[cxx_name = "categoryNames"]
        fn category_names(self: &Self) -> QString;
    }
}

struct ControllerState {
    all_entries: Vec<KaomojiEntry>,
    filtered_indices: Vec<usize>,
    search_text: String,
    selected_category: String,
    custom_entries: Vec<KaomojiEntry>,
}

pub struct KaomojiControllerRust {
    filtered_count: i32,
    state: RefCell<ControllerState>,
}

impl Default for KaomojiControllerRust {
    fn default() -> Self {
        Self {
            filtered_count: 0,
            state: RefCell::new(ControllerState {
                all_entries: Vec::new(),
                filtered_indices: Vec::new(),
                search_text: String::new(),
                selected_category: "all".to_string(),
                custom_entries: Vec::new(),
            }),
        }
    }
}

const CATEGORY_NAMES: &[&str] = &[
    "all",
    "happy",
    "sad",
    "angry",
    "confused",
    "love",
    "animals",
    "people",
    "hands",
    "tables",
    "misc",
];

fn recompute_filtered(state: &mut ControllerState) {
    let search_lower = state.search_text.to_lowercase();
    let cat = &state.selected_category;

    state.filtered_indices = state
        .all_entries
        .iter()
        .enumerate()
        .filter(|(_, entry)| {
            let matches_cat =
                cat == "all" || cat.is_empty() || entry.categories.iter().any(|c| c == cat);
            let matches_search = search_lower.is_empty()
                || entry.chars.to_lowercase().contains(&search_lower)
                || entry.description.to_lowercase().contains(&search_lower);
            matches_cat && matches_search
        })
        .map(|(i, _)| i)
        .collect();
}

impl qobject::KaomojiController {
    fn load_all(self: Pin<&mut Self>) {
        let mut state = self.state.borrow_mut();
        let mut entries = builtin::builtin_kaomoji();
        let custom = storage::load_custom();
        entries.extend(custom.clone());
        state.all_entries = entries;
        state.custom_entries = custom;
        recompute_filtered(&mut state);
        let count = state.filtered_indices.len() as i32;
        drop(state);
        self.set_filtered_count(count);
    }

    fn set_search_text(self: Pin<&mut Self>, text: &QString) {
        let mut state = self.state.borrow_mut();
        state.search_text = text.to_string();
        recompute_filtered(&mut state);
        let count = state.filtered_indices.len() as i32;
        drop(state);
        self.set_filtered_count(count);
    }

    fn set_category(self: Pin<&mut Self>, category: &QString) {
        let mut state = self.state.borrow_mut();
        state.selected_category = category.to_string();
        recompute_filtered(&mut state);
        let count = state.filtered_indices.len() as i32;
        drop(state);
        self.set_filtered_count(count);
    }

    fn get_chars(self: &Self, index: i32) -> QString {
        let state = self.state.borrow();
        let idx = index as usize;
        if idx >= state.filtered_indices.len() {
            return QString::default();
        }
        let real = state.filtered_indices[idx];
        QString::from(&state.all_entries[real].chars)
    }

    fn get_description(self: &Self, index: i32) -> QString {
        let state = self.state.borrow();
        let idx = index as usize;
        if idx >= state.filtered_indices.len() {
            return QString::default();
        }
        let real = state.filtered_indices[idx];
        QString::from(&state.all_entries[real].description)
    }

    fn get_categories(self: &Self, index: i32) -> QString {
        let state = self.state.borrow();
        let idx = index as usize;
        if idx >= state.filtered_indices.len() {
            return QString::default();
        }
        let real = state.filtered_indices[idx];
        QString::from(&state.all_entries[real].categories.join(","))
    }

    fn add_kaomoji(
        self: Pin<&mut Self>,
        chars: &QString,
        desc: &QString,
        categories: &QString,
    ) {
        let entry = KaomojiEntry {
            chars: chars.to_string(),
            description: desc.to_string(),
            categories: categories
                .to_string()
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect(),
        };
        let mut state = self.state.borrow_mut();
        state.custom_entries.push(entry.clone());
        state.all_entries.push(entry);
        storage::save_custom(&state.custom_entries);
        recompute_filtered(&mut state);
        let count = state.filtered_indices.len() as i32;
        drop(state);
        self.set_filtered_count(count);
    }

    fn remove_kaomoji(self: Pin<&mut Self>, index: i32) {
        let mut state = self.state.borrow_mut();
        let idx = index as usize;
        if idx >= state.filtered_indices.len() {
            return;
        }
        let real = state.filtered_indices[idx];
        let is_custom = state.custom_entries.iter().any(|e| {
            e.chars == state.all_entries[real].chars
                && e.description == state.all_entries[real].description
        });
        if !is_custom {
            return;
        }
        let chars = state.all_entries[real].chars.clone();
        let desc = state.all_entries[real].description.clone();
        state
            .custom_entries
            .retain(|e| e.chars != chars || e.description != desc);
        state.all_entries.remove(real);
        storage::save_custom(&state.custom_entries);
        recompute_filtered(&mut state);
        let count = state.filtered_indices.len() as i32;
        drop(state);
        self.set_filtered_count(count);
    }

    fn import_json(self: Pin<&mut Self>, path: &QString) {
        let path_str = path.to_string();
        if let Ok(content) = std::fs::read_to_string(&path_str) {
            let entries = storage::parse_json_entries(&content);
            let mut state = self.state.borrow_mut();
            for entry in entries {
                if !state
                    .all_entries
                    .iter()
                    .any(|e| e.chars == entry.chars && e.description == entry.description)
                {
                    state.custom_entries.push(entry.clone());
                    state.all_entries.push(entry);
                }
            }
            storage::save_custom(&state.custom_entries);
            recompute_filtered(&mut state);
            let count = state.filtered_indices.len() as i32;
            drop(state);
            self.set_filtered_count(count);
        }
    }

    fn export_json(self: &Self, path: &QString) -> QString {
        let state = self.state.borrow();
        let path_str = path.to_string();
        let json = storage::entries_to_json_string(&state.custom_entries);
        match std::fs::write(&path_str, &json) {
            Ok(()) => QString::from("ok"),
            Err(e) => QString::from(&format!("error: {e}")),
        }
    }

    fn copy_to_clipboard(self: &Self, text: &QString) {
        qobject::kaomoji_copy_to_clipboard(text);
    }

    fn category_names(self: &Self) -> QString {
        QString::from(&CATEGORY_NAMES.join(","))
    }
}
