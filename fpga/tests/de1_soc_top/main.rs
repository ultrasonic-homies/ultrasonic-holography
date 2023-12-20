use test_lib::FPGA;
use std::time::Duration;
use std::thread;

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            for i in 0..=255 {
                let address: u8 = 0;
                thread::sleep(Duration::from_millis(10));
                println!("Setting address {} with phase {}", address, i);
                de1_soc.set_phase(address, 0x01*i).unwrap();
            }
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)
        }
    }
}