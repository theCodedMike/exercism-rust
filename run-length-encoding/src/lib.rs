pub fn encode(source: &str) -> String {
    let mut res = "".to_string();
    let mut prev = '0';
    let mut repeat = 0;

    for (idx, cur) in source.chars().enumerate() {
        if idx == 0 {
            prev = cur;
            repeat = 1;
        } else {
            if prev == cur {
                repeat += 1;
            } else {
                process_prev(&mut res, prev, repeat);

                prev = cur;
                repeat = 1;
            }
        }
    }
    if repeat != 0 {
        process_prev(&mut res, prev, repeat);
    }

    res
}

fn process_prev(res: &mut String, prev: char, repeat: i32) {
    if repeat > 1 {
        res.push_str(&repeat.to_string());
    }
    res.push(prev);
}

pub fn decode(source: &str) -> String {
    let mut res = "".to_string();

    let mut repeat = 0;
    for c in source.chars() {
        match c {
            '0'..='9' => {
                let digit = (c as u8 - b'0') as usize;
                repeat = repeat * 10 + digit;
            }
            _ => {
                if repeat > 1 {
                    res.push_str(&c.to_string().repeat(repeat));
                    repeat = 0;
                } else {
                    res.push(c);
                }
            }
        }
    }

    res
}
