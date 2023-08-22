use std::fmt::Display;

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: Display> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let parse_result = self
            .to_string()
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>();
        match parse_result {
            None => false,
            Some(digits) => {
                let sum = digits
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(idx, &val)| {
                        if idx % 2 == 1 {
                            let mut tmp = val * 2;
                            if tmp > 9 {
                                tmp -= 9;
                            }
                            tmp
                        } else {
                            val
                        }
                    })
                    .sum::<u32>();

                digits.len() > 1 && sum % 10 == 0
            }
        }
    }
}
