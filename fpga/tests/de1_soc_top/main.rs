use test_lib::FPGA;
use std::time::Duration;
use std::thread;
use std::io::{self, Write};

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            // Speed Test
            let enable: bool = true;
            let num_writes: u32 = 32768;
            // Pre-optimization
            for _ in 0..3 {
                let start_time = std::time::Instant::now();
                for i in 0..num_writes {
                    de1_soc.set_phase((i % 4).try_into().unwrap(), (i % 256).try_into().unwrap(), enable).unwrap();
                }
                let exec_time = start_time.elapsed().as_secs_f64();
                println!("32768 phases addressed in {} s", exec_time);
            }
            // Optimization 1
            de1_soc.set_phase_multi(num_writes).unwrap();
            de1_soc.set_phase_multi(num_writes).unwrap();
            de1_soc.set_phase_multi(num_writes).unwrap();
            // Optimization 2
            de1_soc.set_phase_multi_v2(num_writes).unwrap();
            de1_soc.set_phase_multi_v2(num_writes).unwrap();
            de1_soc.set_phase_multi_v2(num_writes).unwrap();

            // Manual Input
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