use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut counter = ('a'..='z').map(|c| (c, 0)).collect::<HashMap<char, i32>>();

    for c in sentence.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let mut c_clone = c;
                if c_clone.is_ascii_uppercase() {
                    c_clone = c_clone.to_ascii_lowercase();
                }
                counter.entry(c_clone).and_modify(|v| *v += 1).or_insert(0);
            }
            _ => {}
        }
    }

    counter.values().all(|v| *v >= 1)
}
