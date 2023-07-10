/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let len1 = s1.len();
    let len2 = s2.len();
    if len1 != len2 {
        return None;
    }

    let mut dist = 0;
    for i in 0..len1 {
        if &s1[i..i + 1] != &s2[i..i + 1] {
            dist += 1;
        }
    }

    Some(dist)
}
