pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let text = text.chars().collect::<Vec<_>>();
        let rows = self.rails;
        let total_len = text.len();
        let mut res = "".to_string();
        let cycle_width = (rows - 1) * 2;

        for idx in 0..rows {
            let mut start = idx;
            let mut step = idx * 2;
            while start < total_len {
                res.push(text[start]);
                step = cycle_width - step;
                if step == 0 {
                    step = cycle_width;
                }
                start += step;
            }
        }

        res
    }

    pub fn decode(&self, cipher: &str) -> String {
        let rows = self.rails;
        let cipher = cipher.chars().collect::<Vec<_>>();
        let total_len = cipher.len();
        let mut res = vec![' '; total_len];
        let cycle_width = (rows - 1) * 2;

        let mut c_idx = 0;
        for i in 0..rows {
            let mut start = i;
            let mut step = i * 2;
            while start < total_len {
                res[start] = cipher[c_idx];
                c_idx += 1;
                step = cycle_width - step;
                if step == 0 {
                    step = cycle_width;
                }
                start += step;
            }
        }

        res.into_iter().collect()
    }
}
