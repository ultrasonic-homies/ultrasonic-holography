mod hat;
use std::io::Write;
// // prototype, sending positions to blender using redis, working
use std::{io, thread, time};
use std::f64::consts::PI;
use eframe::epaint::tessellator::path;
use redis::Commands;
use serde_json; // Import serde_json crate
use std::time::SystemTime;
use rev1::board::Board;
use hat::HatRunner;
use hat::Hat;
use hat::Point;


fn main() {
    let mut board = Board::new().unwrap();
    board.set_preset_calibration();
    board.calibrate();

    let time_inc = 0.01;  // secs
    let start_x = 0.09;   // 5cm
    let start_y = 0.09;   // 5cm
    let start_z = 0.01;   // 10cm
    let mut freq = 0.5;
    let mut period = 1.0 / freq;
    let radius = 0.02;
    let mut input = String::new();
    let mut n_circles: i32 = 1;
    let hat_runner: HatRunner = HatRunner::new(256.0, 0.14);
    let hat_whatever = Hat::new(256.0, 0.12, false, false);


    loop {
        let point = Point {
            x: start_x,
            y: start_y,
            z: start_z,
        };
        let position_vec = vec![point];
        // let position_vec = vec![vec![point]];
        let phases = hat_whatever.run_hat(&position_vec);
        board.set_frame_soft_calibrated(&phases);

        print!("Press enter after centering and trapping the particle: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        }
        let steps = 40;
        // move from start position to the start of a circle in 10 steps
        let mut phases_vecs = vec![];
        for i in 0..steps {
            let x = start_x + (radius) * (i as f32) / steps as f32;
            let y = start_y;
            let z = start_z;
            let point = Point {
                x: x,
                y: y,
                z: z,
            };
            // make vector with one position
            let position_vec = vec![point];
            let phases = hat_whatever.run_hat(&position_vec);
            phases_vecs.push(phases);
        }
        for phase_vec in phases_vecs {
            board.set_frame_soft_calibrated(&phase_vec);
            thread::sleep(time::Duration::from_millis(10));
        }

        // blank text to start circling 5 times, or w and s to increase and decrease frequency
        // and i and k to increase or decrease number of circles 

        loop {
            println!("------------------------------------");
            println!("Calculating phases for frequency = {:?}", freq);
            let start_time = SystemTime::now();
            let mut phases_vecs = vec![];
            let divisions = 360;
            for i in 0..divisions {
                let angle = (i as f32 / divisions as f32) * 2.0 *  PI as f32;
                let x = start_x + radius * (angle).cos();
                let y = start_y + radius * (angle).sin();
                let z = start_z;
                let point = Point {
                    x: x,
                    y: y,
                    z: z,
                };
                // make vector with one position
                let position_vec = vec![point];
                let phases = hat_whatever.run_hat(&position_vec);
                phases_vecs.push(phases);
            }
            let end_time = SystemTime::now();
            println!("Time to calculate phases: {:?}s", end_time.duration_since(start_time).unwrap().as_secs_f32());
          

            println!("Enter to start circling, n_circles (i/k): {:?}, freq (w/s): {:.1} hz", n_circles, freq);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            period = 1.0 / freq;
            if input.trim() == "" {
                println!("Starting to circle");
                for _ in 0..n_circles {
                    for phase_vec in &phases_vecs {
                        board.set_frame_soft_calibrated(&phase_vec);
                        thread::sleep(time::Duration::from_millis((1000.0 * period / divisions as f64) as u64));
                    }
                }
                
            }
            else {
                // for each character entered
                for c in input.chars() {
                    if c == 'w' {
                        freq += 0.1;
                    } else if c == 's' {
                        freq -= 0.1;
                    } else if c == 'i' {
                        n_circles += 1;
                    } else if c == 'k' {
                        n_circles -= 1;
                    }
                }
            }
        }
    }
}
