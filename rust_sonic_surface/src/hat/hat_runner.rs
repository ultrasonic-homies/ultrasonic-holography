use num::complex::Complex;
use std::f32::consts::PI;

use std::thread;

use super::Hat;
use super::Point;

pub struct HatRunner {
    hat: Hat,
}

impl HatRunner {
    pub fn new(phase_res: f32, z: f32) -> HatRunner {
        return HatRunner {
            hat: Hat::new(phase_res, z, false),
        };
    }

    pub fn run(&self, control_points: &Vec<Vec<Point>>) -> Vec<Vec<f32>> {
        // TODO: allow for different transducer configurations
        let mut phases: Vec<Vec<f32>> = vec![vec![]; control_points.len()];

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
                        **ps = self.hat.run_hat(cps);
                    }
                });
            }
        });

        thread::scope(|s| {
            for (ps, cps) in phase_chunks.into_remainder() {
                s.spawn(|| {
                    **ps = self.hat.run_hat(cps);
                });
            }
        });

        return phases;
    }
}
