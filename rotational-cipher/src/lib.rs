pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => rot(c as u8, b'a', key),
            'A'..='Z' => rot(c as u8, b'A', key),
            _ => c,
        })
        .collect()
}

fn rot(c: u8, lower_bound: u8, key: i8) -> char {
    let new = (c - lower_bound) as i8 + key;
    if new < 0 {
        ((new + 26) as u8 + lower_bound) as char
    } else {
        (new as u8 % 26 + lower_bound) as char
    }
}
