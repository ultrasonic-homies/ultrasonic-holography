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
use std::collections::HashMap;


fn main() {
    let mut board = Board::new().unwrap();
    board.set_preset_calibration();
    board.calibrate();

    let start_x = 0.089;   // 5cm
    let start_y = 0.089;   // 5cm
    let start_z = 0.001;   // 14cm
    let mut freq = 0.5;
    let mut period = 1.0 / freq;
    let mut amplitude= 0.05;
    let mut input = String::new();
    let mut n_oscillations: i32 = 2;
    let hat_runner: HatRunner = HatRunner::new(256.0, 0.153);
    let hat_whatever = Hat::new(256.0, 0.172, false, false);


    loop {
        let point = Point {
            x: start_x,
            y: start_y,
            z: start_z,
        };
        let position_vec = vec![point];
        // let position_vec = vec![vec![point]];
        let phases = hat_whatever.run_hat(&position_vec);
        board.set_frame(&phases);
        // blank text to start circling 5 times, or w and s to increase and decrease frequency
        // and i and k to increase or decrease number of circles 
        let mut divisions = 360;

        loop {
            println!("------------------------------------");
            println!("Calculating phases for frequency = {:?}, divisions = {:?}", freq, divisions);
            let start_time = SystemTime::now();
            let mut phases_vecs = vec![];
            for i in 0..divisions {
                let angle = (i as f64) * 2.0 * PI / (divisions as f64);
                let x = start_x + amplitude * angle.sin() as f32;
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
            let end_time = SystemTime::now();
            println!("Time to calculate phases: {:?}s", end_time.duration_since(start_time).unwrap().as_secs_f32());
          

            println!("Enter to start linear oscillation, Divisions (i/k) : {:?}, freq (w/s): {:?} hz", divisions, freq);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            period = 1.0 / freq;
            if input.trim() == "" {
                println!("Starting to oscillate");
                for _ in 0..n_oscillations {
                    for phase_vec in &phases_vecs {
                        board.set_frame(&phase_vec);
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
                        divisions *= 2;
                    } else if c == 'k' {
                        divisions /= 2;
                    }
                }
            }
        }
    }
}
