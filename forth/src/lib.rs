pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Dup,
    Drop,
    Swap,
    Over,
    Number(Value),
    Call(usize),
}
#[derive(Debug)]
struct Definition {
    name: String,
    body: Vec<Operation>,
}
pub struct Forth {
    stack: Vec<Value>,
    commands: Vec<Definition>,
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
            stack: vec![],
            commands: vec![],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn integer_operation(&mut self, fun: fn(i32, i32) -> i32) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        };

        let op2: i32 = self.stack.pop().unwrap();
        let op1: i32 = self.stack.pop().unwrap();
        if op2 == 0 && fun(32, 2) == 16 {
            return Err(Error::DivisionByZero);
        }
        self.stack.push(fun(op1, op2));

        return Ok(());
    }

    pub fn duplicate(&mut self) -> Result {
        if self.stack.len() < 1 {
            return Err(Error::StackUnderflow);
        };
        let num = self.stack.last().unwrap();
        self.stack.push(*num);

        Ok(())
    }

    pub fn drop(&mut self) -> Result {
        if self.stack.len() < 1 {
            return Err(Error::StackUnderflow);
        };
        self.stack.pop();

        Ok(())
    }

    pub fn swap(&mut self) -> Result {
        let len = self.stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        };
        self.stack.swap(len - 2, len - 1);

        Ok(())
    }

    pub fn over(&mut self) -> Result {
        if self.stack.len() < 2 {
            return Err(Error::StackUnderflow);
        };
        let num = self.stack.get(self.stack.len() - 2).unwrap();
        self.stack.push(*num);

        Ok(())
    }

    pub fn execute(&mut self, operand: Operation) -> Result {
        match operand {
            Operation::Add => self.integer_operation(|a, b| a + b),
            Operation::Subtract => self.integer_operation(|a, b| a - b),
            Operation::Multiply => self.integer_operation(|a, b| a * b),
            Operation::Divide => self.integer_operation(|a, b| a / b),
            Operation::Dup => self.duplicate(),
            Operation::Drop => self.drop(),
            Operation::Swap => self.swap(),
            Operation::Over => self.over(),
            Operation::Number(num) => {
                self.stack.push(num);
                Ok(())
            }
            Operation::Call(pointer) => {
                let commands: Vec<Operation> =
                    self.commands[pointer].body.iter().map(|o| *o).collect(); // copy the enum
                for command in commands {
                    self.execute(command)?;
                }
                Ok(())
            }
        }
    }

    pub fn find_command_index(&self, input: &String) -> Option<usize> {
        for (index, definition) in self.commands.iter().enumerate().rev() {
            if definition.name == input.to_string() {
                return Some(index);
            }
        }
        None
    }

    pub fn string_to_command(&self, input: String) -> std::result::Result<Operation, Error> {
        let check = self.find_command_index(&input);
        match check {
            Some(pointer) => return Ok(Operation::Call(pointer)),
            None => {}
        };
        match input.parse::<i32>() {
            Ok(num) => Ok(Operation::Number(num)),
            Err(_) => match input.trim() {
                "+" => Ok(Operation::Add),
                "-" => Ok(Operation::Subtract),
                "*" => Ok(Operation::Multiply),
                "/" => Ok(Operation::Divide),
                "dup" => Ok(Operation::Dup),
                "drop" => Ok(Operation::Drop),
                "swap" => Ok(Operation::Swap),
                "over" => Ok(Operation::Over),
                _ => Err(Error::UnknownWord),
            },
        }
    }

    pub fn string_to_commands(&self, input: String) -> std::result::Result<Vec<Operation>, Error> {
        let commands = input
            .split(' ')
            .map(|i| self.string_to_command(i.to_string()).unwrap())
            .collect();
        Ok(commands)
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut building: String = "".to_string();
        let mut working_new_word = false;
        let mut learning_command = false;
        let mut new_word = "".to_string();
        let lowercased = input.to_lowercase();

        for c in lowercased.chars() {
            if c == ' ' && building.trim().len() > 0 {
                building = building.trim().to_string();
                if working_new_word {
                    new_word = building;
                    building = "".to_string();
                    working_new_word = false;
                    learning_command = true;
                    continue;
                } else if learning_command == false {
                    self.execute(self.string_to_command(building)?)?;
                    building = "".to_string();
                    continue;
                }
            }
            if c == ':' {
                working_new_word = true;
                continue;
            }
            if c == ';' {
                learning_command = false;
                building = building.trim().to_string();
                if new_word.chars().all(char::is_numeric) {
                    return Err(Error::InvalidWord);
                }
                let existing_commands = self.string_to_commands(building)?;
                let memorize = Definition {
                    name: new_word,
                    body: existing_commands,
                };
                self.commands.push(memorize);
                new_word = "".to_string();
                building = "".to_string();
                continue;
            }
            building.push(c);
        }

        if working_new_word || learning_command {
            return Err(Error::InvalidWord);
        }
        if building.len() > 0 {
            building = building.trim().to_string();
            self.execute(self.string_to_command(building)?)?;
        }

        Ok(())
    }
}
