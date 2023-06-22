pub fn square(s: u32) -> u64 {
    if s == 0 || s >= 65 {
        panic!("Square must be between 1 and 64");
    }

    2_u64.pow(s - 1)
}

pub fn total() -> u64 {
    (1..=64_u32).map(|i| square(i)).sum()
}
