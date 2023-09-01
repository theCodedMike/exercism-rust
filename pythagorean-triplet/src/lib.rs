use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();

    for a in 1..=sum / 3 {
        let sum_b_c = sum - a;
        let denominator = 2 * sum_b_c;
        let numerator = sum_b_c * sum_b_c - a * a;
        if numerator % denominator == 0 {
            let b = numerator / denominator;
            if a < b {
                set.insert([a, b, sum - a - b]);
            }
        }
    }

    set
}
