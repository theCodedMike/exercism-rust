use std::ops::Index;

pub fn series(digits: &str, len: usize) -> Vec<String> {
    let str_len = digits.len();
    if len == 0 {
        return vec!["".to_string(); str_len + 1];
    }
    if len > str_len {
        return vec![];
    }

    let mut res = vec![];
    for end in len..=str_len {
        let begin = end - len;
        res.push(digits.index(begin..end).to_string());
    }

    res
}
