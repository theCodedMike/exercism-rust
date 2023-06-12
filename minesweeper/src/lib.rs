pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = vec![];
    let rows = minefield.len();
    if rows == 0 {
        return res;
    }
    let cols = minefield[0].len();
    if cols == 0 {
        res.push(String::new());
        return res;
    }

    for (i, &row_str) in minefield.iter().enumerate() {
        let mut res_row = String::new();

        for (j, ch) in row_str.chars().enumerate() {
            match ch {
                ' ' => {
                    let count = get_mine_count(i, j, minefield, rows as isize, cols as isize);
                    if count == 0 {
                        res_row.push(ch);
                    } else {
                        res_row.push_str(count.to_string().as_str());
                    }
                }
                _ => res_row.push(ch),
            }
        }

        res.push(res_row);
    }

    res
}

fn get_mine_count(
    row_idx: usize,
    col_idx: usize,
    minefield: &[&str],
    row_len: isize,
    col_len: isize,
) -> usize {
    let mut count = 0;

    for i in row_idx..=row_idx + 2 {
        for j in col_idx..=col_idx + 2 {
            let real_i = i as isize - 1;
            let real_j = j as isize - 1;

            if 0 <= real_i && real_i < row_len && 0 <= real_j && real_j < col_len {
                let ch = &minefield[real_i as usize][real_j as usize..real_j as usize + 1];
                if ch == "*" {
                    count += 1;
                }
            }
        }
    }

    count
}
