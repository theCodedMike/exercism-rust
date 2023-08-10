const U64_MAX: u128 = u64::MAX as u128;

pub fn collatz(n: u64) -> Option<u64> {
    let mut n = n as u128;
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
            if n > U64_MAX {
                return None;
            }
        }
        steps += 1;
    }

    Some(steps)
}
