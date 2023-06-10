use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res = HashSet::new();
    let word = word.to_lowercase();
    if word.is_empty() {
        return res;
    }
    let sorted_word = to_chars_and_sort(&word);

    let mut possible_word;
    let mut possible_sorted_word;
    for &p in possible_anagrams {
        possible_word = p.to_lowercase();
        if possible_word.is_empty() {
            continue;
        }
        if word == possible_word {
            continue;
        }

        possible_sorted_word = to_chars_and_sort(&possible_word);

        if sorted_word == possible_sorted_word {
            res.insert(p);
        }

        possible_word.clear();
        possible_sorted_word.clear();
    }

    res
}

fn to_chars_and_sort(word: &str) -> Vec<char> {
    let mut sorted_vec = vec![];

    for char in word.chars() {
        sorted_vec.push(char);
    }

    sorted_vec.sort_unstable();

    sorted_vec
}
