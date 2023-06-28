use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() {
        return 0;
    }

    let mut res = HashSet::new();
    for i in 1..u32::MAX {
        let mut all_greater_than_limit = true;

        for &f in factors {
            if f == 0 {
                continue;
            }
            let product = i * f;
            if product < limit {
                all_greater_than_limit = false;
                res.insert(product);
            }
        }

        if all_greater_than_limit {
            break;
        }
    }

    res.iter().sum()
}
