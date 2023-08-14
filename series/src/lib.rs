pub fn series(digits: &str, len: usize) -> Vec<String> {
    let str_len = digits.len();
    if len == 0 {
        return vec!["".to_string(); str_len + 1];
    }
    if len > str_len {
        return vec![];
    }

    digits
        .as_bytes()
        .windows(len)
        .map(|w| String::from_utf8_lossy(w).to_string())
        .collect::<Vec<_>>()
}
