use std::fs;
use std::path::PathBuf;

use crate::kaomoji::KaomojiEntry;

fn data_dir() -> PathBuf {
    let base = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    base.join("kaomoji-picker")
}

fn custom_file() -> PathBuf {
    data_dir().join("custom.json")
}

pub fn load_custom() -> Vec<KaomojiEntry> {
    let path = custom_file();
    if !path.exists() {
        return Vec::new();
    }
    let content = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };
    parse_json_entries(&content)
}

pub fn save_custom(entries: &[KaomojiEntry]) {
    let dir = data_dir();
    let _ = fs::create_dir_all(&dir);
    let json = entries_to_json_string(entries);
    let _ = fs::write(custom_file(), json);
}

pub fn parse_json_entries(json: &str) -> Vec<KaomojiEntry> {
    let mut entries = Vec::new();
    let trimmed = json.trim();
    if trimmed.is_empty() || trimmed == "[]" {
        return entries;
    }

    // Simple JSON parsing without serde
    // Expected format: [{"chars":"...","description":"...","categories":["..."]}, ...]
    let mut i = 0;
    let bytes = trimmed.as_bytes();

    // Find the opening bracket
    while i < bytes.len() && bytes[i] != b'[' {
        i += 1;
    }
    if i >= bytes.len() {
        return entries;
    }
    i += 1; // skip [

    while i < bytes.len() {
        // Skip whitespace
        while i < bytes.len() && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= bytes.len() || bytes[i] == b']' {
            break;
        }
        if bytes[i] != b'{' {
            break;
        }
        i += 1; // skip {

        let mut chars = String::new();
        let mut description = String::new();
        let mut categories = Vec::new();

        while i < bytes.len() && bytes[i] != b'}' {
            // Skip whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }
            if i >= bytes.len() || bytes[i] == b'}' {
                break;
            }

            // Read key
            let key = read_json_string(bytes, &mut i);
            // Skip :
            while i < bytes.len() && bytes[i] != b':' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            // Skip whitespace
            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                i += 1;
            }

            match key.as_str() {
                "chars" => chars = read_json_string(bytes, &mut i),
                "description" => description = read_json_string(bytes, &mut i),
                "categories" => {
                    // Read array of strings
                    if i < bytes.len() && bytes[i] == b'[' {
                        i += 1;
                        while i < bytes.len() && bytes[i] != b']' {
                            while i < bytes.len() && bytes[i].is_ascii_whitespace() {
                                i += 1;
                            }
                            if i < bytes.len() && bytes[i] == b'"' {
                                categories.push(read_json_string(bytes, &mut i));
                            }
                            while i < bytes.len() && bytes[i] != b',' && bytes[i] != b']' {
                                i += 1;
                            }
                            if i < bytes.len() && bytes[i] == b',' {
                                i += 1;
                            }
                        }
                        if i < bytes.len() {
                            i += 1; // skip ]
                        }
                    }
                }
                _ => {}
            }

            // Skip comma between fields
            while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
                i += 1;
            }
        }

        if i < bytes.len() {
            i += 1; // skip }
        }

        if !chars.is_empty() {
            entries.push(KaomojiEntry {
                chars,
                description,
                categories,
            });
        }

        // Skip comma between objects
        while i < bytes.len() && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
    }

    entries
}

fn read_json_string(bytes: &[u8], i: &mut usize) -> String {
    while *i < bytes.len() && bytes[*i] != b'"' {
        *i += 1;
    }
    if *i >= bytes.len() {
        return String::new();
    }
    *i += 1; // skip opening quote
    let start = *i;
    while *i < bytes.len() && bytes[*i] != b'"' {
        if bytes[*i] == b'\\' {
            *i += 2; // skip escaped char
        } else {
            *i += 1;
        }
    }
    let end = *i;
    if *i < bytes.len() {
        *i += 1; // skip closing quote
    }
    String::from_utf8_lossy(&bytes[start..end]).to_string()
}

pub fn entries_to_json_string(entries: &[KaomojiEntry]) -> String {
    let mut json = String::from("[\n");
    for (idx, entry) in entries.iter().enumerate() {
        json.push_str("  {\n");
        json.push_str(&format!("    \"chars\": {},\n", json_escape(&entry.chars)));
        json.push_str(&format!(
            "    \"description\": {}",
            json_escape(&entry.description)
        ));
        if !entry.categories.is_empty() {
            json.push_str(",\n    \"categories\": [");
            for (ci, cat) in entry.categories.iter().enumerate() {
                if ci > 0 {
                    json.push(',');
                }
                json.push_str(&format!("\n      {}", json_escape(cat)));
            }
            json.push_str("\n    ]");
        }
        json.push_str("\n  }");
        if idx < entries.len() - 1 {
            json.push(',');
        }
        json.push('\n');
    }
    json.push_str("]\n");
    json
}

fn json_escape(s: &str) -> String {
    let mut result = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            _ => result.push(c),
        }
    }
    result.push('"');
    result
}
