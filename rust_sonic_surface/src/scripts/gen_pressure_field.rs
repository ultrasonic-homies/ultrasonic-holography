use num::complex::Complex;
use rust_sonic_surface::hat::{p, Gorkov, Hat, Point};
use std::io::Write;

fn main() {
    let Z = 0.16;
    let hat = Hat::new(256.0, Z, false, true);
    let gorkov = Gorkov::new(256.0, Z, false);

    // solve for a control point
    let cps = vec![
        Point::new(0.08, 0.08, 0.01 + 0.00428 - 0.00428 / 2.0),
        Point::new(0.08, 0.08, 0.01 - 0.00428 - 0.00428 / 2.0),
    ];
    let cps = vec![
        Point::new(0.06, 0.08, 0.08),
        Point::new(0.10, 0.08, 0.08),
        Point::new(0.08, 0.06, 0.08),
        Point::new(0.08, 0.10, 0.08),
    ];

    let phases = hat.run_hat(&cps);
    // let phases = gorkov.run_gorkov(&cps);

    let transducers: Vec<Point> = hat.transducers;
    let reflected_transducers: Vec<Point> = transducers
        .iter()
        .map(|p| Point {
            x: p.x,
            y: p.y,
            z: -p.z,
        })
        .collect();

    let x0 = 0.02;
    let y0 = 0.08;
    let z0 = 0.06;

    let xsize = 0.14 - 0.02;
    let ysize = 0.0;
    let zsize = 0.04;

    let Nx = 160;
    let Ny = 1;
    let Nz = 160;

    let Nx_sep = xsize / Nx as f32;
    let Ny_sep = ysize / Ny as f32;
    let Nz_sep = zsize / Nz as f32;
    let mut field = vec![vec![vec![Complex::new(0.0, 0.0); Nz]; Ny]; Nx];

    for x in 0..Nx {
        for y in 0..Ny {
            // if y == 0 {
            //     print!("\r{:.1}%", x as f32 / Nx as f32 * 100.0);
            //     std::io::stdout().flush().unwrap();
            // }

            for z in 0..Nz {
                let point = Point::new(
                    x as f32 * Nx_sep + Nx_sep / 2.0 + x0,
                    y as f32 * Ny_sep + Ny_sep / 2.0 + y0,
                    z as f32 * Nz_sep + Nz_sep / 2.0 + z0,
                );

                for i in 0..transducers.len() {
                    let vec_r = point - transducers[i];
                    let r = vec_r.norm();
                    let theta = (vec_r.z / r).acos();
                    field[x][y][z] += phases[i] * p(r, theta, 0.0);
                }

                for i in 0..reflected_transducers.len() {
                    let vec_r = point - reflected_transducers[i];
                    let r = vec_r.norm();
                    let theta = (vec_r.z / r).acos();
                    field[x][y][z] += phases[i] * p(r, theta, 0.0);
                }
            }
        }
    }

    println!(
        "\nx: {}, {}",
        Nx_sep / 2.0 + x0,
        (Nx as f32 + Nx_sep / 2.0) * Nx_sep + x0
    );
    println!(
        "y: {}, {}",
        Ny_sep / 2.0 + y0,
        (Ny as f32 + Ny_sep / 2.0) * Ny_sep + y0
    );
    println!(
        "y: {}, {}",
        Nz_sep / 2.0 + z0,
        (Nz as f32 + Nz_sep / 2.0) * Nz_sep + z0
    );

    let s = serde_pickle::to_vec(&field, Default::default()).unwrap();
    let mut file = std::fs::File::create("field.pickle").unwrap();
    file.write_all(&s).unwrap();
}
