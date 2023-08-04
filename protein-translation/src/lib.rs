use std::collections::HashMap;

pub struct CodonsInfo<'a> {
    data: HashMap<&'a str, &'a str>,
}

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.data.get(codon).map(|&v| v)
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        let mut len = rna.len();
        if len == 0 {
            return None;
        }

        let mut left = 0;
        let mut right = 3;
        let mut res = vec![];

        while let Some(codon) = rna.get(left..right) {
            match self.data.get(codon) {
                None => return None,
                Some(&protein) => {
                    if protein == "stop codon" {
                        break;
                    } else {
                        res.push(protein);
                    }
                }
            }
            len -= 3;
            left = right;
            right += if len >= 3 {
                3
            } else {
                if len == 0 {
                    break;
                }
                len
            };
        }

        Some(res)
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    let data = pairs.into_iter().collect::<HashMap<_, _>>();
    CodonsInfo { data }
}
