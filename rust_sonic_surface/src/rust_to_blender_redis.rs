// // prototype, sending positions to blender using redis, working
use std::{thread, time};
use std::f64::consts::PI;
use redis::Commands;
use rmp_serde::encode::{to_vec, write};


fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let start_x = 0.05;   // 5cm
    let start_y = 0.05;   // 5cm
    let start_z = 0.14;   // 14cm
    let freq = 0.5;
    let period = 1.0 / freq;

    for i in 0.. {
        let t = (i as f64 * time_inc) % period;
        let x = start_x + 0.02 * (2.0 * PI * freq * 2.0 * t).sin();
        let y = start_y + 0.02 * (2.0 * PI * freq * 2.0 * t).cos();
        let z = start_z;

        // Print out the positions vector
        let position = (x, y, z);
        let position_2 = (-x, -y, z);
        let pos_vector = vec![position, position_2];
        let a = to_vec( &pos_vector).expect("Failed to encode");
        let _: () = con.publish("positions", format!("{:?}", a)).unwrap();


        // Sleep for 1 millisecond
        thread::sleep(time::Duration::from_millis(10));
    }
}
