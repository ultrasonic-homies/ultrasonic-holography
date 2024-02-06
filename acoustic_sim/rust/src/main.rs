#![allow(unused_imports)]
mod hat;
mod hat_mt;
mod hat_rayon;
mod hat_runner;
mod util;

use num::complex::Complex;
use std::f32::consts::PI;
use std::time::Instant;

fn main() {
    #[cfg(feature = "benchmark")]
    {
        benchmark();
        return;
    }

    let radius = 0.02;
    let initial_height = 0.05;

    let t_sep = 0.01;
    let num_t = 100;
    let ts: Vec<f32> = (0..num_t).map(|t| t as f32 * t_sep).collect();
    let xs: Vec<f32> = ts
        .iter()
        .map(|t| radius * (2.0 * PI * t).sin() + initial_height)
        .collect();
    let ys: Vec<f32> = ts
        .iter()
        .map(|t| radius * (2.0 * PI * t).sin() + initial_height)
        .collect();
    let zs: Vec<f32> = ts
        .iter()
        .map(|t| radius * (2.0 * PI * t).sin() + initial_height)
        .collect();

    let cps: Vec<Vec<util::Point>> = xs
        .iter()
        .zip(ys.iter())
        .zip(zs.iter())
        .map(|((x, y), z)| {
            vec![util::Point {
                x: *x,
                y: *y,
                z: *z,
            }]
        })
        .collect();

    let mut phases: Vec<Vec<Complex<f32>>> = vec![];

    let now = Instant::now();
    for control_points in cps {
        phases.push(hat::run_hat(&control_points, 16.0, 0.1));
    }
    let time = now.elapsed();
}

#[cfg(feature = "benchmark")]
fn benchmark() {
    // setup test case
    let t_sep = 0.01;
    let num_t = 100;
    let ts: Vec<f32> = (0..num_t).map(|t| t as f32 * t_sep).collect();
    let xs: Vec<f32> = ts
        .iter()
        .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
        .collect();
    let ys: Vec<f32> = ts
        .iter()
        .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
        .collect();
    let zs: Vec<f32> = ts
        .iter()
        .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
        .collect();

    let cps: Vec<Vec<util::Point>> = xs
        .iter()
        .zip(ys.iter())
        .zip(zs.iter())
        .map(|((x, y), z)| {
            vec![util::Point {
                x: *x,
                y: *y,
                z: *z,
            }]
        })
        .collect();

    // benchmark single-threaded HAT
    println!("Running single-threaded HAT benchmark...");
    let mut phases: Vec<Vec<Complex<f32>>> = vec![];

    let now = Instant::now();
    for control_points in &cps {
        phases.push(hat::run_hat(control_points, 16.0, 0.1));
    }
    let time = now.elapsed();

    println!("Benchmark took {} seconds.", time.as_secs_f32());
    println!("frames per second: {}", num_t as f32 / time.as_secs_f32());

    // benchmark multi-threaded HAT
    println!("Running multi-threaded HAT benchmark...");
    let mut phases: Vec<Vec<Complex<f32>>> = vec![];

    let now = Instant::now();
    for control_points in &cps {
        phases.push(hat_mt::run_hat(control_points, 16.0, 0.1));
    }
    let time = now.elapsed();

    println!("Benchmark took {} seconds.", time.as_secs_f32());
    println!("frames per second: {}", num_t as f32 / time.as_secs_f32());

    // benchmark rayon HAT
    // println!("Running rayon HAT benchmark...");
    // let mut phases: Vec<Vec<Complex<f32>>> = vec![];

    // let now = Instant::now();
    // for control_points in &cps {
    //     phases.push(hat_rayon::run_hat(control_points, 16.0, 0.1));
    // }
    // let time = now.elapsed();

    // println!("Benchmark took {} seconds.", time.as_secs_f32());
    // println!("frames per second: {}", num_t as f32 / time.as_secs_f32());

    // benchmark HatRunner
    println!("Running HatRunner benchmark...");

    let runner = hat_runner::HatRunner {};
    let now = Instant::now();
    let mut phases = runner.run(&cps, 16.0, 0.1);
    let time = now.elapsed();

    println!("Benchmark took {} seconds.", time.as_secs_f32());
    println!("frames per second: {}", num_t as f32 / time.as_secs_f32());
}
