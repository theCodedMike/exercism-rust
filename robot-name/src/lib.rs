use rand::Rng;
use std::cell::RefCell;
use std::collections::HashSet;

thread_local! {
    pub static USED_NAMES: RefCell<HashSet<String>> = RefCell::new(
        HashSet::new()
    );
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: gen_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = gen_name();
    }
}

///
/// 随机生成一个名字
///
/// FORMAT: two uppercase letters + three digits, such as RX837 or BC811
///
pub(crate) fn gen_name() -> String {
    loop {
        let mut rng = rand::thread_rng();
        let first_letter = rng.gen_range('A'..='Z');
        let second_letter = rng.gen_range('A'..='Z');
        let digits = rng.gen_range(0..=999);
        let candidate = format!("{}{}{:03}", first_letter, second_letter, digits);

        if USED_NAMES.with(|ref_set| {
            let mut set = ref_set.borrow_mut();
            if !set.contains(&candidate) {
                set.insert(candidate.clone());
                true
            } else {
                false
            }
        }) {
            return candidate;
        }
    }
}
