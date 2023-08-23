pub fn number(user_number: &str) -> Option<String> {
    let res = user_number
        .bytes()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();

    match res.len() {
        7 | 10 | 11 => {
            let determine_str_is_valid = |len: usize, bytes: Vec<u8>| -> Option<String> {
                let (n1_idx, n2_idx, begin) = if len == 7 {
                    (0, 0, 0)
                } else if len == 10 {
                    (0, 3, 0)
                } else {
                    // len == 11
                    if bytes[0] != b'1' {
                        return None;
                    }
                    (1, 4, 1)
                };

                if bytes[n1_idx] < b'2' || bytes[n2_idx] < b'2' {
                    None
                } else {
                    Some(String::from_utf8_lossy(&bytes[begin..]).to_string())
                }
            };

            determine_str_is_valid(res.len(), res)
        }
        _ => None,
    }
}
