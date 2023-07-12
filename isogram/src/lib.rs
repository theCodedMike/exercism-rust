use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map = HashMap::new();
    for c in candidate.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                let key = c.to_ascii_lowercase();
                map.entry(key).and_modify(|v| *v += 1).or_insert(1);
                if map[&key] > 1 {
                    return false;
                }
            }
            _ => {}
        }
    }

    true
}
