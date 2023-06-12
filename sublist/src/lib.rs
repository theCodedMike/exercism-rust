#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (_, 0) => Comparison::Superlist,
        (0, _) => Comparison::Sublist,
        (first_len, second_len) if first_len < second_len => {
            if second_list.windows(first_len).any(|w| w.eq(first_list)) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        (first_len, second_len) if first_len > second_len => {
            if first_list.windows(second_len).any(|w| w.eq(second_list)) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
        _ => {
            if first_list.eq(second_list) {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
    }
}
