use test_lib::FPGA;
use std::time::Duration;
use std::thread;
use std::io::{self, Write};

fn main() {
    match FPGA::new("FT7TEQ7VB", "async") {
        Ok(mut de1_soc) => {
            let enable: bool = true;
            for i in 0..=255 {
                let a: u8 = 0;
                thread::sleep(Duration::from_millis(10));
                println!("Setting address {} with phase {}", a, i);
                de1_soc.set_phase(a, 0x01*i, enable).unwrap();
            }
            let mut input = String::new();
            let mut address: u8;
            let mut phase: u8;
            loop {
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
                de1_soc.set_phase(address, phase, enable).unwrap();
                println!("Setting address {} with phase {}", address, phase);
            }
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)

        }
    }
}