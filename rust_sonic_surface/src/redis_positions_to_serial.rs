mod hat;
mod serial_port_helper;
mod sonic_surface;

use futures::StreamExt;
use redis_async::{client, resp::FromResp};
use std::any::type_name;
use tokio::sync::broadcast;
use std::time::SystemTime;
// import Point from hat util
use hat::Hat;
use hat::Point;
use serial_port_helper::{choose_serial_port, list_serial_ports};
use sonic_surface::convert_to_sonic_surface_output;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[tokio::main]
async fn main() {
    // create serial connection
    let Ok(port_names) = list_serial_ports() else {
        eprintln!("Error: Unable to list serial ports.");
        return;
    };
    let Some(selected_port) = choose_serial_port(&port_names) else {
        eprintln!("Invalid selected port, exiting.");
        return;
    };
    println!("You selected serial port: {}", selected_port);

    let baud_rate = 230_400;

    let builder = serialport::new(selected_port.clone(), baud_rate);
    println!("{:?}", &builder);
    let mut serial_conn = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", selected_port, e);
        ::std::process::exit(1);
    });

    // Connect to Redis
    let pubsub_con = client::pubsub_connect("127.0.0.1", 6379)
        .await
        .expect("Cannot connect to Redis");
    let mut msgs = pubsub_con
        .subscribe("positions")
        .await
        .expect("Cannot subscribe to topic");
    // Create a broadcast channel to receive messages
    let hat = Hat::new(32.0, 0.095, true);
    while let Some(message) = msgs.next().await {
        match message {
            Ok(message) => {
                let start_time = SystemTime::now();
                let msg = String::from_resp(message).unwrap();

                // println!("Received message: {:?}", msg);
                // create hat points from the list of points like [[1,2,3]]
                let control_points: Vec<Point> =
                    serde_json::from_str(&msg).expect("Failed to parse JSON");
                // println!("Received control points: {:?}", control_points);
                let phases: Vec<f32> = hat.run_hat(&control_points);
                let ss_output = convert_to_sonic_surface_output(&phases);
                // println!("Sending message: {:?}", ss_output);
                let processing_dur = start_time.elapsed().unwrap();

                serial_conn.write_all(&ss_output).unwrap();
                serial_conn.flush().unwrap();
                let total_dur = start_time.elapsed().unwrap();
                // println!("processing time: {:?} s, total time {:?} s", processing_dur.as_secs_f32(), total_dur.as_secs_f32());
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }
}
