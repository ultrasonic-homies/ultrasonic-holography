use rev1::fpga::FPGA;
use std::thread;
use std::io::{self, Write};

fn main() {
    match FPGA::new("FT7TEQ7VA") {
        Ok(mut de1_soc) => {
            match FPGA::new("FT7TEQ7VB") {
                Ok(mut de0_cv) => {
                    let num_writes: u32 = 32768;
                    // Test 1: Sequential
                    let start_time_seq = std::time::Instant::now();
                    de0_cv.set_phase_frame_buf(num_writes).unwrap();
                    de1_soc.set_phase_frame_buf(num_writes).unwrap();
                    let exec_time_seq = start_time_seq.elapsed().as_secs_f64();
                    println!("Sequential: 65536 phases addressed in {} s", exec_time_seq);
                    // Test 2: Parallel
                    let start_time_par = std::time::Instant::now();
                    let handle = thread::spawn(move || {
                        de0_cv.set_phase_frame_buf(num_writes).unwrap();
                    });
                    de1_soc.set_phase_frame_buf(num_writes).unwrap();
                    let exec_time_par = start_time_par.elapsed().as_secs_f64();
                    handle.join().unwrap();
                    println!("Parallel: 65536 phases addressed in {} s", exec_time_par);
                    // Command line input
                    let mut board_id: u8 = 1;
                    let mut input = String::new();
                    let mut address: u8;
                    let mut phase: u8;
                    let mut enable: bool;
                    let mut quit: bool = false;
                    loop {
                        loop {
                            input.clear();
                            println!("Select a board: 1, 2, or 0 to quit");
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            match input.trim().parse::<u8>() {
                                Ok(parsed_u8) => {
                                    if parsed_u8 == 0 {
                                        quit = true;
                                        break;
                                    }
                                    else if parsed_u8 == 1 || parsed_u8 == 2 {
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
                        if quit {
                            break;
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
                            // de0_cv.set_phase(address, phase, enable).unwrap();
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