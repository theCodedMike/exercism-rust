use std::collections::{BTreeMap, BTreeSet};

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    roster: BTreeMap<u32, BTreeSet<&'a str>>, // key: grade, val: students
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.roster
            .entry(grade)
            .and_modify(|s| {
                s.insert(student);
            })
            .or_insert_with(|| {
                let mut set = BTreeSet::new();
                set.insert(student);
                set
            });
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().map(|k| *k).collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.roster
            .get(&grade)
            .map_or(vec![], |s| s.iter().map(|n| n.to_string()).collect())
    }
}
