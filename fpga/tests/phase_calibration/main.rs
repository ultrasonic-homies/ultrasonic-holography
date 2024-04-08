extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::Print;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;
use rev1::fpga::FPGA;

fn main() {
    match FPGA::new("REV1CHB") {
        Ok(mut de1_soc) => {
            match FPGA::new("REV1CHA") {
                Ok(mut de0_cv) => {
                    let help_message = "1: trans-1\r\n2: trans+1\\rn3: phase-1\r\n4: phase+1\r\n 5:phase-10\r\n 6:phase+10\r\n q: quit\r\n";
                    let mut transducer: u8 = 0;
                    // note, this is not most up to date calibration, just the one used before getting our latest one
                    let mut phases: Vec<u8> = vec![29, 80, 31, 15, 70, 59, 75, 73, 64, 61, 91, 64, 30, 225, 51, 13, 16, 75, 76, 75, 53, 69, 23, 78, 81, 72, 18, 
                    32, 98, 102, 42, 95, 74, 86, 21, 51, 89, 38, 88, 26, 79, 20, 101, 106, 110, 10, 22, 68, 70, 10, 28, 19, 87, 68, 15, 14, 49, 39, 80, 59, 80, 19, 
                    59, 92, 178, 44, 75, 110, 67, 63, 24, 85, 28, 11, 79, 31, 93, 50, 24, 40, 79, 23, 75, 27, 106, 87, 74, 69, 93, 15, 21, 67, 11, 31, 13, 85, 86, 
                    14, 40, 103, 28, 14, 17, 67, 70, 15, 96, 94, 90, 0, 16, 53, 12, 21, 8, 79, 22, 21, 75, 86, 151, 83, 11, 52, 5, 72, 52, 19, 21, 206, 61, 5, 59, 
                    65, 95, 18, 76, 46, 22, 5, 32, 10, 95, 23, 25, 9, 20, 74, 93, 15, 25, 49, 11, 61, 14, 0, 25, 26, 41, 54, 83, 79, 12, 8, 13, 90, 65, 10, 26, 10, 
                    90, 90, 34, 12, 14, 19, 93, 90, 102, 34, 86, 23, 38, 76, 97, 43, 0, 80, 23, 82, 105, 64, 13, 45, 0, 98, 6, 6, 71, 77, 57, 24, 178, 8, 98, 45, 
                    17, 84, 75, 77, 35, 95, 96, 14, 48, 76, 66, 3, 85, 21, 80, 0, 40, 20, 32, 78, 97, 90, 82, 76, 20, 88, 79, 11, 78, 82, 23, 102, 29, 57, 75, 72, 
                    0, 0, 84, 19, 14, 69, 22, 14, 26, 43, 3, 22, 100, 55];
                    let zeros = vec![0.0; 128];
                    de1_soc.set_multi(&zeros, &(0..128).collect::<Vec<u8>>());
                    de0_cv.set_multi(&zeros, &(0..128).collect::<Vec<u8>>());
                    de1_soc.set_phase_calibration();
                    de0_cv.set_phase_calibration();               
                    // Workflow
                    let mut stdout = stdout();
                    //going into raw mode
                    enable_raw_mode().unwrap();

                    //clearing the screen, going to top left corner and printing welcoming message
                    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0), Print(help_message)).unwrap();

                    //key detection
                    loop {
                        //going to top left corner
                        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
                        //matching the key
                        match read().unwrap() {
                            //i think this speaks for itself
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('1'),
                                modifiers: KeyModifiers::NONE,
                                //clearing the screen and printing our message
                            }) => {
                                transducer = transducer.wrapping_sub(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('2'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                transducer = transducer.wrapping_add(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('3'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                phases[transducer as usize] = phases[transducer as usize].wrapping_sub(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('4'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                phases[transducer as usize] = phases[transducer as usize].wrapping_add(1);

                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('5'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                phases[transducer as usize] = phases[transducer as usize].wrapping_sub(10);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('6'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                phases[transducer as usize] = phases[transducer as usize].wrapping_add(10);

                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('q'),
                                modifiers: KeyModifiers::NONE,
                            }) => break,
                            _ => (),
                        }
                        execute!(stdout, Clear(ClearType::All), 
                            Print(help_message), 
                            Print("trans="), 
                            Print(transducer),
                            Print(" phase="), 
                            Print(phases[transducer as usize])
                        ).unwrap();
                        if transducer < 128 {
                            de1_soc.set_phase(transducer, phases[transducer as usize], true).unwrap();
                        }
                        else {
                            de0_cv.set_phase((transducer-128), phases[transducer as usize], true).unwrap();
                        }
                    }
                    //disabling raw mode
                    disable_raw_mode().unwrap();
                    println!("{:?}", phases);
                    // let mut board_id: u8 = 1;
                    // let mut input = String::new();
                    // let mut address: u8;
                    // let mut phase: u8;
                    // let mut enable: bool;
                    // let mut quit: bool = false;
                    // loop {
                    //     loop {
                    //         input.clear();
                    //         println!("Select a board: 1, 2, or 3 to calibrate, or 0 to quit");
                    //         io::stdout().flush().unwrap();
                    //         io::stdin().read_line(&mut input).unwrap();
                    //         match input.trim().parse::<u8>() {
                    //             Ok(parsed_u8) => {
                    //                 if parsed_u8 == 0 {
                    //                     quit = true;
                    //                     break;
                    //                 }
                    //                 else if parsed_u8 == 3 {
                    //                     de1_soc.set_phase_calibration().unwrap();
                    //                     de0_cv.set_phase_calibration().unwrap();
                    //                 }
                    //                 else if parsed_u8 == 1 || parsed_u8 == 2 {
                    //                     board_id = parsed_u8;
                    //                     break;
                    //                 }
                    //                 else {
                    //                     println!("Input invalid, must be in range [1, 2]");
                    //                 }
                    //             }
                    //             Err(err) => {
                    //                 println!("Input invalid, only u8 accepted. ({})", err);
                    //             }
                    //         }
                    //     }
                    //     if quit {
                    //         break;
                    //     }
                    //     loop {
                    //         input.clear();
                    //         println!("Select an address:");
                    //         io::stdout().flush().unwrap();
                    //         io::stdin().read_line(&mut input).unwrap();
                    //         match input.trim().parse::<u8>() {
                    //             Ok(parsed_u8) => {
                    //                 address = parsed_u8;
                    //                 break;
                    //             }
                    //             Err(err) => {
                    //                 println!("Input invalid, only u8 accepted. ({})", err);
                    //             }
                    //         }
                    //     }
                    //     loop {
                    //         input.clear();
                    //         println!("Select a phase:");
                    //         io::stdout().flush().unwrap();
                    //         io::stdin().read_line(&mut input).unwrap();
                    //         match input.trim().parse::<u8>() {
                    //             Ok(parsed_u8) => {
                    //                 phase = parsed_u8;
                    //                 break;
                    //             }
                    //             Err(err) => {
                    //                 println!("Input invalid, only u8 accepted. ({})", err);
                    //             }
                    //         }
                    //     }
                    //     loop {
                    //         input.clear();
                    //         println!("Enable? Y/N");
                    //         io::stdout().flush().unwrap();
                    //         io::stdin().read_line(&mut input).unwrap();
                    //         match input.trim() {
                    //             "Y" => {
                    //                 enable = true;
                    //                 break;
                    //             }
                    //             "N" => {
                    //                 enable = false;
                    //                 break;
                    //             }
                    //             _ => {
                    //                 println!("Input invalid, must be Y or N.");
                    //             }
                    //         }
                    //     }
                    //     if board_id == 1 {
                    //         de1_soc.set_phase(address, phase, enable).unwrap();
                    //     }
                    //     else {
                    //         de0_cv.set_phase(address, phase, enable).unwrap();
                    //     }
                    //     println!("Setting board {} address {} with phase {}, enabled={}", board_id, address, phase, enable);
                    // }
                    de1_soc.close().unwrap();
                    de0_cv.close().unwrap();
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