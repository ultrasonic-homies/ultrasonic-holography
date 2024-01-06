use test_lib::FPGA;
use std::time::Duration;
use std::thread;
use std::io::{self, Write};

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            match FPGA::new("FT7TEQ7VB", "async") {
                Ok(mut de0_cv) => {
                    let start_time = std::time::Instant::now();
                    for j in 0..64 {
                        for i in 0..=255 {
                            let a: u8 = 0;
                        // thread::sleep(Duration::from_millis(10));
                            // println!("Setting address {} with phase {}", a, i);
                            de0_cv.set_phase(a, 0x01*i, true).unwrap();
                            de1_soc.set_phase(a, 0x01*i, true).unwrap();
                        }
                    }
                    let exec_time = start_time.elapsed().as_secs_f64();
                    println!("32768 phases addressed in {} s", exec_time);
                    let mut board_id: u8 = 1;
                    let mut input = String::new();
                    let mut address: u8;
                    let mut phase: u8;
                    let mut enable: bool;
                    loop {
                        loop {
                            input.clear();
                            println!("Select a board: 1, 2");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            match input.trim().parse::<u8>() {
                                Ok(parsed_u8) => {
                                    if parsed_u8 == 1 || parsed_u8 == 2 {
                                        board_id = parsed_u8;
                                        break;
                                    }
                                    else {
                                        println!("Input invalid, must be in range [1, 2]");
                                    }
                                }
                                Err(err) => {
                                    println!("Input invalid, only u8 accepted. ({})", err);
                                }
                            }
                        }
                        loop {
                            input.clear();
                            println!("Select an address:");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            match input.trim().parse::<u8>() {
                                Ok(parsed_u8) => {
                                    address = parsed_u8;
                                    break;
                                }
                                Err(err) => {
                                    println!("Input invalid, only u8 accepted. ({})", err);
                                }
                            }
                        }
                        loop {
                            input.clear();
                            println!("Select a phase:");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            match input.trim().parse::<u8>() {
                                Ok(parsed_u8) => {
                                    phase = parsed_u8;
                                    break;
                                }
                                Err(err) => {
                                    println!("Input invalid, only u8 accepted. ({})", err);
                                }
                            }
                        }
                        loop {
                            input.clear();
                            println!("Enable? Y/N");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            match input.trim() {
                                "Y" => {
                                    enable = true;
                                    break;
                                }
                                "N" => {
                                    enable = false;
                                    break;
                                }
                                _ => {
                                    println!("Input invalid, must be Y or N.");
                                }
                            }
                        }
                        if board_id == 1 {
                            de1_soc.set_phase(address, phase, enable).unwrap();
                        }
                        else {
                            de0_cv.set_phase(address, phase, enable).unwrap();
                        }
                        println!("Setting board {} address {} with phase {}, enabled={}", board_id, address, phase, enable);
                    }
                }
                Err(device_type_error) => {
                    println!("Initialization failed for de0_cv with error: {}", device_type_error)
                }
            }
        }
        Err(device_type_error) => {
            println!("Initialization failed for de1_soc with error: {}", device_type_error)
        }
    }
}