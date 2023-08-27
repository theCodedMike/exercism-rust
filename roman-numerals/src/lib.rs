use std::fmt::{Display, Formatter, Result};

const NUMERALS: [(usize, [&'static str; 10]); 4] = [
    (
        1000,
        ["", "M", "MM", "MMM", "--", "-", "--", "---", "----", "--"],
    ),
    (
        100,
        ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"],
    ),
    (
        10,
        ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"],
    ),
    (
        1,
        ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"],
    ),
];

pub struct Roman(usize);

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let val = self.0;
        let res = if val > 3999 {
            "".to_string()
        } else {
            NUMERALS
                .iter()
                .map(|&(base, nums)| nums[(val / base) % 10])
                .collect()
        };

        write!(f, "{}", res)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        Roman(num as usize)
    }
}
