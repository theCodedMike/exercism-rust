pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count)
            .map(|i| {
                let mut row = Vec::with_capacity((i + 1) as usize);
                (0..=i).for_each(|j| {
                    if j == 0 || j == i {
                        row.push(1_u32);
                    } else {
                        row.push((row[j as usize - 1] * (i + 1 - j)) / j);
                    }
                });
                row
            })
            .collect()
    }
}
