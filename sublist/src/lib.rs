use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let first_len = first_list.len();
    let second_len = second_list.len();

    let mut res = Comparison::Unequal;

    match first_len.cmp(&second_len) {
        Ordering::Less => {
            if determine_short_contained_within_long(first_list, second_list) {
                res = Comparison::Sublist;
            }
        }
        Ordering::Equal => {
            if determine_short_contained_within_long(first_list, second_list) {
                res = Comparison::Equal;
            }
        }
        Ordering::Greater => {
            if determine_short_contained_within_long(second_list, first_list) {
                res = Comparison::Superlist;
            }
        }
    }

    res
}

fn determine_short_contained_within_long<T: PartialEq>(short: &[T], long: &[T]) -> bool {
    let short_len = short.len();
    let long_len = long.len();

    let mut i = 0;

    while i + short_len <= long_len {
        if long
            .get(i..i + short_len)
            .filter(|&s| s.eq(short))
            .is_some()
        {
            return true;
        }

        i += 1;
    }

    false
}
