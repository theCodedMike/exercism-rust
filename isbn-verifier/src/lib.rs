/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    let mut valid_digit = 0;

    for c in isbn.chars().rev() {
        match c {
            '0'..='9' => {
                valid_digit += 1;
                sum += valid_digit * c.to_digit(10).unwrap();
            }
            '-' => {}
            'X' => {
                valid_digit += 1;
                if valid_digit != 1 {
                    return false;
                }
                sum += 10;
            }
            _ => return false,
        }
    }

    if sum == 0 || valid_digit != 10 {
        return false;
    }

    sum % 11 == 0
}
