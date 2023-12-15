use test_lib::FPGA;

const phase = 0x40;
const address = 0x01;

fn main() {
    match FPGA::new("FT7TEQ7VA", "sync") {
        Ok(mut de1_soc) => {
            match de1_soc.set_phase(phase, address) {
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