pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let row_len = input.len();

    input
        .into_iter()
        .enumerate()
        .filter_map(|(x, row)| {
            let max = match row.iter().max() {
                None => return None, // row may be emtpy
                Some(&max) => max,
            };

            let row_res = row
                .into_iter()
                .enumerate()
                .filter_map(|(y, v)| {
                    if *v != max {
                        return None;
                    }
                    if (0..row_len).all(|i| input[i][y] >= *v) {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if row_res.is_empty() {
                None
            } else {
                Some(row_res)
            }
        })
        .flatten()
        .collect::<Vec<_>>()
}
