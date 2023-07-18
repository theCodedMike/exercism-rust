use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }
    match calc_aliquot_sum(num).cmp(&num) {
        Ordering::Less => Some(Classification::Deficient),
        Ordering::Equal => Some(Classification::Perfect),
        Ordering::Greater => Some(Classification::Abundant),
    }
}

fn calc_aliquot_sum(num: u64) -> u64 {
    let half = num / 2;
    (1..=half).filter(|&i| num % i == 0).sum()
}
