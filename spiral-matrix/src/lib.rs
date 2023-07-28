pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut size = size as usize;
    let mut res = vec![vec![0; size]; size];

    let iter_count = (size as f64 / 2.0).ceil() as usize;
    let mut num = 1;
    for i in 0..iter_count {
        process_outer_layer(i, i, &mut res, &mut num, size);

        if size <= 2 {
            if size == 1 {
                res[i][i] = num;
            }
            break;
        }

        size -= 2;
    }

    res
}

fn process_outer_layer(
    mut x: usize,
    mut y: usize,
    matrix: &mut Vec<Vec<u32>>,
    num: &mut u32,
    size: usize,
) {
    // left -> right
    for _ in 1..size {
        matrix[x][y] = *num;
        *num += 1;
        y += 1;
    }

    // top
    //  ↓
    // bottom
    for _ in 1..size {
        matrix[x][y] = *num;
        *num += 1;
        x += 1;
    }

    // left <- right
    for _ in 1..size {
        matrix[x][y] = *num;
        *num += 1;
        y -= 1;
    }

    // top
    //  ↑
    // bottom
    for _ in 1..size {
        matrix[x][y] = *num;
        *num += 1;
        x -= 1;
    }
}
