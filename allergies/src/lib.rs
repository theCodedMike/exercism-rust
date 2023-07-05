#[derive(Debug)]
pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,         // 1
    Peanuts,      // 2
    Shellfish,    // 4
    Strawberries, // 8
    Tomatoes,     // 16
    Chocolate,    // 32
    Pollen,       // 64
    Cats,         // 128
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        Allergies::convert_allergen_to_score(allergen) <= self.score
    }

    fn convert_allergen_to_score(allergen: &Allergen) -> u32 {
        match allergen {
            Allergen::Eggs => 1,
            Allergen::Peanuts => 2,
            Allergen::Shellfish => 4,
            Allergen::Strawberries => 8,
            Allergen::Tomatoes => 16,
            Allergen::Chocolate => 32,
            Allergen::Pollen => 64,
            Allergen::Cats => 128,
        }
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        let mut res = vec![];
        if score == 0 {
            return res;
        }

        let mut exp = score.ilog2();

        if 2_u32.pow(exp) == score {
            if let Some(val) = Allergies::convert_score_to_allergen(score) {
                res.push(val);
            }
        } else {
            if score % 2 == 1 {
                // score为奇数
                res.push(Allergen::Eggs);
                score -= 1;
            }
            while score != 0 {
                let part = 2_u32.pow(exp);
                if let Some(val) = Allergies::convert_score_to_allergen(part) {
                    res.push(val);
                }

                exp -= 1;
                score -= part;
            }
        }

        res
    }

    fn convert_score_to_allergen(score: u32) -> Option<Allergen> {
        match score {
            1 => Some(Allergen::Eggs),
            2 => Some(Allergen::Peanuts),
            4 => Some(Allergen::Shellfish),
            8 => Some(Allergen::Strawberries),
            16 => Some(Allergen::Tomatoes),
            32 => Some(Allergen::Chocolate),
            64 => Some(Allergen::Pollen),
            128 => Some(Allergen::Cats),
            _ => None,
        }
    }
}
