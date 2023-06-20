const PRE_TEMPLATE_0: &str = "No more bottles of beer on the wall, no more bottles of beer.\n";
const PRE_TEMPLATE_1: &str = "1 bottle of beer on the wall, 1 bottle of beer.\n";
const PRE_TEMPLATE_OTHER: &str = "# bottles of beer on the wall, # bottles of beer.\n";

const MID_TEMPLATE_0: &str = "Go to the store and buy some more, ";
const MID_TEMPLATE_1: &str = "Take it down and pass it around, ";
const MID_TEMPLATE_OTHER: &str = "Take one down and pass it around, ";

const POST_TEMPLATE_0: &str = "99 bottles of beer on the wall.\n";
const POST_TEMPLATE_1: &str = "no more bottles of beer on the wall.\n";
const POST_TEMPLATE_2: &str = "1 bottle of beer on the wall.\n";
const POST_TEMPLATE_OTHER: &str = "# bottles of beer on the wall.\n";

pub fn verse(n: u32) -> String {
    get_pre_template(n) + &get_mid_template(n) + &get_post_template(n)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = "".to_string();

    for i in (end..=start).rev() {
        res += &verse(i);
        if i != end {
            res += "\n";
        }
    }

    res
}

fn get_pre_template(n: u32) -> String {
    match n {
        0 => PRE_TEMPLATE_0.to_string(),
        1 => PRE_TEMPLATE_1.to_string(),
        _ => PRE_TEMPLATE_OTHER.replace("#", &n.to_string()),
    }
}

fn get_mid_template(n: u32) -> String {
    match n {
        0 => MID_TEMPLATE_0.to_string(),
        1 => MID_TEMPLATE_1.to_string(),
        _ => MID_TEMPLATE_OTHER.to_string(),
    }
}

fn get_post_template(n: u32) -> String {
    match n {
        0 => POST_TEMPLATE_0.to_string(),
        1 => POST_TEMPLATE_1.to_string(),
        2 => POST_TEMPLATE_2.to_string(),
        _ => POST_TEMPLATE_OTHER.replace("#", &(n - 1).to_string()),
    }
}
