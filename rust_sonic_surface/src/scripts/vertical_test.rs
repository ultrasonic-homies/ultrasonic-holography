// always repeat yourself
use std::io::Write;
// // prototype, sending positions to blender using redis, working
use std::{io, thread, time};
use std::f64::consts::PI;
use redis::Commands;
use serde_json; // Import serde_json crate

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut redis_con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let start_x = 0.05;   // 5cm
    let start_y = 0.05;   // 5cm
    let start_z = 0.005;   // 14cm
    let mut freq = 0.2;
    let mut period = 1.0 / freq;
    let mut amplitude= 0.005;
    let mut input = String::new();
    let mut n_oscillations: i32 = 1;



    loop {
        println!("Setting to start position. Place the particle at {:?}, {:?}, {:?}", start_x, start_y, start_z);
        let start_position = (start_x, start_y, start_z);
        let position_vec = vec![start_position];
        let json_string: String = serde_json::to_string(&position_vec).expect("Failed to serialize to JSON");
        let _: () = redis_con.publish("positions", json_string).unwrap();


        println!("Enter to start vertical oscillation, Amplitude (i/k): {:?}, freq (w/s): {:?} hz", amplitude, freq);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        period = 1.0 / freq;
        if input.trim() == "" {
            println!("Starting to oscillate");
            for loop_index in 0..n_oscillations {
                // output a circle of positions at frequency, e.g. 0.5 hz should 2 seconds per circle
                for i in 0..360 {
                    let angle = (i as f64) * PI / 180.0;
                    let x = start_x;
                    let y = start_y;
                    let z = start_z + angle.sin() * amplitude/2.0;
                    let position = (x, y, z);
                    let position_vec = vec![position];
                    // let msg_packed = to_vec(&position_vec).expect("Failed to encode");
                    let json_string: String = serde_json::to_string(&position_vec).expect("Failed to serialize to JSON");
                    let _: () = redis_con.publish("positions", json_string).unwrap();
                    // thread::sleep(time::Duration::from_millis((1000.0 * period / 360.0) as u64));
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
                    amplitude += 0.01;
                } else if c == 'k' {
                    amplitude -= 0.01;
                }
            }
        }
    }
}
