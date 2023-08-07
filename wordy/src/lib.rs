use std::ops::Index;
use std::str::FromStr;

pub fn answer(command: &str) -> Option<i32> {
    let len = command.len();
    let mut operand = vec![];
    let mut operator: Option<Box<dyn Fn(i32, i32) -> i32>> = None;
    let mut result = None;
    let mut operator_idx = 0;

    for (idx, word) in command.index(..len - 1).split(" ").enumerate() {
        match word {
            "plus" | "minus" | "multiplied" | "divided" | "raised" => {
                if operand.len() != 1 {
                    return None;
                }
                if idx - operator_idx == 1 {
                    return None;
                }
                operator_idx = idx;

                match word {
                    "plus" => {
                        operator = Some(Box::new(|a, b| a + b));
                    }
                    "minus" => {
                        operator = Some(Box::new(|a, b| a - b));
                    }
                    "multiplied" => {
                        operator = Some(Box::new(|a, b| a * b));
                    }
                    "divided" => {
                        operator = Some(Box::new(|a, b| a / b));
                    }
                    "raised" => {
                        operator = Some(Box::new(|a, b| a.pow(b as u32)));
                    }
                    _ => {}
                }
            }
            "cubed" => {
                return None;
            }
            _ if word.contains(|ch| '0' <= ch && ch <= '9') => {
                // parse number
                let mut len = word.len();
                loop {
                    let sub_word = word.index(0..len);
                    match i32::from_str(sub_word) {
                        Ok(num) => {
                            operand.push(num);
                            break;
                        }
                        Err(_) => {
                            len -= 1;
                        }
                    }
                }

                // compute
                if operand.len() == 2 {
                    match operator {
                        None => return None,
                        Some(ref op) => {
                            let right = operand.remove(1);
                            let left = operand.remove(0);
                            let val = (op)(left, right);
                            operand.push(val);
                            result = Some(val);
                        }
                    }
                    operator = None;
                }
            }
            _ => {}
        }
    }

    if result.is_some() {
        return result;
    }
    if operand.len() == 1 && operator.is_none() {
        return Some(operand[0]);
    }
    None
}
