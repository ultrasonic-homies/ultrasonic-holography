// // prototype, sending positions to blender using redis, working
mod hat;
use std::{thread, time, io};
use std::f64::consts::PI;
use redis::Commands;
use serde_json;
use hat::Hat;


#[derive(Debug)]
struct Square {
    center: (f64, f64, f64),
    side_length: f64,
}

impl Square {
    fn new(center: (f64, f64, f64), side_length: f64) -> Self {
        Square { center, side_length}
    }

    fn rotate(&self, angle_z: f64) -> Vec<(f64, f64, f64)> {
        let (cx, cy, cz) = self.center;
        let half_side = self.side_length / 2.0;

        let cos_z = angle_z.cos();
        let sin_z = angle_z.sin();

        let vertices = self.vertices();
        let mut rotated_vertices = Vec::new();
        for (x, y, z) in vertices {
            // Translate to the origin
            let x_translated = x - cx;
            let y_translated = y - cy;
            let z_translated = z - cz;

            // Rotate around z-axis
            let x_rotated_z = x_translated * cos_z - y_translated * sin_z;
            let y_rotated_z = x_translated * sin_z + y_translated * cos_z;
            let z_rotated_z = z_translated;

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
            (cx - half_side, cy + half_side, cz), // Front bottom left
            (cx + half_side, cy + half_side, cz), // Front bottom right
            (cx - half_side, cy - half_side, cz), // Back bottom left
            (cx + half_side, cy - half_side, cz), // Back bottom right
        ]
    }
}
fn main() {
    let sonic_surface: bool = false;
    // let mut hat = Hat::new(256.0, 0.14, false, false);
    let helper_sequence_on: bool = false;
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let cube_start_x = if sonic_surface { 0.05} else {0.08};   // m
    let cube_start_y = if sonic_surface { 0.05} else {0.08};   // 5cm
    let cube_start_z = 0.01;   // 14cm
    let freq = 0.5;
    let square = Square::new((cube_start_x, cube_start_y, cube_start_z), 0.045);
    println!("Original vertices: {:?}", square.vertices());
    if helper_sequence_on {
        let loading_x = if sonic_surface { 0.05} else {0.08}; 
        let loading_y = if sonic_surface { 0.05} else {0.08}; 
        let loading_z = 0.03;
        let num_steps = 20;
        println!("Starting helper sequence.");
        let mut current_positions: Vec<(f64, f64, f64)> =  Vec::new();
        for i in 0..square.vertices().len() {
            let (x, y, z) = square.vertices()[i];
            let mut current_positions_copy = current_positions.clone();
            let loading_position = (loading_x, loading_y, loading_z);
            current_positions_copy.push(loading_position);
            let json_string: String = serde_json::to_string(&current_positions_copy).expect("Failed to serialize to JSON");
            let _: () = con.publish("positions", json_string).unwrap();

            println!("Loading vertex {x}, {y}, {z}. Load the particle into {loading_position:?} and press enter");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "" {
                return;
            }
            // move from loading position to vertex, while maintaining the rest of the vertices
            for j in 0..num_steps {
                let curr_x = loading_x + (x - loading_x) * (j as f64) / num_steps as f64;
                let curr_y = loading_y + (y - loading_y) * (j as f64) / num_steps as f64;
                let curr_z = loading_z + (z - loading_z) * (j as f64) / num_steps as f64;
                let mut current_positions_copy = current_positions.clone();
                current_positions_copy.push((curr_x, curr_y, curr_z));
                let json_string: String = serde_json::to_string(&current_positions_copy).expect("Failed to serialize to JSON");
                let _: () = con.publish("positions", json_string).unwrap();
                thread::sleep(time::Duration::from_millis(40));
            }
            // add current vertex to current_positions
            current_positions.push((x, y, z));
        
        }
        println!("Helper sequence complete.");
    }
    // send starting vertices
    let json_string: String = serde_json::to_string(&square.vertices()).expect("Failed to serialize to JSON");
    let _: () = con.publish("positions", json_string).unwrap();
    println!("Load points at the vertices to {:?} if they're not already there, press enter to start rotating", square.vertices());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "" {
        return;
    }
    // Set the current positions    
    let json_string: String = serde_json::to_string(&square.vertices()).expect("Failed to serialize to JSON");
    let _: () = con.publish("positions", json_string).unwrap();
    
    
    // Rotation
    for i in 0.. {
        // let t = (i as f64 * time_inc) % period;
        // let x = start_x + 0.02 * (2.0 * PI * freq * 2.0 * t).sin();
        // let y = start_y + 0.02 * (2.0 * PI * freq * 2.0 * t).cos();
        // let z = start_z;
        // mod i by 360 and turn into radians
        let angle_z = (i % 360) as f64 * PI / 180.0;

        let pos_vector = square.rotate(angle_z);
        // println!("{:?}", pos_vector);
        let json_string: String = serde_json::to_string(&pos_vector).expect("Failed to serialize to JSON");
        // println!("{:?}", angle_z);
        let _: () = con.publish("positions", json_string).unwrap();


        // Sleep for 10 milliseconds
        thread::sleep(time::Duration::from_millis(10));
    }
}
