///    2 3 5 7 11 13
/// n: 0 1 2 3 4  5
pub fn nth(n: u32) -> u32 {
    let mut count = 0;
    let mut nth_prime = 0;

    for u in 2..=u32::MAX {
        if is_prime(u) {
            count += 1;

            if count == n + 1 {
                nth_prime = u;
                break;
            }
        }
    }

    nth_prime
}

fn is_prime(num: u32) -> bool {
    let mut is_prime = true;

    let half = num / 2;
    for i in 2..=half {
        if num % i == 0 {
            is_prime = false;
            break;
        }
    }

    is_prime
}
