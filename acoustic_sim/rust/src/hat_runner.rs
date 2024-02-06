use num::complex::Complex;
use std::f32::consts::PI;

use std::thread;

use crate::hat::run_hat;
use crate::util::Point;

pub struct HatRunner {}

impl HatRunner {
    pub fn run(
        &self,
        control_points: &Vec<Vec<Point>>,
        phase_res: f32,
        z: f32,
    ) -> Vec<Vec<Complex<f32>>> {
        // TODO: allow for different transducer configurations
        let mut phases: Vec<Vec<Complex<f32>>> = vec![vec![]; control_points.len()];

        // for k threads, data broken into k segments with segment length n//k = m
        // segments 0..m, m..2m, 2m..3m, ..., (k-1)m..km
        // remainder is km..n
        let k = 8; // TODO: set programatically
        let n = control_points.len();
        let m = n / k;

        let mut phase_chunks = phases.iter_mut().zip(control_points).collect::<Vec<_>>();
        let mut phase_chunks = phase_chunks.chunks_exact_mut(m);
        thread::scope(|s| {
            for chunk in phase_chunks.by_ref() {
                s.spawn(|| {
                    for (ps, cps) in chunk {
                        **ps = run_hat(cps, phase_res, z);
                    }
                });
            }
        });

        thread::scope(|s| {
            for (ps, cps) in phase_chunks.into_remainder() {
                s.spawn(|| {
                    **ps = run_hat(cps, phase_res, z);
                });
            }
        });

        return phases;
    }
}
