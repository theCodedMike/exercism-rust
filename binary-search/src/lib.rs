/// cargo test -- --ignored

pub fn find<ARRAY: AsRef<[T]>, T: Ord>(array: ARRAY, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();
    let mut mid;

    while left < right {
        mid = (left + right) / 2;
        if key < array[mid] {
            right -= 1;
        } else if key > array[mid] {
            left += 1;
        } else {
            return Some(mid);
        }
    }

    None
}
