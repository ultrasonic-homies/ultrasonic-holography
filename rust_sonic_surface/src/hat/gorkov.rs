use lazy_static::lazy_static;
use num::complex::Complex;
use std::f32::consts::PI;

use scilib::math::bessel;

use super::p;
use super::Point;
use super::Vec2D;

const WAVE_LENGTH: f32 = 343.0 / 40000.0;
const OMEGA: f32 = 2.0 * PI * WAVE_LENGTH;
const K: f32 = 2.0 * PI / WAVE_LENGTH;

const P_0: f32 = 1.293; // density of air
const EMITTER_RADIUS: f32 = 0.005; // radius of the emitter: 5 mm

const RHO_P: f32 = 2000.0;
const C_P: f32 = 1000.0;
const C_0: f32 = 343.0;

lazy_static! {
    static ref V_P: f32 = 0.002_f32.powi(3);
    static ref K_1: f32 =
        1.0 / 4.0 * *V_P * (1.0 / (C_0.powi(2) * P_0) - 1.0 / (C_P.powi(2) * RHO_P));
    static ref K_2: f32 = 3.0 / 4.0 * *V_P * (P_0 - RHO_P);
}

pub struct Gorkov {
    pub transducers: Vec<Point>,
    pub phase_res: f32,
    pub z: f32,
}

impl Gorkov {
    // TODO: add support for different transducer arrangements
    pub fn new(phase_res: f32, z: f32, sonic_surface: bool) -> Gorkov {
        let mut transducers: Vec<Point> = vec![];
        let n_transducers: usize;
        let sep: f32;
        if sonic_surface {
            n_transducers = 10;
            sep = 0.01;
        } else {
            n_transducers = 16;
            sep = 0.0105;
        }

        for i in 0..n_transducers {
            for j in 0..n_transducers {
                transducers.push(Point {
                    x: EMITTER_RADIUS + i as f32 * sep,
                    y: EMITTER_RADIUS + j as f32 * sep,
                    z: z,
                })
            }
        }

        return Gorkov {
            transducers,
            phase_res,
            z,
        };
    }

    pub fn run_gorkov(&self, control_points: &Vec<Point>) -> Vec<f32> {
        return calc_transducer_phases(&self.transducers, control_points, self.phase_res)
            .iter()
            .map(|p| p.arg() + PI)
            .collect();
    }
}

fn calc_transducer_phases(
    transducers: &Vec<Point>,
    control_points: &Vec<Point>,
    phase_res: f32,
) -> Vec<Complex<f32>> {
    let mut learning_rate = 10_000_000.0;
    let mut min_gorkov = 100000.0;
    let mut last_gorkov = 0.0;
    let mut t_pressures = vec![Complex::<f32>::new(1.0, 0.0); transducers.len()];

    let reflected_transducers: Vec<Point> = transducers
        .iter()
        .map(|p| Point {
            x: p.x,
            y: p.y,
            z: -p.z,
        })
        .collect();
    let combined_transducers: Vec<Point> = transducers
        .iter()
        .chain(reflected_transducers.iter())
        .map(|p| *p)
        .collect();

    for n in 0..100 {
        let mut cp_gorkov = vec![0.0; control_points.len()];
        let mut grad_sum = vec![0.0; transducers.len()];
        let combined_t_pressures = t_pressures.repeat(2);

        for i in 0..control_points.len() {
            cp_gorkov[i] = gorkov(
                &combined_transducers,
                &combined_t_pressures,
                control_points[i],
            );
            let grad = grad_gorkov(transducers, &t_pressures, control_points[i]);
            let grad_r = grad_gorkov(&reflected_transducers, &t_pressures, control_points[i]);

            for j in 0..grad.len() {
                grad_sum[j] += grad[j] + grad_r[j];
            }
        }

        for i in 0..transducers.len() {
            let phi = t_pressures[i].arg() - learning_rate * grad_sum[i];
            // let phi = t_pressures[i].arg(); //  - learning_rate * grad_sum[i];
            t_pressures[i] = Complex::new(0.0, phi).exp();
        }

        let cur_gorkov: f32 = cp_gorkov.iter().sum();
        if cur_gorkov < min_gorkov {
            min_gorkov = cur_gorkov;
        }
        if cur_gorkov > min_gorkov {
            learning_rate = learning_rate / 2.0;
        }

        if ((cur_gorkov - last_gorkov) / cur_gorkov).abs() < 0.00001 {
            break;
        }

        println!("{:?}", cur_gorkov);
        last_gorkov = cur_gorkov;
    }

    // quantize
    for p in &mut t_pressures {
        let mut theta = p.arg() + PI;
        theta = (theta / (2.0 * PI) * phase_res).round() * 2.0 * PI / phase_res;
        *p = Complex::from_polar(1.0, theta);
    }

    return t_pressures;
}

fn gorkov(transducers: &Vec<Point>, t_pressures: &Vec<Complex<f32>>, control_point: Point) -> f32 {
    let h = 0.0001;
    let p1 = p_sum(transducers, t_pressures, control_point);
    let p2 = p_sum(
        transducers,
        t_pressures,
        control_point + Point::new(0.0, 0.0, h),
    );
    let dpdz = (p2 - p1) / h;

    return *K_1 * p1.norm_sqr() - *K_2 * dpdz.norm_sqr();
}

fn grad_gorkov(
    transducers: &Vec<Point>,
    t_pressures: &Vec<Complex<f32>>,
    control_point: Point,
) -> Vec<f32> {
    let h = 0.0001;
    let p1 = p_sum(transducers, t_pressures, control_point);
    let p2 = p_sum(
        transducers,
        t_pressures,
        control_point + Point::new(0.0, 0.0, h),
    );
    let dpdz = (p2 - p1) / h;

    let mut grad = vec![0.0; t_pressures.len()];

    for i in 0..transducers.len() {
        let vec_r = control_point - transducers[i];
        let phi = t_pressures[i].arg();

        grad[i] = 2.0 * *K_1 * (p1 * dphi(vec_r, phi).conj()).re
            - 2.0 * *K_2 * (dpdz * dzdphi(vec_r, phi).conj()).re;
    }

    return grad;
}

fn dphi(vec_r: Point, phi: f32) -> Complex<f32> {
    let r = vec_r.norm();
    let theta = (vec_r.z / r).acos();
    return Complex::new(0.0, 1.0) * Complex::new(0.0, phi).exp() * p(r, theta, 0.0);
}

fn dzdphi(vec_r: Point, phi: f32) -> Complex<f32> {
    let h = 0.0001;
    return (dphi(vec_r + Point::new(0.0, 0.0, h), phi) - dphi(vec_r, phi)) / h;
}

fn p_sum(
    transducers: &Vec<Point>,
    t_pressures: &Vec<Complex<f32>>,
    control_point: Point,
) -> Complex<f32> {
    let mut pressure = Complex::new(0.0, 0.0);
    for i in 0..transducers.len() {
        let vec_r = control_point - transducers[i];
        let r = vec_r.norm();
        let theta = (vec_r.z / r).acos();
        pressure += t_pressures[i] * p(r, theta, 0.0);
    }

    return pressure;
}
