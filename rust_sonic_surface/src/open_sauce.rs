mod hat;
mod serial_port_helper;
mod sonic_surface;

use futures::StreamExt;
use redis_async::{client, resp::FromResp};
use serde_derive::Deserialize;
use std::any::type_name;
use tokio::sync::broadcast;
use std::time::SystemTime;
// import Point from hat util
use hat::Hat;
use hat::Gorkov;
use hat::Point;
use serial_port_helper::{choose_serial_port, list_serial_ports};
use sonic_surface::convert_to_sonic_surface_output;
use rev1::board::Board;
use std::time::Duration;
use rmp_serde::decode::from_slice;



fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Message {
    r#type: String,
    command: String,
}


#[tokio::main]
async fn main() {
    let haptic_feedback: bool = false;
    // let mut board = Board::new().unwrap();
    // if doing haptic feedback, setting board to 200 hz helps a lot
    if haptic_feedback {
        // board.modulate(200.0, true);
    }
    // board.modulate(200.0, true);
    // board.set_preset_calibration();
    // board.calibrate();
    // Connect to Redis
    let pubsub_con = client::pubsub_connect("127.0.0.1", 6379)
        .await
        .expect("Cannot connect to Redis");
    let mut msgs = pubsub_con
        .subscribe("commands")
        .await
        .expect("Cannot subscribe to topic");
    // Create a broadcast channel to receive messages
    let hat = Hat::new(256.0, 0.148, false, false);
    println!("Ready for command messages");
    let mut playing_music = false;
    while let Some(Ok(redis_msg)) = msgs.next().await {
        match redis_msg {
            redis_async::resp::RespValue::BulkString(ref u8_array)  => {
                match from_slice::<Message>(u8_array) {
                    Ok(actual_msg) => {
                        // println!("Received message: {:?}", actual_msg);
                        // Additional processing logic here
                        if actual_msg.r#type == "p" { //positions
                            if playing_music {
                                // board.shut_up();
                                playing_music = false;
                            }
                            let control_points: Vec<Point> =
                                serde_json::from_str(&actual_msg.command).expect("Failed to parse JSON");
                            // println!("Received control points: {:?}", control_points);
                            // println!("Received control points: {:?}", control_points);
                            let phases: Vec<f32> = hat.run_hat(&control_points);
                            // let t1 = SystemTime::now();
                            // let ss_output = convert_to_sonic_surface_output(&phases);
                            // println!("Sending message: {:?}", ss_output);
                            // let processing_dur = start_time.elapsed().unwrap();
                            // board.set_frame(&phases);
                        } else if actual_msg.r#type == "m" { //music
                            let music_command: &str = &actual_msg.command;
                            // convert to String
                            let msg = String::from(music_command);
                            let split_msg: Vec<&str> = msg.split(",").collect();
                            let freq: f32 = split_msg[0].parse().unwrap();
                            let on_off: bool = split_msg[1].parse().unwrap();
                            // println!("Received music: {:?}", split_msg);
                            // board.modulate_two_boards(freq, on_off);
                            playing_music = true;
                            // do something with music
                        }

                    }   
                    Err(e) => {
                        eprintln!("Failed to parse message: {}", e);
                    }
                }
                
            }
            _ => {
                eprintln!("Unexpected message format");
                println!("{:?}", redis_msg);
            } 
        }
    }

    // while let Some(message) = msgs.next().await {
    //     match message {
    //         Ok(message) => {
    //             // convert message to Message using rmp message pack
    //             let message_bytes = message.get_payload_bytes();
    //             let msg: Message = from_slice(&message).unwrap();
    //             println!("Received message: {:?}", msg);
    //             // println!("Received message: {:?}", msg);
    //             // create hat points from the list of points like [[1,2,3]]
    //             let msg: Message =
    //                 serde_json::from_str(&msg).expect("Failed to parse JSON");
                
    //             // check if "type" is "music" or "positions". If it's positions, make vector of Points
    //             if msg.r#type == "p" { //positions
    //                 if playing_music {
    //                     // board.shut_up();
    //                     playing_music = false;
    //                 }
    //                 let control_points: Vec<Point> =
    //                     serde_json::from_str(&msg.command).expect("Failed to parse JSON");
    //                 println!("Received control points: {:?}", control_points);
    //                 // println!("Received control points: {:?}", control_points);
    //                 let phases: Vec<f32> = hat.run_hat(&control_points);
    //                 // let t1 = SystemTime::now();
    //                 // let ss_output = convert_to_sonic_surface_output(&phases);
    //                 // println!("Sending message: {:?}", ss_output);
    //                 // let processing_dur = start_time.elapsed().unwrap();
    //                 // board.set_frame(&phases);
    //             } else if msg.r#type == "m" { //music
    //                 let music_command: &str = &msg.command;
    //                 // convert to String
    //                 let msg = String::from(music_command);
    //                 let split_msg: Vec<&str> = msg.split(",").collect();
    //                 let freq: f32 = split_msg[0].parse().unwrap();
    //                 let on_off: bool = split_msg[1].parse().unwrap();
    //                 println!("Received music: {:?}", split_msg);
    //                 // board.modulate_two_boards(freq, on_off);
    //                 playing_music = true;
    //                 // do something with music
    //             }
  
          
    //         }
    //         Err(e) => {
    //             eprintln!("ERROR: {}", e);
    //             break;
    //         }
    //     }
    // }
}
