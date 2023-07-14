/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if is_palindrome(value) {
            Some(Palindrome(value))
        } else {
            None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let smallest_product = min * min;
    let largest_product = max * max;

    let mut smallest_palindrome = 0;
    for v in smallest_product..=largest_product {
        if is_palindrome(v) && is_divide_exactly(v, min, max) {
            smallest_palindrome = v;
            break;
        }
    }

    let mut largest_palindrome = 0;
    for v in (smallest_product..=largest_product).rev() {
        if is_palindrome(v) && is_divide_exactly(v, min, max) {
            largest_palindrome = v;
            break;
        }
    }

    if smallest_palindrome != 0 && largest_palindrome != 0 {
        Some((
            Palindrome(smallest_palindrome),
            Palindrome(largest_palindrome),
        ))
    } else {
        None
    }
}

fn is_palindrome(mut value: u64) -> bool {
    let len = value.ilog10() + 1;
    let is_len_odd = len % 2 == 1;

    let mut right_half = 0;
    for _ in 0..(len / 2) {
        let rem = value % 10;
        right_half = right_half * 10 + rem;
        value /= 10;
    }

    if is_len_odd {
        value /= 10;
    }

    value == right_half
}

fn is_divide_exactly(value: u64, min: u64, max: u64) -> bool {
    (min..=max).any(|i| {
        let rem = value % i; // 余数
        let quo = value / i; // 商
        rem == 0 && (min <= quo) && (quo <= max)
    })
}
