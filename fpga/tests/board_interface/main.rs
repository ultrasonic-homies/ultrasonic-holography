use rev1::board::Board;
use libftd2xx::{list_devices, DeviceInfo};
use std::io::{self, Write};

fn main() {

    println!("Hello, world!");
    let devices: Vec<DeviceInfo> = list_devices().unwrap();
    for device in devices {
        println!("device properties: {:?}", device);
    }
    match Board::new() {
        Ok(mut board) => {
            loop {
                let zero_frame: Vec<f32> = vec![0.0; 256];
                let flip_frame: Vec<f32> = vec![1.6; 256];
                let mut input = String::new();
                let mut quit: bool = false;
                input.clear();
                println!("1: Send Zeros\n2: Send 180 degrees\n3: Send Calibration Phases\n4: Calibrate\n0: Quit");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<u8>() {
                    Ok(parsed_u8) => match parsed_u8 {
                        0 => break,
                        1 => board.set_frame(&zero_frame),
                        2 => board.set_frame(&flip_frame),
                        3 => board.set_preset_calibration(),
                        4 => board.calibrate(),
                        _ => println!("Input invalid"),
                    }
                    Err(err) => {
                        println!("Input invalid, only u8 accepted. ({})", err);
                    }
                }
            }
            board.close();
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
