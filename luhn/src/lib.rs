/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let len = code.trim().len();
    if len <= 1 {
        return false;
    }

    let mut digit_count = 0;
    let mut sum = 0;
    for c in code.chars().rev() {
        if c != ' ' {
            digit_count += 1;
            match convert_to_digit(c) {
                None => return false,
                Some(val) => {
                    if digit_count % 2 == 0 {
                        // 偶数位
                        let doubled = val * 2;
                        if doubled > 9 {
                            sum += doubled - 9;
                        } else {
                            sum += doubled
                        }
                    } else {
                        // 奇数位
                        sum += val;
                    }
                }
            }
        }
    }

    sum % 10 == 0
}

fn convert_to_digit(c: char) -> Option<i32> {
    match c {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        _ => None,
    }
}
