const QUESTION: &str = "Sure.";
const YELL: &str = "Whoa, chill out!";
const YELL_AND_QUESTION: &str = "Calm down, I know what I'm doing!";
const SILENCE: &str = "Fine. Be that way!";
const OTHER: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let mut lowercase_letters = 0;
    let mut uppercase_letters = 0;
    let mut has_question_mark = false;
    let mut digit_letters = 0;

    for c in message.chars() {
        match c {
            'a'..='z' | 'A'..='Z' => {
                if has_question_mark {
                    has_question_mark = false;
                }
                if 'a' <= c && c <= 'z' {
                    lowercase_letters += 1;
                } else {
                    uppercase_letters += 1;
                }
            }
            '0'..='9' => digit_letters += 1,
            '?' => has_question_mark = true,
            _ => {}
        }
    }

    if has_question_mark {
        return if uppercase_letters > 0 && lowercase_letters == 0 {
            // 有问号?且只有大写字母
            YELL_AND_QUESTION
        } else {
            // 有问号?且字母随便
            QUESTION
        };
    } else {
        if uppercase_letters > 0 && lowercase_letters == 0 {
            // 没有问号?且只有大写字母
            return YELL;
        } else if uppercase_letters == 0 && lowercase_letters == 0 && digit_letters == 0 {
            // 没有问号?且没有大小写字母且没有数字
            return SILENCE;
        }
    }

    OTHER
}
