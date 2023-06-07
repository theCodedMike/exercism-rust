use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut res = String::new();

    for char in input.graphemes(true).rev() {
        res.push_str(char)
    }

    res
}
