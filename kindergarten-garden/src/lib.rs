pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let seed_to_plants = |seeds: &str, pos: usize| {
        seeds
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| {
                if idx / 2 == pos {
                    match c {
                        'R' => Some("radishes"),
                        'C' => Some("clover"),
                        'G' => Some("grass"),
                        'V' => Some("violets"),
                        _ => panic!("Unknown Seed: {}", c),
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };

    [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ]
    .into_iter()
    .zip(0..12)
    .filter_map(|(name, idx)| {
        if name == student {
            let (first_row, second_row) =
                diagram.split_once('\n').expect("Failed to split diagram");
            let mut plants = seed_to_plants(first_row, idx);
            plants.append(&mut seed_to_plants(second_row, idx));
            Some(plants)
        } else {
            None
        }
    })
    .flatten()
    .collect()
}
