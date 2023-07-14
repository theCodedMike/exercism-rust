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

    let mut start = min * min;
    let mut end = max * max;
    let mut smallest_palindrome_product = 0;
    let mut largest_palindrome_product = 0;
    let mut find_smalest_palindrome = false;
    let mut find_largest_palindrome = false;
    while start <= end {
        process_one_slide(
            &mut find_smalest_palindrome,
            &mut start,
            &mut smallest_palindrome_product,
            true,
            min,
            max,
        );

        process_one_slide(
            &mut find_largest_palindrome,
            &mut end,
            &mut largest_palindrome_product,
            false,
            min,
            max,
        );

        if find_smalest_palindrome && find_largest_palindrome {
            break;
        }
    }

    if find_smalest_palindrome && find_largest_palindrome {
        Some((
            Palindrome(smallest_palindrome_product),
            Palindrome(largest_palindrome_product),
        ))
    } else {
        None
    }
}

fn process_one_slide(
    find_palindrome: &mut bool,
    value: &mut u64,
    palindrome_product: &mut u64,
    is_start: bool,
    min: u64,
    max: u64,
) {
    if !(*find_palindrome) {
        if is_palindrome(*value) && is_divide_exactly(*value, min, max) {
            *find_palindrome = true;
            *palindrome_product = *value;
        } else {
            if is_start {
                *value += 1;
            } else {
                *value -= 1;
            }
        }
    }
}

fn is_palindrome(mut value: u64) -> bool {
    let len = value.ilog10() + 1;
    let is_len_odd = len % 2 == 1;

    let mut right_half = 0;
    for _ in 0..(len / 2) {
        right_half = right_half * 10 + value % 10;
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
