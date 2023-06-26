const END_TEMPLATE: &str = "And all for the want of a #.";
const OTHER_TEMPLATE: &str = "For want of a # the @ was lost.";

pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        1 => END_TEMPLATE.replace("#", list[0]),
        _ => {
            let mut sentences = list
                .windows(2)
                .map(|s| OTHER_TEMPLATE.replace("#", s[0]).replace("@", s[1]))
                .collect::<Vec<_>>();
            sentences.push(END_TEMPLATE.replace("#", list[0]));

            sentences.join("\n")
        }
    }
}
