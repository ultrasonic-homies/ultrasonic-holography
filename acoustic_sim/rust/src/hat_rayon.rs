use num::complex::Complex;
use std::f32::consts::PI;

use rayon::prelude::*;
use scilib::math::bessel;

use crate::hat::Point;
use crate::hat::Vec2D;

const WAVE_LENGTH: f32 = 343.0 / 40000.0;
const OMEGA: f32 = 2.0 * PI * WAVE_LENGTH;
const K: f32 = 2.0 * PI / WAVE_LENGTH;

const P_0: f32 = 1.293; // density of air
const EMITTER_RADIUS: f32 = 0.005; // radius of the emitter: 5 mm

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
    // let mut propagators: Vec2D<Complex<f32>> = Vec2D::new(
    //     Complex::new(0.0, 0.0),
    //     control_points.len(),
    //     transducers.len(),
    // );

    let ij: Vec<(usize, usize)> = (0..control_points.len())
        .flat_map(|i| (0..transducers.len()).map(move |j| (i, j)))
        .collect();

    let vec: Vec<Complex<f32>> = ij
        .par_iter()
        .map(|(i, j)| {
            let vec_r = control_points[*i].sub(&transducers[*j]);
            let r = vec_r.norm();
            let theta = (vec_r.z / r).acos();
            return p(r, theta, 0.0);
        })
        .collect();

    return Vec2D {
        vec: vec,
        len_i: control_points.len(),
        len_j: transducers.len(),
    };

    // for i in 0..control_points.len() {
    //     for j in 0..transducers.len() {
    //         let vec_r = control_points[i].sub(&transducers[j]);
    //         let r = vec_r.norm();
    //         let theta = (vec_r.z / r).acos();
    //         propagators.set(i, j, p(r, theta, 0.0));
    //     }
    // }

    // return propagators;
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
        let ij: Vec<(usize, usize)> = (0..control_points.len())
            .flat_map(|i| (0..transducers.len()).map(move |j| (i, j)))
            .collect();

        // forward propagate
        // for i in 0..c_pressures.len() {
        c_pressures = (0..c_pressures.len())
            .into_par_iter()
            .map(|i| {
                let mut c = Complex::new(0.0, 0.0);

                // direct contributions
                c += (0..t_pressures.len())
                    .into_par_iter()
                    .fold(
                        || Complex::new(0.0, 0.0),
                        |a, j| a + t_pressures[j] * propagators.ix(i, j),
                    )
                    .sum::<Complex<f32>>();
                // for j in 0..t_pressures.len() {
                //     c += t_pressures[j] * propagators.ix(i, j);
                // }

                // reflection contributions
                // TODO: can possibly add a reflection coefficient here to account for imperfect
                // reflective surface
                c += (0..t_pressures.len())
                    .into_par_iter()
                    .fold(
                        || Complex::new(0.0, 0.0),
                        |a, j| a + t_pressures[j] * reflected_propagators.ix(i, j),
                    )
                    .sum::<Complex<f32>>();
                // for j in 0..t_pressures.len() {
                //     c = c + t_pressures[j] * reflected_propagators.ix(i, j);
                // }

                // each control point has an amplitude of 1 / n
                // c_pressures[i] = c / c.norm() / c_pressures.len() as f32;
                c / c.norm() / c_pressures.len() as f32
            })
            .collect();

        // backwards propagate
        //for j in 0..t_pressures.len() {
        t_pressures = (0..t_pressures.len())
            .into_par_iter()
            .map(|j| {
                let mut pl: Complex<f32> = Complex::new(0.0, 0.0);

                // direct contributions
                // for i in 0..c_pressures.len() {
                //     pl = pl + c_pressures[i] * propagators.ix(i, j).conj();
                // }
                pl += (0..c_pressures.len())
                    .into_par_iter()
                    .fold(
                        || Complex::new(0.0, 0.0),
                        |a, i| a + c_pressures[i] * propagators.ix(i, j).conj(),
                    )
                    .sum::<Complex<f32>>();

                // reflection contributions
                // TODO: can possibly add a reflection coefficient here to account for imperfect
                // reflective surface
                // for i in 0..c_pressures.len() {
                //     pl = pl + c_pressures[i] * reflected_propagators.ix(i, j).conj();
                // }
                pl += (0..c_pressures.len())
                    .into_par_iter()
                    .fold(
                        || Complex::new(0.0, 0.0),
                        |a, i| a + c_pressures[i] * reflected_propagators.ix(i, j).conj(),
                    )
                    .sum::<Complex<f32>>();

                pl
            })
            .collect();

        // normalize
        let max = t_pressures
            .par_iter()
            .map(|p| p.norm())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();

        //for p in &mut t_pressures {
        //    *p = *p / max;
        //}
        t_pressures = t_pressures.par_iter().map(|p| p / max).collect();

        // quantize
        t_pressures = t_pressures
            .par_iter()
            .map(|p| {
                let (_r, mut theta) = p.to_polar();
                theta = (theta / (2.0 * PI) * phase_res).round() * 2.0 * PI / phase_res;
                Complex::from_polar(1.0, theta)
            })
            .collect();
    }

    return t_pressures;
}
