use crate::fpga::{FPGA, PHASE_CONV_FACTOR};
use std::error::Error;

// Dev Boards
// const FPGA_0_SERIAL: &str = "FT7TEQ7VA";
// const FPGA_1_SERIAL: &str = "FT7TEQ7VB";
// Rev 1 *PRIMARY = CHANNEL B*
const FPGA_0_SERIAL: &str = "REV1CHB";
const FPGA_1_SERIAL: &str = "REV1CHA";

const PHASE_CALIBRATION: [u8; 256] = [29, 80, 31, 15, 70, 59, 75, 73, 64, 61, 91, 64, 30, 225, 51, 13, 16, 75, 76, 75, 53, 69, 23, 78, 81, 72, 18, 32, 98, 102, 19, 95, 74, 86, 21, 51, 66, 23, 88, 26, 79, 20, 101, 76, 110, 10, 22, 68, 70, 10, 28, 19, 87, 40, 15, 14, 49, 39, 80, 59, 80, 19, 59, 92, 178, 44, 63, 77, 43, 63, 24, 
            85, 28, 11, 79, 31, 93, 50, 24, 40, 79, 23, 75, 27, 74, 66, 54, 69, 93, 15, 21, 67, 11, 31, 13, 85, 86, 14, 40, 72, 28, 14, 17, 67, 70, 15, 96, 67, 90, 0, 16, 53, 12, 21, 8, 79, 22, 21, 75, 86, 151, 83, 11, 52, 5, 72, 52, 19, 21, 206, 61, 5, 59, 65, 74, 18, 76, 46, 22, 5, 32, 10, 95, 23, 25, 9, 20, 44, 68, 15, 25, 49, 11, 61, 14, 0, 25, 26, 41, 54, 53, 79, 12, 8, 13, 61, 65, 10, 26, 10, 90, 90, 34, 6, 14, 19, 71, 78, 93, 19, 86, 23, 38, 76, 97, 43, 0, 80, 23, 82, 105, 64, 13, 45, 0, 80, 6, 6, 71, 77, 57, 24, 178, 8, 77, 45, 17, 84, 75, 77, 5, 71, 73, 14, 48, 76, 66, 3, 85, 
            21, 56, 0, 40, 20, 32, 49, 76, 61, 73, 52, 0, 88, 79, 11, 59, 62, 8, 102, 15, 42, 0, 5, 0, 0, 66, 6, 14, 69, 22, 14, 26, 28, 3, 22, 100, 55];
const NUM_TRANSDUCERS_PER_FPGA: usize = 128;
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
                        let order0: Vec<u8> = (0..(NUM_TRANSDUCERS_PER_FPGA as u8)).into_iter().collect::<Vec<u8>>();
                        let order1: Vec<u8> = (0..(NUM_TRANSDUCERS_PER_FPGA as u8)).into_iter().collect::<Vec<u8>>();
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

    /** set_frame_calibrated
     * Software calibrated with the PHASE_CONV_FACTOR vector
     */
    pub fn set_frame_soft_calibrated(&mut self, phases: &Vec<f32>) {
        let mut phases_calibrated: Vec<u8> = vec![0; 2*NUM_TRANSDUCERS_PER_FPGA];
        for i in 0..(2*NUM_TRANSDUCERS_PER_FPGA) {
            phases_calibrated[i] = ((phases[i] * PHASE_CONV_FACTOR).round() as u8).wrapping_add(PHASE_CALIBRATION[i]);
        }
        self.fpga0.set_multi_bytes(&phases_calibrated[0..self.order0.len()], &self.order0).expect(&format!("set_frame: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.set_multi_bytes(&phases_calibrated[self.order0.len()..self.order0.len()+self.order1.len()], &self.order1).expect(&format!("set_frame: write timed out for {}", FPGA_1_SERIAL));
    }

    /** set_frame_bytes
     * Enables all transducers and sets the transducer array to the specified phases
     * @param phases: Vector of discretized phases of values [0 255] in row-major order
     */
    pub fn set_frame_bytes(&mut self, phases: &Vec<u8>) {
        self.fpga0.set_multi_bytes(&phases[0..NUM_TRANSDUCERS_PER_FPGA], &self.order0).expect(&format!("set_frame_bytes: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.set_multi_bytes(&phases[(NUM_TRANSDUCERS_PER_FPGA)..(NUM_TRANSDUCERS_PER_FPGA * 2)], &self.order1).expect(&format!("set_frame_bytes: write timed out for {}", FPGA_1_SERIAL));
    }

    /** set_preset_calibration
     * Enables all transducers and sets them to the pre-determined calibration
     */
    pub fn set_preset_calibration(&mut self) {
        self.set_frame_bytes(&PHASE_CALIBRATION.to_vec());
    }

    pub fn close(&mut self) {
        self.fpga0.close().unwrap();
        self.fpga1.close().unwrap();
    }

}