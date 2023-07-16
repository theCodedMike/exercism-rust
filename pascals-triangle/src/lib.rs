pub struct PascalsTriangle {
    row_count: usize,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count as usize,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.row_count);

        for i in 0..self.row_count {
            let mut row = Vec::with_capacity(i + 1);
            for j in 0..=i {
                let val;
                if j == 0 || j == i {
                    val = 1_u32;
                } else {
                    val = rows[i - 1][j - 1] + rows[i - 1][j];
                }
                row.push(val);
            }
            rows.push(row);
        }

        rows
    }
}
