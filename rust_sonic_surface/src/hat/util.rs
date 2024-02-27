use serde_derive::{Serialize, Deserialize}; 


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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn sub(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
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
