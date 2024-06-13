use rev1::board::Board;
use libftd2xx::{list_devices, DeviceInfo};
use std::io::{self, Write};

const NUM_TRANSDUCERS:usize = 256;

const HZ_C4: f32 = 261.6;
const HZ_D4: f32 = 293.6;
const HZ_E4: f32 = 329.6;
const HZ_F4: f32 = 349.2;
const HZ_G4: f32 = 392.0;
const HZ_A4: f32 = 440.0;
const HZ_B4: f32 = 493.9;
const HZ_C5: f32 = 523.2;

fn main() {

    println!("Hello, world!");
    let devices: Vec<DeviceInfo> = list_devices().unwrap();
    for device in devices {
        println!("device properties: {:?}", device);
    }
    match Board::new() {
        Ok(mut board) => {
            loop {
                let zero_frame: Vec<f32> = vec![0.0; NUM_TRANSDUCERS];
                let flip_frame: Vec<f32> = vec![1.6; NUM_TRANSDUCERS];
                let mut input = String::new();
                input.clear();
                // println!("1: Send Zeros\n2: Send 180 degrees\n3: Send Calibration Phases\n4: Calibrate\n0: Quit");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                match input.trim().parse::<u8>() {
                    Ok(parsed_u8) => match parsed_u8 {
                        0 => break,
                        // 1 => board.set_frame(&zero_frame),
                        // 2 => board.set_frame(&flip_frame),
                        // 3 => board.set_preset_calibration(),
                        // 4 => board.calibrate(),
                        // 5 => {
                        //     board.modulate(440.0, true);
                        // }
                        // 6 => {
                        //     board.modulate(220.0, true);
                        // }
                        // 7 => board.modulate(0.0, false),
                        1 => board.modulate_multi_test(0b0001, false, HZ_C4, true),
                        2 => board.modulate_multi_test(0b0010, false, HZ_D4, true),
                        3 => board.modulate_multi_test(0b0100, false, HZ_E4, true),
                        4 => board.modulate_multi_test(0b1000, false, HZ_F4, true),
                        5 => board.modulate_multi_test(0b0001, true, HZ_G4, true),
                        6 => board.modulate_multi_test(0b0010, true, HZ_A4, true),
                        7 => board.modulate_multi_test(0b0100, true, HZ_B4, true),
                        8 => board.modulate_multi_test(0b1000, true, HZ_C5, true),
                        9 => {
                            board.set_frame(&zero_frame);
                            // board.modulate(440.0, true);
                        },
                        10 => board.shut_up(),
                        11 => board.modulate_multi_test(0b0001, false, HZ_C4, false),
                        12 => board.modulate_multi_test(0b0010, false, HZ_D4, false),
                        13 => board.modulate_multi_test(0b0100, false, HZ_E4, false),
                        14 => board.modulate_multi_test(0b1000, false, HZ_F4, false),
                        15 => board.modulate_multi_test(0b0001, true, HZ_G4, false),
                        16 => board.modulate_multi_test(0b0010, true, HZ_A4, false),
                        17 => board.modulate_multi_test(0b0100, true, HZ_B4, false),
                        18 => board.modulate_multi_test(0b1000, true, HZ_C5, false),
                        20 => { // Disable modulation
                            board.modulate_multi_test(0b1111, false, 0.0, false);
                            board.modulate_multi_test(0b1111, true, 0.0, false);
                        },

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
