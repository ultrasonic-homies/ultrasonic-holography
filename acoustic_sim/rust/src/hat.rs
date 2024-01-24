use num::complex::Complex;
use std::f32::consts::PI;

use scilib::math::bessel;

const WAVE_LENGTH: f32 = 343.0 / 40000.0;
const OMEGA: f32 = 2.0 * PI * WAVE_LENGTH;
const K: f32 = 2.0 * PI / WAVE_LENGTH;

const P_0: f32 = 1.293; // density of air
const EMITTER_RADIUS: f32 = 0.005; // radius of the emitter: 5 mm

#[derive(Debug)]
pub struct Vec2D<T> {
    vec: Vec<T>,
    len_i: usize,
    len_j: usize,
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

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
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
}

// TODO: add support for different transducer arrangements
pub fn run_hat(control_points: &Vec<Point>, phase_res: f32, z: f32) -> Vec<Complex<f32>> {
    let mut transducers: Vec<Point> = vec![];
    let sep = 0.01;
    let size = 10;

    for i in 0..size {
        for j in 0..size {
            transducers.push(Point {
                x: EMITTER_RADIUS + i as f32 * sep,
                y: EMITTER_RADIUS + j as f32 * sep,
                z: z,
            })
        }
    }

    return calc_transducer_phases(&transducers, control_points, phase_res);
}

// far field piston-source model: from https://jontallen.ece.illinois.edu/uploads/473.F18/Lectures/Chapter_7b.pdf
fn p(r: f32, theta: f32, t: f32) -> Complex<f32> {
    let sin_theta = f32::sin(theta);

    let amp: Complex<f32> = Complex::new(0.0, 1.0) * OMEGA * P_0 * EMITTER_RADIUS.powi(2)
        / (2.0 * r)
        * Complex::new(0.0, OMEGA * t + K * r).exp();

    if sin_theta == 0.0 {
        return amp;
    } else {
        return amp * 2.0 * bessel::j_n(1, (K * EMITTER_RADIUS * sin_theta) as f64) as f32
            / (K * EMITTER_RADIUS * sin_theta);
    };
}

fn gen_propagators(transducers: &Vec<Point>, control_points: &Vec<Point>) -> Vec2D<Complex<f32>> {
    let mut propagators: Vec2D<Complex<f32>> = Vec2D::new(
        Complex::new(0.0, 0.0),
        control_points.len(),
        transducers.len(),
    );

    for i in 0..control_points.len() {
        for j in 0..transducers.len() {
            let vec_r = control_points[i].sub(&transducers[j]);
            let r = vec_r.norm();
            let theta = (vec_r.z / r).acos();
            propagators.set(i, j, p(r, theta, 0.0));
        }
    }

    return propagators;
}

fn calc_transducer_phases(
    transducers: &Vec<Point>,
    control_points: &Vec<Point>,
    phase_res: f32,
) -> Vec<Complex<f32>> {
    let propagators = gen_propagators(transducers, control_points);
    let reflected_transducers: Vec<Point> = transducers
        .iter()
        .map(|p| Point {
            x: p.x,
            y: p.y,
            z: -p.z,
        })
        .collect();
    let reflected_propagators = gen_propagators(&reflected_transducers, control_points);
    let mut c_pressures = vec![Complex::<f32>::new(0.0, 0.0); control_points.len()];
    let mut t_pressures = vec![Complex::<f32>::new(1.0, 0.0); transducers.len()];

    for _iter in 0..10 {
        // forward propagate
        for i in 0..c_pressures.len() {
            let mut c = Complex::new(0.0, 0.0);

            // direct contributions
            for j in 0..t_pressures.len() {
                c += t_pressures[i] * propagators.ix(i, j);
            }

            // reflection contributions
            // TODO: can possibly add a reflection coefficient here to account for imperfect
            // reflective surface
            for j in 0..t_pressures.len() {
                c = c + t_pressures[i] * reflected_propagators.ix(i, j);
            }

            // each control point has an amplitude of 1 / n
            c_pressures[i] = c / c.norm() / c_pressures.len() as f32;
        }

        // backwards propagate
        for j in 0..t_pressures.len() {
            let mut pl: Complex<f32> = Complex::new(0.0, 0.0);

            // direct contributions
            for i in 0..c_pressures.len() {
                pl = pl + c_pressures[i] * propagators.ix(i, j).conj();
            }

            // reflection contributions
            // TODO: can possibly add a reflection coefficient here to account for imperfect
            // reflective surface
            for i in 0..c_pressures.len() {
                pl = pl + c_pressures[i] * reflected_propagators.ix(i, j).conj();
            }

            t_pressures[j] = pl;
        }

        // normalize
        let max = t_pressures
            .iter()
            .map(|p| p.norm())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();

        for p in &mut t_pressures {
            *p = *p / max;
        }

        // quantize
        for p in &mut t_pressures {
            let (_r, mut theta) = p.to_polar();
            theta = (theta / (2.0 * PI) * phase_res).round() * 2.0 * PI / phase_res;
            *p = Complex::from_polar(1.0, theta);
        }
    }

    return t_pressures;
}
