use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();

    for mut word in words.split(&[' ', ',', '\n', ':', '!', '&', '@', '$', '%', '^', '.']) {
        if word.is_empty() {
            continue;
        }
        if word.starts_with("'") {
            word = word.strip_prefix("'").unwrap();
        }
        if word.ends_with("'") {
            word = word.strip_suffix("'").unwrap();
        }

        map.entry(word.to_ascii_lowercase())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    map
}
