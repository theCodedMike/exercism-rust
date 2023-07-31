use std::ops::{Add, Sub};

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: Default + Copy + PartialEq + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().any(|x| *x == T::default()) {
            return None;
        }
        let mut sorted_sides = sides;
        sorted_sides.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if sorted_sides[2] - (sorted_sides[0] + sorted_sides[1]) > T::default() {
            return None;
        }
        Some(Triangle {
            sides: sorted_sides,
        })
    }
    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }
    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[0] != self.sides[2]
    }
    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[1] == self.sides[2]
            || self.sides[0] == self.sides[2]
    }
}
