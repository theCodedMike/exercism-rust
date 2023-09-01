use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    let half = sum / 2;

    for i in 1..=half {
        for j in i..=half {
            let k = sum - i - j;
            if k < j {
                continue;
            }
            if i * i + j * j == k * k {
                set.insert([i, j, k]);
            }
        }
    }

    set
}
