/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    let mut distance = 0;

    let mut s1 = s1.chars();
    let mut s2 = s2.chars();
    loop {
        match (s1.next(), s2.next()) {
            (Some(c1), Some(c2)) if c1 != c2 => distance += 1,
            (Some(_), None) | (None, Some(_)) => return None,
            (None, None) => break,
            _ => {}
        }
    }

    Some(distance)
}
