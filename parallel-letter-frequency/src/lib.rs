use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    std::thread::scope(|s| {
        let mut map = HashMap::new();
        let len = input.len();
        if len == 0 {
            return map;
        }

        let chunk_size = (len as f64 / worker_count as f64).ceil() as usize;
        let mut iter = input.chunks(chunk_size);
        let mut handles = vec![];

        for _ in 0..worker_count {
            let chunks = iter.next();

            handles.push(s.spawn(move || {
                let mut frequency = HashMap::new();

                match chunks {
                    None => {}
                    Some(chunks) => chunks
                        .iter()
                        .filter(|&&line| !line.trim().is_empty())
                        .for_each(|&line| {
                            line.to_lowercase().chars().for_each(|c| {
                                if c.is_alphabetic() {
                                    frequency
                                        .entry(c)
                                        .and_modify(|count| *count += 1)
                                        .or_insert(1);
                                }
                            })
                        }),
                }

                frequency
            }));
        }

        for handle in handles {
            match handle.join() {
                Ok(frequency) => {
                    for (key, val) in frequency {
                        map.entry(key)
                            .and_modify(|count| *count += val)
                            .or_insert(val);
                    }
                }
                Err(_) => {
                    println!("Failed to join");
                }
            }
        }

        map
    })
}
