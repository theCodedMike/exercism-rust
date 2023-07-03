pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    let mut res = true;

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                if stack.is_empty() {
                    res = false;
                    break;
                }

                if !is_match(stack.pop().unwrap(), c) {
                    res = false;
                    break;
                }
            }
            _ => {}
        }
    }

    res && stack.is_empty()
}

fn is_match(left: char, right: char) -> bool {
    match (left, right) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}
