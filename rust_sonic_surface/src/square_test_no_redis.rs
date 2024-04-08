// // prototype, sending positions to blender using redis, working
mod hat;
use std::{thread, time, io};
use std::f64::consts::PI;
use redis::Commands;
use serde_json;
use hat::Hat;
use hat::Point;
use rev1::board::Board;


#[derive(Debug)]
struct Square {
    center: (f32, f32, f32),
    side_length: f32,
}

impl Square {
    fn new(center: (f32, f32, f32), side_length: f32) -> Self {
        Square { center, side_length}
    }

    fn rotate(&self, angle_z: f64, height: f64) -> Vec<Point> {
        let (cx, cy, cz) = self.center;
        let half_side = self.side_length / 2.0;

        let cos_z = angle_z.cos();
        let sin_z = angle_z.sin();

        let vertices = self.vertices();
        let mut rotated_vertices = Vec::new();
        for point in vertices {
            // Translate to the origin
            let x_translated = point.x - cx;
            let y_translated = point.y - cy;

            // Rotate around z-axis
            let x_rotated_z = x_translated * cos_z as f32 - y_translated * sin_z as f32;
            let y_rotated_z = x_translated * sin_z as f32 + y_translated * cos_z as f32;

            // Translate back to the center
            let x_final = x_rotated_z + cx;
            let y_final = y_rotated_z + cy;
            let z_final = height;

            rotated_vertices.push(Point{x:x_final, y:y_final, z:z_final as f32});
        }
        rotated_vertices
    }

    fn vertices(&self) -> Vec<Point> {
        let (cx, cy, cz) = self.center;
        let half_side = self.side_length / 2.0;

        vec![
            Point {x:(cx - half_side) as f32, y:(cy + half_side) as f32,z: cz as f32}, // Front bottom left
            Point {x:(cx + half_side) as f32, y:(cy + half_side) as f32,z: cz as f32}, // Front bottom right
            Point {x:(cx - half_side) as f32, y:(cy - half_side) as f32,z: cz as f32}, // Back bottom left
            Point {x:(cx + half_side) as f32, y:(cy - half_side) as f32,z: cz as f32}, // Back bottom right
        ]
    }
}
fn main() {
    let sonic_surface: bool = false;
    // let mut hat = Hat::new(256.0, 0.14, false, false);
    let mut board = Board::new().unwrap();
    let hat = Hat::new(256.0, 0.172, sonic_surface, false);
    board.set_preset_calibration();
    board.calibrate();
    let helper_sequence_on: bool = false;
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut con = client.get_connection().expect("Failed to establish redis connection");

    let time_inc = 0.01;  // secs
    let cube_start_x = if sonic_surface { 0.05} else {0.08};   // m
    let cube_start_y = if sonic_surface { 0.05} else {0.08};   // 5cm
    let cube_start_z = 0.04;   // 14cm
    let freq = 0.5;
    let square = Square::new((cube_start_x, cube_start_y, cube_start_z), 0.065);
    println!("Original vertices: {:?}", square.vertices());
    if helper_sequence_on {
        let loading_x = if sonic_surface { 0.05} else {0.08}; 
        let loading_y = if sonic_surface { 0.05} else {0.08}; 
        let loading_z = 0.01;
        let num_steps = 20;
        println!("Starting helper sequence.");
        let mut current_positions: Vec<Point> =  Vec::new();
        for i in 0..square.vertices().len() {
            let vertex = square.vertices()[i];
            let mut current_positions_copy = current_positions.clone();
            let loading_position = Point {x:loading_x, y:loading_y, z:loading_z};
            current_positions_copy.push(loading_position);
            let phases = hat.run_hat(&current_positions_copy);
            board.set_frame(&phases);

            println!("Loading vertex {vertex:?} Load the particle into {loading_position:?} and press enter");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "" {
                return;
            }
            // move from loading position to vertex, while maintaining the rest of the vertices
            for j in 0..num_steps {
                let curr_x = loading_x + (vertex.x - loading_x) * (j as f32) / num_steps as f32;
                let curr_y = loading_y + (vertex.y - loading_y) * (j as f32) / num_steps as f32;
                let curr_z = loading_z + (vertex.z - loading_z) * (j as f32) / num_steps as f32;
                let mut current_positions_copy = current_positions.clone();
                current_positions_copy.push(Point {x:curr_x, y: curr_y, z:curr_z});
                let phases = hat.run_hat(&current_positions_copy);
                board.set_frame(&phases);
                thread::sleep(time::Duration::from_millis(40));
            }
            // add current vertex to current_positions
            current_positions.push(vertex);
        
        }
        println!("Helper sequence complete.");
    }
    // send starting vertices
    let phases = hat.run_hat(&square.vertices());
    board.set_frame(&phases);
    println!("Load points at the vertices to {:?} if they're not already there, press enter to start rotating", square.vertices());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "" {
        return;
    }


    // calculate phases
    let mut phase_list = Vec::new();
    let divisions = 180;
    let mut prev_phases: Option<Vec<f32>> = None;

    let z_amplitude:f64 = 0.01;
    for i in 0..divisions {
        // let t = (i as f64 * time_inc) % period;
        // let x = start_x + 0.02 * (2.0 * PI * freq * 2.0 * t).sin();
        // let y = start_y + 0.02 * (2.0 * PI * freq * 2.0 * t).cos();
        // let z = start_z;
        // mod i by 360 and turn into radians
        let angle_z = (i) as f64 * 2.0 * PI / divisions as f64;
        // height should oscillate from cube_start_z to something higher based on amplitude
        let height = cube_start_z as f64 + z_amplitude - z_amplitude * angle_z.cos() as f64;
        let pos_vector = square.rotate(angle_z, height as f64);
        // println!("{:?}", pos_vector);
        let mut phases = hat.run_hat(&pos_vector);

        // //calculate the average difference from the previous phases
        // let mut avg_diff: f32 = 0.0;
        // if let Some(prev_phases) = prev_phases {
        //     let mut sum_diff = 0.0;
        //     for i in 0..phases.len() {
        //         sum_diff += (phases[i] - prev_phases[i]);
        //     }
        //     avg_diff = sum_diff / phases.len() as f32;
        //     println!("Average difference: {:?}", avg_diff)
        // }
        // //subtract the average difference from the current phases
        // for i in 0..phases.len() {
        //     phases[i] -= avg_diff;
        // }
        // prev_phases = Some(phases.clone());
        phase_list.push(phases);

    }

    for i in 0.. {
        let phases = &phase_list[i % divisions];
        board.set_frame(&phases);
        // Sleep for 10 milliseconds
        thread::sleep(time::Duration::from_millis(30));
    }
}
