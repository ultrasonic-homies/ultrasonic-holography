use num::complex::Complex;
use rust_sonic_surface::hat::{p, Gorkov, Hat, Point};
use std::io::Write;

fn main() {
    let hat = Hat::new(32.0, 0.10, true);
    let gorkov = Gorkov::new(256.0, 0.16, false);

    // solve for a control point
    let cps = vec![Point::new(0.025, 0.05, 0.05), Point::new(0.025, 0.05, 0.05)];
    // let phases = hat.run_hat(&cps);
    let phases = gorkov.run_gorkov(&cps);

    let transducers: Vec<Point> = hat.transducers;
    let reflected_transducers: Vec<Point> = transducers
        .iter()
        .map(|p| Point {
            x: p.x,
            y: p.y,
            z: -p.z,
        })
        .collect();

    let N = 160;
    let N_sep = 0.16 / N as f32;
    let mut field = vec![vec![vec![Complex::new(0.0, 0.0); N]; N]; N];

    for x in 0..N {
        for y in 0..N {
            if y == 0 {
                print!("\r{:.1}%", x as f32 / N as f32 * 100.0);
                std::io::stdout().flush().unwrap();
            }

            for z in 0..N {
                let point = Point::new(
                    x as f32 + N_sep / 2.0,
                    y as f32 + N_sep / 2.0,
                    z as f32 + N_sep / 2.0,
                );

                for i in 0..transducers.len() {
                    let vec_r = point * N_sep - transducers[i];
                    let r = vec_r.norm();
                    let theta = (vec_r.z / r).acos();
                    field[x][y][z] += phases[i] * p(r, theta, 0.0);
                }

                for i in 0..reflected_transducers.len() {
                    let vec_r = point * N_sep - reflected_transducers[i];
                    let r = vec_r.norm();
                    let theta = (vec_r.z / r).acos();
                    field[x][y][z] += phases[i] * p(r, theta, 0.0);
                }
            }
        }
    }

    // println!("{:?}", field);

    let s = serde_pickle::to_vec(&field, Default::default()).unwrap();
    let mut file = std::fs::File::create("field.pickle").unwrap();
    file.write_all(&s).unwrap();
}
