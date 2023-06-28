pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0;
    }

    (1..limit)
        .filter(|&x| factors.iter().filter(|&&f| f != 0).any(|&f| x % f == 0))
        .sum()
}
