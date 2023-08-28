use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum RotateDirection {
    Left,
    None,
    Right,
}

pub fn rotate(input: &str, key: i8) -> String {
    let (direction, dist) = match key % 26 {
        lt_0 if lt_0 < 0 => (RotateDirection::Right, lt_0 + 26),
        gt_0 if gt_0 > 0 => (RotateDirection::Left, gt_0),
        _ => (RotateDirection::None, 0),
    };
    if direction == RotateDirection::None {
        input.to_string()
    } else {
        let mut a_to_z = ('a'..='z').collect::<Vec<_>>();
        a_to_z.rotate_left(dist as usize);
        let map = ('a'..='z')
            .zip(a_to_z.into_iter())
            .collect::<HashMap<char, char>>();

        input
            .chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let (lower, is_upper) = if c.is_ascii_uppercase() {
                        (c.to_ascii_lowercase(), true)
                    } else {
                        (c, false)
                    };

                    let mut res = map[&lower];
                    if is_upper {
                        res = res.to_ascii_uppercase();
                    }
                    res
                } else {
                    c
                }
            })
            .collect()
    }
}
