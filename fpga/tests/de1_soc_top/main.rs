use test_lib::FPGA;

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            for i in 0..4 {
                de1_soc.set_phase(i, 0x40*i).unwrap();
            }
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)
        }
    }
}