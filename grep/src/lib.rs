use anyhow::Error;
use std::fs;

/// While using `&[&str]` to handle flags is convenient for exercise purposes,
/// and resembles the output of [`std::env::args`], in real-world projects it is
/// both more convenient and more idiomatic to contain runtime configuration in
/// a dedicated struct. Therefore, we suggest that you do so in this exercise.
///
/// In the real world, it's common to use crates such as [`clap`] or
/// [`structopt`] to handle argument parsing, and of course doing so is
/// permitted in this exercise as well, though it may be somewhat overkill.
///
/// [`clap`]: https://crates.io/crates/clap
/// [`std::env::args`]: https://doc.rust-lang.org/std/env/fn.args.html
/// [`structopt`]: https://crates.io/crates/structopt
#[derive(Debug, Default)]
pub struct Flags {
    _n: bool,
    _l: bool,
    _i: bool,
    _v: bool,
    _x: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut obj = Flags::default();

        for &flag in flags {
            match flag {
                "-n" => obj._n = true,
                "-l" => obj._l = true,
                "-i" => obj._i = true,
                "-v" => obj._v = true,
                "-x" => obj._x = true,
                _ => panic!("unsupported flag: {}", flag),
            }
        }

        obj
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    // todo!
    let result = files
        .iter()
        .map(|&file| {
            let content = fs::read_to_string(file)?;
            let matched_lines = content
                .lines()
                .enumerate()
                .filter_map(|(idx, line)| {
                    let mut pattern = pattern.to_string();
                    let mut line = line.to_string();
                    if flags._i {
                        line = line.to_lowercase();
                        pattern = pattern.to_lowercase();
                    }

                    if line.contains(&pattern) {
                        if line.len() == pattern.len() {
                            // all match
                        } else {
                            //
                        }
                    } else {
                        if flags._v {
                        } else {
                        }
                    }
                    Some(format!("{}", idx + 1))
                })
                .collect::<Vec<_>>();

            match (matched_lines.is_empty(), flags._l) {
                (true, _) => Ok(vec![]),
                (false, true) => Ok(vec![file.to_string()]),
                (false, false) => Ok(matched_lines),
            }
        })
        .collect::<Result<Vec<Vec<String>>, Error>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    Ok(result)
}
