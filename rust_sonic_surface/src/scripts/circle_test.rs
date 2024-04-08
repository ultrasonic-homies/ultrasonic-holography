use std::io::Write;
// // prototype, sending positions to blender using redis, working
use std::{io, thread, time};
use std::f64::consts::PI;
use redis::Commands;
use serde_json; // Import serde_json crate
use std::time::SystemTime;


fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut redis_con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let start_x = 0.08;   // 5cm
    let start_y = 0.08;   // 5cm
    let start_z = 0.02;   // 10cm
    let mut freq = 0.5;
    let mut period = 1.0 / freq;
    let radius = 0.02;
    let mut input = String::new();
    let mut n_circles: i32 = 1;



    loop {
        let start_position = (start_x, start_y, start_z);
        let position_vec = vec![start_position];
        let json_string: String = serde_json::to_string(&position_vec).expect("Failed to serialize to JSON");
        let _: () = redis_con.publish("positions", json_string).unwrap();

        print!("Press enter after centering and trapping the particle: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        }
        let steps = 40;
        // move from start position to the start of a circle in 10 steps
        for i in 0..steps {
            let x = start_x + (radius) * (i as f64) / steps as f64;
            let y = start_y;
            let z = start_z;
            let position = (x, y, z);
            // make vector with one position
            let position_vec = vec![position];
            let json_string: String = serde_json::to_string(&position_vec).expect("Failed to serialize to JSON");
            let _: () = redis_con.publish("positions", json_string).unwrap();
            let curr_time = SystemTime::now();
            println!("Sent position: at time: {:?}", curr_time);
            thread::sleep(time::Duration::from_millis(10));
        }

        // blank text to start circling 5 times, or w and s to increase and decrease frequency
        // and i and k to increase or decrease number of circles 

        loop {
            println!("------------------------------------");
            println!("Enter to start circling, n_circles (i/k): {:?}, freq (w/s): {:.1} hz", n_circles, freq);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            period = 1.0 / freq;
            if input.trim() == "" {
                println!("Starting to circle");
                for _ in 0..n_circles {
                    // output a circle of positions at frequency, e.g. 0.5 hz should 2 seconds per circle
                    let divisions = 360;
                    for i in (0..divisions) {
                        let angle = (i as f64 / divisions as f64) * 2.0 *  PI;
                        let x = start_x + radius * (angle).cos();
                        let y = start_y + radius * (angle).sin();
                        let z = start_z;
                        // let position = (x, y, z);
                        // make 4 traps in cardinal directions in x and y
                        // distance is 3 mm
  
                        // let position_vec = vec![position1, position2, position3, position4];
                        let position_vec = vec![(x, y, z)];
                        // let msg_packed = to_vec(&position_vec).expect("Failed to encode");
                        let json_string: String = serde_json::to_string(&position_vec).expect("Failed to serialize to JSON");
                        let _: () = redis_con.publish("positions", json_string).unwrap();
                        let curr_time = SystemTime::now();
                        println!("Sent position: at time: {:?}", curr_time);
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
