use std::collections::HashMap;
use std::sync::mpsc;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    std::thread::scope(|s| {
        let mut map = HashMap::new();
        let len = input.len();
        if len == 0 {
            return map;
        }

        let (sender, receiver) = mpsc::channel();
        let chunk_size = (len as f64 / worker_count as f64).ceil() as usize;
        let mut iter = input.chunks(chunk_size);

        for _ in 0..worker_count {
            let sender_clone = sender.clone();
            let chunks = iter.next();

            s.spawn(move || match chunks {
                None => {}
                Some(chunks) => chunks
                    .iter()
                    .filter(|&&line| !line.trim().is_empty())
                    .for_each(|&line| {
                        line.to_lowercase().chars().for_each(|c| {
                            if c.is_alphabetic() {
                                if sender_clone.send(c).is_err() {
                                    println!("Fail to send");
                                }
                            }
                        })
                    }),
            });
        }

        // 这行代码必须存在
        drop(sender);

        for val in receiver {
            map.entry(val).and_modify(|count| *count += 1).or_insert(1);
        }

        map
    })
}
