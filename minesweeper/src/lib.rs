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

    for i in 0..rows {
        let mut row_str = String::new();

        for j in 0..cols {
            let c = &minefield[i][j..j + 1];
            match c {
                " " => {
                    let count = get_mine_count(i, j, minefield, rows as isize, cols as isize);
                    if count == 0 {
                        row_str.push_str(" ");
                    } else {
                        row_str.push_str(count.to_string().as_str());
                    }
                }
                _ => {
                    row_str.push_str(c);
                }
            }
        }

        res.push(row_str);
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
