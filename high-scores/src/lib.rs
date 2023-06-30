#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|v| *v)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|v| *v)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut ori = self.scores().iter().map(|v| *v).collect::<Vec<_>>();
        ori.sort_unstable_by(|a, b| b.cmp(a));
        return if ori.len() < 3 {
            ori
        } else {
            let (left, _right) = ori.split_at(3);
            Vec::from(left)
        };
    }
}
