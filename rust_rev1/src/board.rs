use crate::fpga::{FPGA, PHASE_CONV_FACTOR};
use std::error::Error;

// Dev Boards
// const FPGA_0_SERIAL: &str = "FT7TEQ7VA";
// const FPGA_1_SERIAL: &str = "FT7TEQ7VB";
// Rev 1 *PRIMARY = CHANNEL B*
const FPGA_0_SERIAL: &str = "REV1CHB";
const FPGA_1_SERIAL: &str = "REV1CHA";

//james original calibration
// const PHASE_CALIBRATION: [u8; 256] = [29, 80, 31, 15, 70, 59, 75, 73, 64, 61, 91, 64, 30, 225, 51, 13, 16, 75, 76, 75, 53, 69, 23, 78, 81, 72, 18, 32, 98, 102, 19, 95, 74, 86, 21, 51, 66, 23, 88, 26, 79, 20, 101, 76, 110, 10, 22, 68, 70, 10, 28, 19, 87, 40, 15, 14, 49, 39, 80, 59, 80, 19, 59, 92, 178, 44, 63, 77, 43, 63, 24, 
//             85, 28, 11, 79, 31, 93, 50, 24, 40, 79, 23, 75, 27, 74, 66, 54, 69, 93, 15, 21, 67, 11, 31, 13, 85, 86, 14, 40, 72, 28, 14, 17, 67, 70, 15, 96, 67, 90, 0, 16, 53, 12, 21, 8, 79, 22, 21, 75, 86, 151, 83, 11, 52, 5, 72, 52, 19, 21, 206, 61, 5, 59, 65, 74, 18, 76, 46, 22, 5, 32, 10, 95, 23, 25, 9, 20, 44, 68, 15, 25, 49, 11, 61, 14, 0, 25, 26, 41, 54, 53, 79, 12, 8, 13, 61, 65, 10, 26, 10, 90, 90, 34, 6, 14, 19, 71, 78, 93, 19, 86, 23, 38, 76, 97, 43, 0, 80, 23, 82, 105, 64, 13, 45, 0, 80, 6, 6, 71, 77, 57, 24, 178, 8, 77, 45, 17, 84, 75, 77, 5, 71, 73, 14, 48, 76, 66, 3, 85, 
//             21, 56, 0, 40, 20, 32, 49, 76, 61, 73, 52, 0, 88, 79, 11, 59, 62, 8, 102, 15, 42, 0, 5, 0, 0, 66, 6, 14, 69, 22, 14, 26, 28, 3, 22, 100, 55];
// kevin amended
// const PHASE_CALIBRATION: [u8; 256] = [29, 80, 31, 15, 70, 59, 75, 73, 64, 61, 91, 64, 30, 225, 51, 13, 16, 75, 76, 75, 53, 69, 23, 78, 81, 72, 18, 
// 32, 98, 102, 42, 95, 74, 86, 21, 51, 89, 38, 88, 26, 79, 20, 101, 106, 110, 10, 22, 68, 70, 10, 28, 19, 87, 68, 15, 14, 49, 39, 80, 59, 80, 19, 
// 59, 92, 178, 44, 75, 110, 67, 63, 24, 85, 28, 11, 79, 31, 93, 50, 24, 40, 79, 23, 75, 27, 106, 87, 74, 69, 93, 15, 21, 67, 11, 31, 13, 85, 86, 
// 14, 40, 103, 28, 14, 17, 67, 70, 15, 96, 94, 90, 0, 16, 53, 12, 21, 8, 79, 22, 21, 75, 86, 151, 83, 11, 52, 5, 72, 52, 19, 21, 206, 61, 5, 59, 
// 65, 95, 18, 76, 46, 22, 5, 32, 10, 95, 23, 25, 9, 20, 74, 93, 15, 25, 49, 11, 61, 14, 0, 25, 26, 41, 54, 83, 79, 12, 8, 13, 90, 65, 10, 26, 10, 
// 90, 90, 34, 12, 14, 19, 93, 90, 102, 34, 86, 23, 38, 76, 97, 43, 0, 80, 23, 82, 105, 64, 13, 45, 0, 98, 6, 6, 71, 77, 57, 24, 178, 8, 98, 45, 
// 17, 84, 75, 77, 35, 95, 96, 14, 48, 76, 66, 3, 85, 21, 80, 0, 40, 20, 32, 78, 97, 90, 82, 76, 20, 88, 79, 11, 78, 82, 23, 102, 29, 57, 75, 72, 
// 0, 0, 84, 19, 14, 69, 22, 14, 26, 43, 3, 22, 100, 55];
const PHASE_CALIBRATION: [u8; 256] = [69, 90, 69, 64, 83, 76, 82, 85, 69, 70, 90, 68, 57, 213, 55, 49, 58, 83, 82, 82, 53, 76, 71, 86, 87, 76, 58, 
68, 92, 101, 56, 90, 75, 92, 57, 69, 75, 63, 96, 62, 87, 60, 99, 77, 136, 58, 61, 77, 71, 50, 67, 59, 99, 68, 58, 59, 68, 69, 85, 74, 90, 62, 74, 
109, 197, 64, 75, 89, 67, 63, 62, 89, 73, 55, 95, 63, 102, 70, 54, 76, 86, 67, 87, 63, 81, 80, 74, 82, 95, 58, 58, 79, 54, 67, 59, 86, 87, 61, 69, 
83, 61, 56, 66, 83, 78, 59, 113, 77, 99, 58, 63, 77, 57, 60, 55, 86, 67, 61, 86, 93, 190, 90, 60, 67, 55, 77, 67, 61, 54, 209, 78, 55, 67, 77, 78, 
61, 77, 67, 60, 55, 22, 57, 107, 60, 55, 49, 60, 74, 76, 52, 63, 69, 52, 68, 57, 0, 59, 56, 59, 67, 70, 81, 54, 49, 54, 82, 67, 48, 61, 49, 87, 92, 
60, 56, 53, 58, 81, 87, 102, 54, 82, 51, 64, 79, 98, 64, 52, 80, 61, 78, 135, 76, 53, 61, 58, 80, 52, 54, 80, 83, 67, 63, 193, 55, 82, 65, 58, 84, 
79, 74, 55, 75, 83, 53, 65, 83, 67, 53, 79, 59, 69, 52, 62, 59, 62, 64, 80, 69, 76, 74, 50, 86, 80, 55, 68, 66, 52, 108, 56, 63, 79, 73, 0, 0, 76, 
57, 56, 77, 59, 58, 66, 68, 56, 68, 111, 69];
const NUM_TRANSDUCERS_PER_FPGA: usize = 128;
const CARRIER_FREQ: f32 = 5_120_000.0;

const CUTOFF_FREQ_HZ: [f32; 8] = [
    130.8, // C3
    164.8, // E3
    196.0, // G3
    261.6, // C4
    329.6, // E4
    392.0, // G4
    466.2, // Bb4
    523.2, // C5
];

// Modulation channels are encoded in one-hot
const ALL_MOD_CHANNELS: u8 = 0b1111;

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

    pub fn set_all_zero_phases(&mut self) {
        self.set_frame(&vec![0.0; 2*NUM_TRANSDUCERS_PER_FPGA]);
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

    pub fn calibrate(&mut self) {
        self.fpga0.set_phase_calibration().expect(&format!("calibrate: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.set_phase_calibration().expect(&format!("calibrate: write timed out for {}", FPGA_1_SERIAL));
        self.set_frame_bytes(&vec![0; NUM_TRANSDUCERS_PER_FPGA * 2]);
    }

    pub fn clear_calibration(&mut self) {
        self.set_frame_bytes(&vec![0; NUM_TRANSDUCERS_PER_FPGA * 2]);
        self.fpga0.set_phase_calibration().expect(&format!("calibrate: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.set_phase_calibration().expect(&format!("calibrate: write timed out for {}", FPGA_1_SERIAL));
    }

    pub fn modulate(&mut self, freq:f32, enable: bool) {
        let half_period: u16 = (CARRIER_FREQ / freq / 2.0).round() as u16;
        self.fpga0.modulate(ALL_MOD_CHANNELS, half_period, enable).expect(&format!("modulate: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.modulate(ALL_MOD_CHANNELS, half_period, enable).expect(&format!("modulate: write timed out for {}", FPGA_1_SERIAL));
    }

    /** modulate_two_notes
     * set the modulation of each half of the board
     */
    pub fn modulate_two_notes(&mut self, freq_0:u32, freq_1:u32, enable: bool) {
        let half_period_0: u16 = (CARRIER_FREQ / freq_0 as f32 / 2.0).round() as u16;
        let half_period_1: u16 = (CARRIER_FREQ / freq_1 as f32 / 2.0).round() as u16;
        self.fpga0.modulate(ALL_MOD_CHANNELS, half_period_0, enable).expect(&format!("modulate_two_notes: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.modulate(ALL_MOD_CHANNELS, half_period_1, enable).expect(&format!("modulate_two_notes: write timed out for {}", FPGA_1_SERIAL));
    }

    /** modulate_two_boards
     * set the modulation of one half of the board, whether the frequency is above or below C4
     */
    pub fn modulate_two_boards(&mut self, freq:f32, enable: bool) {
        let period: u16 = (CARRIER_FREQ / freq as f32 / 2.0).round() as u16;

        if freq < 261.1 {
            self.fpga0.modulate(ALL_MOD_CHANNELS, period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_0_SERIAL));
        } else {
            self.fpga1.modulate(ALL_MOD_CHANNELS, period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_1_SERIAL));
        }
    }

    /** modulate_multi_notes
     * set the modulation of a single channel depending on the frequency
     */
    pub fn modulate_multi_notes(&mut self, freq:f32, enable: bool) {
        let period: u16 = (CARRIER_FREQ / freq as f32 / 2.0).round() as u16;
        for i in 0..CUTOFF_FREQ_HZ.len() {
            if freq > CUTOFF_FREQ_HZ[i] {
                /*
                    i == 0 -> set fpga0, channel 1
                    i == 1 -> set fpga1, channel 1
                    i == 2 -> set fpga0, channel 2
                    etc.
                */
                if i % 2 == 0 {
                    self.fpga0.modulate(1 << (i / 2), period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_0_SERIAL));
                }
                else {
                    self.fpga1.modulate(1 << (i / 2), period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_1_SERIAL));
                }
                break;
            }
        }
    }

    pub fn modulate_multi_test(&mut self, channel: u8, fpga: bool, freq: f32, enable: bool) {
        let period: u16 = (CARRIER_FREQ / freq as f32 / 2.0).round() as u16;
        if fpga {
            self.fpga0.modulate(channel, period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_0_SERIAL));
        } else {
            self.fpga1.modulate(channel, period, enable).expect(&format!("modulate_two_boards: write timed out for {}", FPGA_1_SERIAL));
        }
    }

    pub fn shut_up(&mut self) {
        self.fpga0.modulate(ALL_MOD_CHANNELS, 0, true).expect(&format!("shut_up: write timed out for {}", FPGA_0_SERIAL));
        self.fpga1.modulate(ALL_MOD_CHANNELS, 0, true).expect(&format!("shut_up: write timed out for {}", FPGA_1_SERIAL));
    }

    pub fn close(&mut self) {
        self.fpga0.close().unwrap();
        self.fpga1.close().unwrap();
    }

}