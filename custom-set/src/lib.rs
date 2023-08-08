use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomSet<T: PartialEq + Eq + Hash> {
    data: HashSet<T>,
}

impl<T> CustomSet<T>
where
    T: PartialEq + Eq + Hash + Clone,
{
    pub fn new(input: &[T]) -> Self {
        CustomSet {
            data: input.into_iter().cloned().collect(),
        }
    }

    pub fn contains(&self, element: &T) -> bool {
        self.data.contains(element)
    }

    pub fn add(&mut self, element: T) {
        self.data.insert(element);
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        self.data.is_subset(&other.data)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        self.data.is_disjoint(&other.data)
    }

    #[must_use]
    pub fn intersection(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.intersection(&other.data).cloned().collect(),
        }
    }

    #[must_use]
    pub fn difference(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.difference(&other.data).cloned().collect(),
        }
    }

    #[must_use]
    pub fn union(&self, other: &Self) -> Self {
        CustomSet {
            data: self.data.union(&other.data).cloned().collect(),
        }
    }
}
