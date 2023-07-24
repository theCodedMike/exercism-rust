use std::collections::HashMap;

pub fn encode(mut n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }

    let scale_map: HashMap<i32, &str> = HashMap::from([
        (1, " thousand"),
        (2, " million"),
        (3, " billion"),
        (4, " trillion"),
        (5, " quadrillion"),
        (6, " quintillion"),
    ]);

    let special_num_map: HashMap<u64, &str> = HashMap::from([
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
        (10, "ten"),
        (11, "eleven"),
        (12, "twelve"),
        (13, "thirteen"),
        (14, "fourteen"),
        (15, "fifteen"),
        (16, "sixteen"),
        (17, "seventeen"),
        (18, "eighteen"),
        (19, "nineteen"),
        (20, "twenty"),
        (30, "thirty"),
        (40, "forty"),
        (50, "fifty"),
        (60, "sixty"),
        (70, "seventy"),
        (80, "eighty"),
        (90, "ninety"),
        (100, " hundred"),
    ]);

    let mut res = "".to_string();

    let mut div_count = 0;
    while n > 0 {
        let rem = n % 1000;
        if rem > 0 {
            let sub_word = process_rem(rem, &special_num_map);
            res.insert_str(0, &sub_word);
        }
        n /= 1000;
        div_count += 1;
        if n > 0 && rem > 0 {
            res.insert(0, ' ');
        }
        if n > 0 && n % 1000 != 0 {
            res.insert_str(0, scale_map[&div_count]);
        }
    }

    res
}

fn process_rem(mut rem: u64, special_num_map: &HashMap<u64, &str>) -> String {
    let mut sub_word = "".to_string();
    let mut rem_ge_100 = false;
    if rem >= 100 {
        rem_ge_100 = true;
        sub_word.push_str(special_num_map[&(rem / 100)]);
        sub_word.push_str(special_num_map[&100]);
        rem %= 100;
    }
    if rem > 0 {
        if rem_ge_100 {
            sub_word.push(' ');
        }
        match special_num_map.get(&rem) {
            None => {
                let most = rem / 10 * 10; // 23 -> 20
                sub_word.push_str(special_num_map[&most]);
                sub_word.push('-');
                sub_word.push_str(special_num_map[&(rem - most)]); // 23 - 20 == 3
            }
            Some(&val) => {
                sub_word.push_str(val);
            }
        }
    }

    sub_word
}
