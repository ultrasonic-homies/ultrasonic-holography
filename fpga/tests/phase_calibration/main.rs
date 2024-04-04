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
                    let help_message = "1: phase--\n2: phase++\n3: prev transducer\n4: next transducer\nq: quit\n";
                    let mut transducer: u8 = 0;
                    let mut phases: Vec<u8> = vec![0; 256];
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
                                phases[transducer as usize] = phases[transducer as usize].wrapping_sub(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('2'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                phases[transducer as usize] = phases[transducer as usize].wrapping_add(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('3'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                transducer = transducer.wrapping_sub(1);
                            },
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('4'),
                                modifiers: KeyModifiers::NONE,
                            }) => {
                                transducer = transducer.wrapping_add(1);
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