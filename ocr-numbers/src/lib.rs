// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::ops::Index;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.split('\n').collect::<Vec<_>>();
    let rows = lines.len();
    if rows % 4 != 0 {
        return Err(Error::InvalidRowCount(rows));
    }
    let cols = lines[0].len();
    if cols % 3 != 0 {
        return Err(Error::InvalidColumnCount(cols));
    }

    let mut res = "".to_string();
    lines.chunks(4).enumerate().for_each(|(idx, _4_lines)| {
        if idx > 0 {
            res.push(',');
        }
        let mut col_idx = 0;
        while col_idx != cols {
            let range = col_idx..col_idx + 3;
            let digit = process_single_digit(
                _4_lines[0].index(range.clone()),
                _4_lines[1].index(range.clone()),
                _4_lines[2].index(range),
            );
            res.push(digit);
            col_idx += 3;
        }
    });

    Ok(res)
}

fn process_single_digit(line_1: &str, line_2: &str, line_3: &str) -> char {
    if line_1 == " _ " && line_2 == "| |" && line_3 == "|_|" {
        return '0';
    }
    if line_1 == "   " && line_2 == "  |" && line_3 == "  |" {
        return '1';
    }
    if line_1 == " _ " && line_2 == " _|" && line_3 == "|_ " {
        return '2';
    }
    if line_1 == " _ " && line_2 == " _|" && line_3 == " _|" {
        return '3';
    }
    if line_1 == "   " && line_2 == "|_|" && line_3 == "  |" {
        return '4';
    }
    if line_1 == " _ " && line_2 == "|_ " && line_3 == " _|" {
        return '5';
    }
    if line_1 == " _ " && line_2 == "|_ " && line_3 == "|_|" {
        return '6';
    }
    if line_1 == " _ " && line_2 == "  |" && line_3 == "  |" {
        return '7';
    }
    if line_1 == " _ " && line_2 == "|_|" && line_3 == "|_|" {
        return '8';
    }
    if line_1 == " _ " && line_2 == "|_|" && line_3 == " _|" {
        return '9';
    }
    return '?';
}
