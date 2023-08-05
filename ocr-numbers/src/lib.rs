// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::ops::Index;

const ROW_COUNT: usize = 4;
const COLUMN_COUNT: usize = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.split('\n').collect::<Vec<_>>();
    let rows = lines.len();
    if rows % ROW_COUNT != 0 {
        return Err(Error::InvalidRowCount(rows));
    }
    let cols = lines[0].len();
    if cols % COLUMN_COUNT != 0 {
        return Err(Error::InvalidColumnCount(cols));
    }

    let mut res = "".to_string();
    lines
        .chunks(ROW_COUNT)
        .enumerate()
        .for_each(|(idx, _4_lines)| {
            if idx > 0 {
                res.push(',');
            }
            let mut col_idx = 0;
            while col_idx != cols {
                let range = col_idx..col_idx + COLUMN_COUNT;
                let digit = process_single_digit(
                    _4_lines[0].index(range.clone()),
                    _4_lines[1].index(range.clone()),
                    _4_lines[2].index(range),
                );
                res.push(digit);
                col_idx += COLUMN_COUNT;
            }
        });

    Ok(res)
}

fn process_single_digit(line_1: &str, line_2: &str, line_3: &str) -> char {
    match line_1 {
        "   " => match line_2 {
            "  |" => '1',
            "|_|" => '4',
            _ => '?',
        },
        " _ " => match line_2 {
            "  |" => '7',
            "| |" => match line_3 {
                "|_|" => '0',
                _ => '?',
            },
            " _|" => match line_3 {
                "|_ " => '2',
                " _|" => '3',
                _ => '?',
            },
            "|_ " => match line_3 {
                "|_|" => '6',
                " _|" => '5',
                _ => '?',
            },
            "|_|" => match line_3 {
                "|_|" => '8',
                " _|" => '9',
                _ => '?',
            },
            _ => '?',
        },
        _ => '?',
    }
}
