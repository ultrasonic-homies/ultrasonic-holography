use std::ops::{Add, Div, Mul, Sub};

use serde_derive::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Vec2D<T> {
    pub vec: Vec<T>,
    pub len_i: usize,
    pub len_j: usize,
}

impl<T: std::clone::Clone> Vec2D<T> {
    /// create a new Vec2D initialized to 0
    pub fn new(init: T, i: usize, j: usize) -> Vec2D<T> {
        Vec2D {
            len_i: i,
            len_j: j,
            vec: vec![init; i * j],
        }
    }

    pub fn set(&mut self, i: usize, j: usize, val: T) {
        self.vec[self.len_j * i + j] = val; // row-major order, row-first indexing
    }

    pub fn ix(&self, i: usize, j: usize) -> &T {
        &self.vec[self.len_j * i + j] // row-major order, row-first indexing
    }

    pub fn size(&self) -> (usize, usize) {
        (self.len_i, self.len_j)
    }
}

impl Vec2D<()> {
    pub fn calc_ix(i: usize, j: usize, len_j: usize) -> usize {
        len_j * i + j
    }

    pub fn calc_ij(ix: usize, len_j: usize) -> (usize, usize) {
        (ix / len_j, ix % len_j)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z }
    }

    pub fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn norm(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn print(&self) -> String {
        let a: [f32; 3] = [self.x, self.y, self.z];
        format!("{:?}", a)
    }
}

impl Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: rhs.x * self,
            y: rhs.y * self,
            z: rhs.z * self,
        }
    }
}

impl Div<f32> for Point {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Point> for f32 {
    type Output = Point;

    fn div(self, rhs: Point) -> Self::Output {
        Point {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}
