/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let result = code
        .chars()
        .rev()
        .filter(|c| *c != ' ')
        .map(|c| c.to_digit(10).ok_or(false))
        .collect::<Result<Vec<_>, bool>>();

    return match result {
        Ok(digits) => {
            let sum = digits
                .iter()
                .enumerate()
                .map(|(idx, &v)| {
                    if idx % 2 == 1 {
                        let mut tmp = v * 2;
                        if tmp > 9 {
                            tmp -= 9;
                        }
                        tmp
                    } else {
                        v
                    }
                })
                .sum::<u32>();
            digits.len() > 1 && sum % 10 == 0
        }
        Err(_) => false,
    };
}
