use std::collections::HashMap;
use std::ops::Index;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    syntax: HashMap<String, String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            syntax: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.as_ref()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let len = input.len();
        let mut commands = vec![];

        if input.contains(":") {
            let colon_idx = match input.find(":") {
                None => return Err(Error::InvalidWord),
                Some(idx) => idx,
            };

            let semicolon_idx = match input.rfind(";") {
                None => return Err(Error::InvalidWord),
                Some(idx) => idx,
            };

            if colon_idx != 0 {
                commands.push(input.index(..colon_idx));
            }
            if semicolon_idx != len - 1 {
                commands.push(input.index(semicolon_idx + 1..));
            }

            let original_command = input
                .index(colon_idx..=semicolon_idx)
                .split(":")
                .filter(|&w| !w.is_empty())
                .collect::<Vec<_>>();

            for x in original_command {
                let words = x
                    .split_whitespace()
                    .filter(|&w| !w.eq(";"))
                    .map(|w| w.to_lowercase())
                    .collect::<Vec<_>>();
                let key = words[0].to_lowercase();
                if key.parse::<Value>().is_ok() {
                    return Err(Error::InvalidWord);
                }
                let mut val = words.index(1..).join(" ").to_string();

                for (k, v) in self.syntax.iter() {
                    if val.contains(k) {
                        val = val.replace(k, v);
                    }
                }

                self.syntax.insert(key, val);
            }
        } else {
            commands.push(input);
        }

        // replace
        let new_commands = commands
            .iter()
            .map(|&c| {
                let mut new = c.to_string();

                for (key, val) in self.syntax.iter() {
                    new = new
                        .split_whitespace()
                        .map(|w| if w.eq_ignore_ascii_case(key) { val } else { w })
                        .collect::<Vec<_>>()
                        .join(" ");
                }

                new
            })
            .collect::<Vec<_>>();

        // process
        for command in new_commands {
            for word in command.split_whitespace() {
                if let Err(err) = self.process_word(word) {
                    return Err(err);
                }
            }
        }

        Ok(())
    }

    fn process_word(&mut self, word: &str) -> Result {
        match word {
            "0" | _ if word.parse::<Value>().is_ok() => {
                // process digit
                self.stack.push(word.parse::<Value>().unwrap())
            }
            "+" | "-" | "*" | "/" => {
                // get right value of operator
                let right = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                // get left value of operator
                let left = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                // operate
                let res = match integer_arithmetic(left, right, word) {
                    Ok(res) => res,
                    Err(err) => return Err(err),
                };

                self.stack.push(res);
            }
            _ if word.eq_ignore_ascii_case("dup") => {
                let top = match self.stack.last() {
                    None => return Err(Error::StackUnderflow),
                    Some(&val) => val,
                };
                self.stack.push(top);
            }
            _ if word.eq_ignore_ascii_case("drop") => {
                if let None = self.stack.pop() {
                    return Err(Error::StackUnderflow);
                }
            }
            _ if word.eq_ignore_ascii_case("swap") => {
                // get right value of swap
                let right = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                // get left value of swap
                let left = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                self.stack.push(right);
                self.stack.push(left);
            }
            _ if word.eq_ignore_ascii_case("over") => {
                // get right value of over
                let right = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                // get left value of over
                let left = match self.stack.pop() {
                    None => return Err(Error::StackUnderflow),
                    Some(val) => val,
                };
                self.stack.push(left);
                self.stack.push(right);
                self.stack.push(left);
            }
            _ => {
                if self.syntax.is_empty() {
                    return Err(Error::UnknownWord);
                }
            }
        }

        Ok(())
    }
}

fn integer_arithmetic(
    left: Value,
    right: Value,
    operator: &str,
) -> std::result::Result<Value, Error> {
    match operator {
        "+" => Ok(left + right),
        "-" => Ok(left - right),
        "*" => Ok(left * right),
        "/" => {
            if right == 0 {
                return Err(Error::DivisionByZero);
            }
            Ok(left / right)
        }
        _ => Err(Error::UnknownWord),
    }
}
