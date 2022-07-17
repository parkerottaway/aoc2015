use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        return Point { x: x, y: y };
    }
}
