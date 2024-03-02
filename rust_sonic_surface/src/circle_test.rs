use std::io::Write;
// // prototype, sending positions to blender using redis, working
use std::{io, thread, time};
use std::f64::consts::PI;
use redis::Commands;
use rmp_serde::encode::{to_vec, write};


fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut redis_con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let start_x = 0.05;   // 5cm
    let start_y = 0.05;   // 5cm
    let start_z = 0.14;   // 14cm
    let mut freq = 0.5;
    let mut period = 1.0 / freq;
    let radius = 0.02;
    let mut input = String::new();
    let mut n_circles: i32 = 5;



    loop {
        let start_position = (start_x, start_y, start_z);
        let start_position_vec = vec![start_position];
        let msg_packed = to_vec(&start_position_vec).expect("Failed to encode");
        let _: () = redis_con.publish("positions", format!("{:?}", msg_packed)).unwrap();

        print!("Press enter after centering and trapping the particle: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        }

        let circle_start = (start_x + radius, start_y, start_z);

        // move from start position to the start of a circle in 10 steps
        for i in 0..10 {
            let x = start_x + (radius - start_x) * (i as f64) / 10.0;
            let y = start_y;
            let z = start_z;
            let position = (x, y, z);
            // make vector with one position
            let position_vec = vec![position];
            let msg_packed = to_vec(&position_vec).expect("Failed to encode");
            let _: () = redis_con.publish("positions", format!("{:?}", msg_packed)).unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }

        // blank text to start circling 5 times, or w and s to increase and decrease frequency
        // and i and k to increase or decrease number of circles 

        loop {
            println!("Enter to start circling, n_circles (i/k): {:?}, freq (w/s): {:?} hz", n_circles, freq);
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            period = 1.0 / freq;
            if input.trim() == "" {
                println!("Starting to circle");
                for circle_index in 0..n_circles {
                    // output a circle of positions at frequency, e.g. 0.5 hz should 2 seconds per circle
                    for i in 0..360 {
                        let angle = (i as f64) * PI / 180.0;
                        let x = circle_start.0 + radius * (angle).cos();
                        let y = circle_start.1 + radius * (angle).sin();
                        let z = circle_start.2;
                        let position = (x, y, z);
                        let position_vec = vec![position];
                        let msg_packed = to_vec(&position_vec).expect("Failed to encode");
                        let _: () = redis_con.publish("positions", format!("{:?}", msg_packed)).unwrap();
                        thread::sleep(time::Duration::from_millis((1000.0 * period / 360.0) as u64));
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
