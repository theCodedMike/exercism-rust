pub fn translate(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut s = "".to_string();
            let mut target_idx = 0;
            let mut has_qu = false;
            for (idx, chars) in word.chars().collect::<Vec<_>>().windows(2).enumerate() {
                match (chars[0], chars[1]) {
                    ('a', _)
                    | ('e', _)
                    | ('i', _)
                    | ('o', _)
                    | ('u', _)
                    | ('x', 'r')
                    | ('y', 't') => {
                        if chars[0] == 'u' && has_qu {
                            continue;
                        }
                        target_idx = idx;
                        break;
                    }
                    ('q', 'u') => {
                        has_qu = true;
                    }
                    (_, _) => {}
                }
            }

            if target_idx == 0 {
                s.push_str(word);
            } else {
                s.push_str(&word[target_idx..]);
                s.push_str(&word[..target_idx]);
            }
            s.push_str("ay");

            s
        })
        .collect::<Vec<_>>()
        .join(" ")
}
