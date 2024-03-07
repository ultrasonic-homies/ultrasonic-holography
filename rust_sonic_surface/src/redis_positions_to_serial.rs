mod hat;
mod sonic_surface;
mod serial_port_helper;

use redis_async::{client, resp::FromResp};
use tokio::sync::broadcast;
use futures::StreamExt;
use std::any::type_name;
// import Point from hat util
use hat::Point;
use hat::Hat;
use sonic_surface::convert_to_sonic_surface_output;
use serial_port_helper::{list_serial_ports, choose_serial_port};

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
        .expect("Cannot subscribe to topic");;
    // Create a broadcast channel to receive messages
    let hat = Hat::new(32.0, 0.13);
    while let Some(message) = msgs.next().await {
        match message {
            Ok(message) => {
                let msg = String::from_resp(message).unwrap();
                
                // create hat points from the list of points like [[1,2,3]]
                let control_points: Vec<Point> = serde_json::from_str(&msg).expect("Failed to parse JSON");
                let phases: Vec<f32> = hat.run_hat(&control_points);
                let ss_phases = convert_to_sonic_surface_output(&phases);
                serial_conn.write_all(&ss_phases).unwrap();
                serial_conn.flush().unwrap();     
            }
            Err(e) => {
                eprintln!("ERROR: {}", e);
                break;
            }
        }
    }

}
