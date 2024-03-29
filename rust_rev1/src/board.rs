use crate::fpga::FPGA;
use std::error::Error;

// Dev Boards
// const FPGA_0_SERIAL: &str = "FT7TEQ7VA";
// const FPGA_1_SERIAL: &str = "FT7TEQ7VB";
// Rev 1 *PRIMARY = CHANNEL B*
const FPGA_0_SERIAL: &str = "REV1CHB";
const FPGA_1_SERIAL: &str = "REV1CHA";

pub struct Board {
    fpga0: FPGA,
    fpga1: FPGA,
    order0: Vec<u8>,
    order1: Vec<u8>,
}

impl Board {

    /** new
     * Initializes the Board object with two FPGA objects, and specifies
     * the mapping between row-major index and transducer address
     * @Ok new Board object
     * @Err error if either FPGA does not initialize correctly
     */
    pub fn new() -> Result<Self, Box<dyn Error>> {
        match FPGA::new(FPGA_0_SERIAL) {
            Ok(fpga0) => {
                match FPGA::new(FPGA_1_SERIAL) {
                    Ok(fpga1) => {

                        // Map the index of the solver phase vector to transducer address
                        // Dev Boards
                        // let order0: Vec<u8> = (0..4).into_iter().collect::<Vec<u8>>();
                        // let order1: Vec<u8> = (0..4).into_iter().collect::<Vec<u8>>();
                        // Rev 1
                        let order0: Vec<u8> = (0..128).into_iter().collect::<Vec<u8>>();
                        let order1: Vec<u8> = (0..128).into_iter().collect::<Vec<u8>>();
                        let board = Board {
                            fpga0,
                            fpga1,
                            order0,
                            order1,
                        };
                        Ok(board)
                    }
                    Err(device_type_error) => {
                        return Err(format!("Initialization failed for {} with error: {}", FPGA_1_SERIAL, device_type_error).into());
                    }
                }
            }
            Err(device_type_error) => {
                return Err(format!("Initialization failed for {} with error: {}", FPGA_0_SERIAL, device_type_error).into());
            }
        }

    }

    /** set_frame
     * Enables all transducers and sets the transducer array to the specified phases
     * @param phases: Vector of phases of values [0 2pi] in row-major order
     */
    pub fn set_frame(&mut self, phases: &Vec<f32>) {
        self.fpga0.set_multi(&phases[0..self.order0.len()], &self.order0).expect(&format!("set_frame: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.set_multi(&phases[self.order0.len()..self.order0.len()+self.order1.len()], &self.order1).expect(&format!("set_frame: write timed out for {}", FPGA_1_SERIAL));
    }
}