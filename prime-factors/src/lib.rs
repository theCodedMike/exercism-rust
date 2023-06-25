pub fn factors(mut n: u64) -> Vec<u64> {
    let mut res = vec![];
    if n <= 1 {
        return res;
    }

    let mut prime_factor = 2;

    while prime_factor != n {
        if n % prime_factor == 0 {
            res.push(prime_factor);
            n /= prime_factor;
        } else {
            prime_factor += 1;
        }
    }
    if prime_factor == n {
        res.push(prime_factor);
    }

    res
}
