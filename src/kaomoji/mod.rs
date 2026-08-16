pub mod builtin;
pub mod model;
pub mod storage;

#[derive(Clone)]
pub struct KaomojiEntry {
    pub chars: String,
    pub description: String,
    pub categories: Vec<String>,
}
