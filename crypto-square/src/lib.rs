pub fn encrypt(input: &str) -> String {
    let mut res = "".to_string();

    let normalized = normalized(input);
    let len = normalized.len();
    if len == 0 {
        return res;
    }

    let (row, col) = compute_row_and_col(len);

    convert(&normalized, row, col, &mut res);

    res
}

fn normalized(ori: &str) -> String {
    ori.chars()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                if c.is_ascii_uppercase() {
                    Some(c.to_ascii_lowercase())
                } else {
                    Some(c)
                }
            } else {
                None
            }
        })
        .collect::<String>()
}

fn compute_row_and_col(len: usize) -> (usize, usize) {
    let mut row = 1;
    let mut row_equal_col = false;
    loop {
        if row * row >= len {
            row_equal_col = true;
            break;
        }
        if row * (row + 1) >= len {
            break;
        }

        row += 1;
    }

    if row_equal_col {
        (row, row)
    } else {
        (row, row + 1)
    }
}

fn convert(ori: &str, row: usize, col: usize, res: &mut String) {
    for i in 0..col {
        let mut idx = i;
        for _ in 0..row {
            match ori.get(idx..idx + 1) {
                None => {
                    res.push(' ');
                }
                Some(c) => {
                    res.push_str(c);
                }
            }
            idx += col;
        }
        if i != col - 1 {
            res.push(' ');
        }
    }
}
