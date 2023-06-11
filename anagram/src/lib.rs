use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            !candidate_lower.is_empty()
                && word_lower != candidate_lower
                && word_sorted == get_sorted(&candidate_lower)
        })
        .copied()
        .collect()
}

fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted = word.chars().collect::<Vec<_>>();

    word_sorted.sort_unstable();

    word_sorted
}
