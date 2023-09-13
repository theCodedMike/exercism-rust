use std::collections::HashMap;

pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(mut dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => dice.into_iter().filter(|&v| v == 1).count() as u8 * 1,
        Category::Twos => dice.into_iter().filter(|&v| v == 2).count() as u8 * 2,
        Category::Threes => dice.into_iter().filter(|&v| v == 3).count() as u8 * 3,
        Category::Fours => dice.into_iter().filter(|&v| v == 4).count() as u8 * 4,
        Category::Fives => dice.into_iter().filter(|&v| v == 5).count() as u8 * 5,
        Category::Sixes => dice.into_iter().filter(|&v| v == 6).count() as u8 * 6,
        Category::FullHouse => {
            let mut map = HashMap::new();
            let mut sum = 0;
            for k in dice {
                sum += k;
                map.entry(k).and_modify(|v| *v += 1).or_insert(1);
            }
            if map.len() == 2
                && map.values().all(|&v| v == 2 || v == 3)
                && map.values().map(|v| *v).sum::<u8>() == 5
            {
                sum
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let map = dice
                .into_iter()
                .try_fold(HashMap::new(), |mut acc, k| {
                    acc.entry(k).and_modify(|v| *v += 1).or_insert(1);
                    Some(acc)
                })
                .expect("Failed to fold");
            if map.len() <= 2 && map.values().any(|&v| v >= 4) {
                map.iter()
                    .filter_map(|(&k, &v)| if v >= 4 { Some(k * 4) } else { None })
                    .sum()
            } else {
                0
            }
        }
        Category::LittleStraight => {
            dice.sort_unstable();
            if dice.eq(&[1, 2, 3, 4, 5]) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            dice.sort_unstable();
            if dice.eq(&[2, 3, 4, 5, 6]) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.into_iter().sum(),
        Category::Yacht => {
            let first = dice[0];
            if dice.into_iter().all(|v| v == first) {
                50
            } else {
                0
            }
        }
    }
}
