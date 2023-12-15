use test_lib::{FPGA, MIB};

fn main() {
    match FPGA::new("FT7TEQ7VA", "async") {
        Ok(mut de1_soc) => {
            match de1_soc.test_led() {
                Ok(()) => {
                    println!("test_led completed");
                }
                Err(timeout_error) => {
                    println!("test_led failed with TimeoutError: {}", timeout_error);
                    let _ = de1_soc.close();
                    return
                }
            }
            // match de1_soc.test_read(Some(10 * MIB)) {
            //     Ok(()) => {
            //         println!("test_read completed");
            //     }
            //     Err(timeout_error) => {
            //         println!("test_read failed with TimeoutError: {}", timeout_error);
            //         let _ = de1_soc.close();
            //         return
            //     }
            // }
            match de1_soc.test_write(Some(10 * MIB)) {
                Ok(()) => {
                    println!("test_write completed");
                }
                Err(timeout_error) => {
                    println!("test_write failed with TimeoutError: {}", timeout_error);
                    let _ = de1_soc.close();
                    return
                }
            }
            let _ = de1_soc.close();
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)
        }
    }
}
