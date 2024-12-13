use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Vector {
    pub x: i64,
    pub y: i64,
}

impl Vector {
    pub fn new(x: i64, y: i64) -> Vector {
        Vector { x, y }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Div<i64> for Vector {
    type Output = Vector;

    fn div(self, rhs: i64) -> Self::Output {
        Vector{ x: self.x / rhs, y: self.y / rhs }
    }
}

impl Rem<i64> for Vector {
    type Output = Vector;

    fn rem(self, rhs: i64) -> Self::Output {
        Vector { x: self.x % rhs, y: self.y % rhs }
    }
}