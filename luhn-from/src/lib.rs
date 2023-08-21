use std::fmt::Display;

pub struct Luhn {
    data: Result<Vec<u32>, bool>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        match self.data {
            Ok(ref data) => {
                data.iter()
                    .rev()
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
                    .sum::<u32>()
                    % 10
                    == 0
            }
            Err(_) => false,
        }
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T> From<T> for Luhn
where
    T: Display,
{
    fn from(input: T) -> Self {
        let ori = input.to_string();
        let parse_result = ori
            .chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(|c| c.to_digit(10).ok_or(false))
            .collect::<Result<Vec<_>, bool>>();

        Luhn {
            data: if parse_result.is_ok() && ori.len() > 1 {
                parse_result
            } else {
                Err(false)
            },
        }
    }
}
