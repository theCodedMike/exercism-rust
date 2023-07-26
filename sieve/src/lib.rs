pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound)
        .filter(|i| is_prime(*i))
        .collect::<Vec<_>>()
}

fn is_prime(num: u64) -> bool {
    (2..num).all(|i| num % i != 0)
}
