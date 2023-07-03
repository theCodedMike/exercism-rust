#![allow(unused_assignments)]

pub fn abbreviate(phrase: &str) -> String {
    let mut res = "".to_string();
    let contains_colon = phrase.contains(":");
    let mut prev_uppercase = false;
    let mut cur_uppercase = false;
    let mut whitespace = false;

    for c in phrase.chars() {
        match c {
            _ if c.is_ascii_uppercase() => {
                whitespace = false;
                prev_uppercase = cur_uppercase;
                cur_uppercase = true;

                if contains_colon {
                    res.push(c);
                } else {
                    if !prev_uppercase && cur_uppercase {
                        res.push(c);
                    }
                }
            }
            ' ' | '-' => {
                whitespace = true;
                prev_uppercase = cur_uppercase;
                cur_uppercase = false;
            }
            _ if c.is_ascii_lowercase() => {
                if whitespace {
                    res.push_str(c.to_uppercase().to_string().as_str());
                }
                whitespace = false;
                prev_uppercase = cur_uppercase;
                cur_uppercase = false;
            }
            ':' => break,
            _ => {}
        }
    }

    res
}
