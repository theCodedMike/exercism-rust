use std::ops::{Add, Sub};

pub struct Triangle<T> {
    sides: [T; 3],
}

impl<T: Copy + Add + Sub + PartialEq + PartialOrd + Ord> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self>
    where
        <T as Add>::Output: PartialOrd<T>,
        <T as Sub>::Output: PartialOrd<T>,
    {
        let max = std::cmp::max(sides[0], sides[1]);
        let min = std::cmp::min(sides[0], sides[1]);
        if max + min > sides[2] && max - min < sides[2] {
            Some(Triangle { sides })
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().all(|e| *e == self.sides[0])
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[0] != self.sides[1]
            && self.sides[1] != self.sides[2]
            && self.sides[0] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[0] == self.sides[1]
            || self.sides[0] == self.sides[2]
            || self.sides[1] == self.sides[2]
    }
}
