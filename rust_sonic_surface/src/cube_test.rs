// // prototype, sending positions to blender using redis, working
use std::{thread, time, io};
use std::f64::consts::PI;
use redis::Commands;
use serde_json;


#[derive(Debug)]
struct Cube {
    center: (f64, f64, f64),
    side_length: f64,
}

impl Cube {
    fn new(center: (f64, f64, f64), side_length: f64) -> Self {
        Cube { center, side_length }
    }

    fn rotate(&self, angle_x: f64, angle_y: f64, angle_z: f64) -> Vec<(f64, f64, f64)> {
        let (cx, cy, cz) = self.center;
        let half_side = self.side_length / 2.0;

        let cos_x = angle_x.cos();
        let sin_x = angle_x.sin();
        let cos_y = angle_y.cos();
        let sin_y = angle_y.sin();
        let cos_z = angle_z.cos();
        let sin_z = angle_z.sin();

        let vertices = self.vertices();
        let mut rotated_vertices = Vec::new();
        for (x, y, z) in vertices {
            // Translate to the origin
            let x_translated = x - cx;
            let y_translated = y - cy;
            let z_translated = z - cz;

            // Rotate around x-axis
            let x_rotated_x = x_translated;
            let y_rotated_x = y_translated * cos_x - z_translated * sin_x;
            let z_rotated_x = y_translated * sin_x + z_translated * cos_x;

            // Rotate around y-axis
            let x_rotated_y = x_rotated_x * cos_y + z_rotated_x * sin_y;
            let y_rotated_y = y_rotated_x;
            let z_rotated_y = -x_rotated_x * sin_y + z_rotated_x * cos_y;

            // Rotate around z-axis
            let x_rotated_z = x_rotated_y * cos_z - y_rotated_y * sin_z;
            let y_rotated_z = x_rotated_y * sin_z + y_rotated_y * cos_z;
            let z_rotated_z = z_rotated_y;

            // Translate back to the center
            let x_final = x_rotated_z + cx;
            let y_final = y_rotated_z + cy;
            let z_final = z_rotated_z + cz;

            rotated_vertices.push((x_final, y_final, z_final));
        }
        rotated_vertices
    }

    fn vertices(&self) -> Vec<(f64, f64, f64)> {
        let (cx, cy, cz) = self.center;
        let half_side = self.side_length / 2.0;

        vec![
            (cx - half_side, cy - half_side, cz + half_side), // Front bottom left
            (cx + half_side, cy - half_side, cz + half_side), // Front bottom right
            (cx + half_side, cy + half_side, cz + half_side), // Front top right
            (cx - half_side, cy + half_side, cz + half_side), // Front top left
            (cx - half_side, cy - half_side, cz - half_side), // Back bottom left
            (cx + half_side, cy - half_side, cz - half_side), // Back bottom right
            (cx + half_side, cy + half_side, cz - half_side), // Back top right
            (cx - half_side, cy + half_side, cz - half_side), // Back top left
        ]
    }
}
fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let start_x = 0.05;   // 5cm
    let start_y = 0.05;   // 5cm
    let start_z = 0.05;   // 14cm
    let freq = 0.5;
    let period = 1.0 / freq;
    let cube = Cube::new((start_x, start_y, start_z), 0.025);
    println!("Original vertices: {:?}", cube.vertices());

    let json_string: String = serde_json::to_string(&cube.vertices()).expect("Failed to serialize to JSON");
    let _: () = con.publish("positions", json_string).unwrap();

    println!("Place the particle at the vertices: {:?} and press enter", cube.vertices());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "" {
        return;
    }

    for i in 0.. {
        // let t = (i as f64 * time_inc) % period;
        // let x = start_x + 0.02 * (2.0 * PI * freq * 2.0 * t).sin();
        // let y = start_y + 0.02 * (2.0 * PI * freq * 2.0 * t).cos();
        // let z = start_z;
        // mod i by 360 and turn into radians
        let angle_z = (i % 360) as f64 * PI / 180.0;

        let pos_vector = cube.rotate(angle_z, -angle_z,  0.0);
        // println!("{:?}", pos_vector);
        let json_string: String = serde_json::to_string(&pos_vector).expect("Failed to serialize to JSON");
        // println!("{:?}", angle_z);
        let _: () = con.publish("positions", json_string).unwrap();


        // Sleep for 1 millisecond
        thread::sleep(time::Duration::from_millis(10));
    }
}
