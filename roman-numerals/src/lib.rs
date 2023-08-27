use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::sync::OnceLock;

static INIT: OnceLock<HashMap<usize, &str>> = OnceLock::new();

pub struct Roman(usize);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let roman_map = INIT.get_or_init(|| {
            HashMap::from([
                (1, "I"),
                (5, "V"),
                (10, "X"),
                (50, "L"),
                (100, "C"),
                (500, "D"),
                (1000, "M"),
            ])
        });

        let mut res = "".to_string();
        let mut val = self.0;
        let mut base = 1;
        while val != 0 {
            let rem = val % 10;
            if rem != 0 {
                let curr_val = rem * base;
                let sub_res = if roman_map.contains_key(&curr_val) {
                    roman_map[&curr_val].to_string()
                } else {
                    match rem {
                        2..=3 => roman_map[&base].repeat(rem),
                        4 => roman_map[&base].to_string() + roman_map[&(5 * base)],
                        6..=8 => {
                            roman_map[&(5 * base)].to_string() + &roman_map[&base].repeat(rem - 5)
                        }
                        9 => roman_map[&base].to_string() + roman_map[&(base * 10)],
                        _ => panic!("unsupported rem: {}", rem),
                    }
                };
                res.insert_str(0, &sub_res);
            }

            base *= 10;
            val /= 10;
        }

        write!(f, "{}", res)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num as usize)
    }
}
