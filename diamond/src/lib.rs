const A: u8 = b'A';

pub fn get_diamond(mut c: char) -> Vec<String> {
    if !c.is_ascii_alphabetic() {
        panic!("c must be 'A'..='Z' or 'a'..='z'");
    }
    if c.is_ascii_lowercase() {
        c = c.to_ascii_uppercase();
    }

    let cur_idx = c as u8 - A;
    let len = cur_idx * 2 + 1;

    (0..len)
        .map(|idx| {
            if idx <= cur_idx {
                (A + idx) as char
            } else {
                (A + len - 1 - idx) as char
            }
        })
        .enumerate()
        .map(|(idx, c)| {
            (0..len)
                .map(|i| {
                    let pos = horizontal_position(idx as u8, len);
                    if i == pos.0 || i == pos.1 {
                        c
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        })
        .collect()
}

fn horizontal_position(row_idx: u8, len: u8) -> (u8, u8) {
    let half_idx = len / 2;
    let first_idx;
    if row_idx <= half_idx {
        first_idx = half_idx - row_idx;
    } else {
        first_idx = row_idx - half_idx;
    }
    (first_idx, len - 1 - first_idx)
}
