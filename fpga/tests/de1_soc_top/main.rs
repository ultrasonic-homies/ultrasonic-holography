use test_lib::FPGA;

const PHASE: u8 = 0x40;
const ADDRESS: u8 = 0x01;

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            match de1_soc.set_phase(PHASE, ADDRESS) {
                Ok(()) => {
                    println!("set_phase completed");
                }
                Err(timeout_error) => {
                    println!("set_phase failed with TimeoutError: {}", timeout_error);
                    let _ = de1_soc.close();
                    return
                }
            }
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)
        }
    }
}