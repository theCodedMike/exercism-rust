pub fn encode(source: &str) -> String {
    let mut res = "".to_string();
    let mut chars = source.chars().peekable();

    let mut repeat = 0;
    while let Some(curr) = chars.next() {
        repeat += 1;
        if chars.peek() != Some(&curr) {
            if repeat > 1 {
                res.push_str(&repeat.to_string());
            }
            res.push(curr);
            repeat = 0;
        }
    }

    res
}

pub fn decode(source: &str) -> String {
    let mut res = "".to_string();

    let mut repeat = String::new();
    for c in source.chars() {
        if c.is_numeric() {
            repeat.push(c);
        } else {
            res.push_str(&c.to_string().repeat(repeat.parse::<usize>().unwrap_or(1)));
            repeat.clear();
        }
    }

    res
}
