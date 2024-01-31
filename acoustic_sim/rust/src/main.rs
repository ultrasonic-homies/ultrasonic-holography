#![allow(unused_imports)]
// mod hat;
// mod hat_mt;
mod hat_rayon;

use num::complex::Complex;
use std::f32::consts::PI;
use std::time::Instant;

use std::sync::Arc;

// fn main() {
//     let control_points = vec![hat_mt::Point {
//         x: 0.05,
//         y: 0.05,
//         z: 0.05,
//     }];
//
//     let phases = hat_mt::run_hat(Arc::new(control_points), 16.0, 0.1);
//     println!("ran");
//     println!("{:?}", phases);
// }

fn main() {
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

    let cps: Vec<Vec<hat_rayon::Point>> = xs
        .iter()
        .zip(ys.iter())
        .zip(zs.iter())
        .map(|((x, y), z)| {
            vec![hat_rayon::Point {
                x: *x,
                y: *y,
                z: *z,
            }]
        })
        .collect();

    let mut phases: Vec<Vec<Complex<f32>>> = vec![];

    // benchmark
    let now = Instant::now();
    for control_points in cps {
        phases.push(hat_rayon::run_hat(&control_points, 16.0, 0.1));
    }
    let time = now.elapsed();

    println!("Benchmark took {} seconds.", time.as_secs_f32());
    println!("frames per second: {}", 100.0 / time.as_secs_f32());
}

// fn main() {
//     // setup test case
//     let control_points = vec![hat::Point {
//         x: 0.05,
//         y: 0.05,
//         z: 0.05,
//     }];
//
//     let t_sep = 0.01;
//     let num_t = 100;
//     let ts: Vec<f32> = (0..num_t).map(|t| t as f32 * t_sep).collect();
//     let xs: Vec<f32> = ts
//         .iter()
//         .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
//         .collect();
//     let ys: Vec<f32> = ts
//         .iter()
//         .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
//         .collect();
//     let zs: Vec<f32> = ts
//         .iter()
//         .map(|t| 0.02 * (2.0 * PI * t).sin() + 0.05)
//         .collect();
//
//     let cps: Vec<Vec<hat::Point>> = xs
//         .iter()
//         .zip(ys.iter())
//         .zip(zs.iter())
//         .map(|((x, y), z)| {
//             vec![hat::Point {
//                 x: *x,
//                 y: *y,
//                 z: *z,
//             }]
//         })
//         .collect();
//
//     let mut phases: Vec<Vec<Complex<f32>>> = vec![];
//
//     // benchmark
//     let now = Instant::now();
//     for control_points in cps {
//         phases.push(hat::run_hat(&control_points, 16.0, 0.1));
//     }
//     let time = now.elapsed();
//
//     println!("Benchmark took {} seconds.", time.as_secs_f32());
//     println!("frames per second: {}", 100.0 / time.as_secs_f32());
// }
