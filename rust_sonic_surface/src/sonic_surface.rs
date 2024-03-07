use std::f32::consts::PI;

use eframe::egui::mutex::RwLock;

use crate::hat::Point;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

const PHASE_DIVS: u32 = 32;
const N_EMMITERS: u32 = 256;
const EMITTERS_ORDER: [usize; 256] = [
    0, 7, 1, 2, 64, 71, 65, 66, 128, 135, 129, 130, 192, 199, 193, 194, 4, 3, 6, 5, 68, 67, 70, 69,
    132, 131, 134, 133, 196, 195, 198, 197, 8, 15, 9, 10, 72, 79, 73, 74, 136, 143, 137, 138, 200,
    207, 201, 202, 12, 11, 14, 13, 76, 75, 78, 77, 140, 139, 142, 141, 204, 203, 206, 205, 16, 23,
    17, 18, 80, 87, 81, 82, 144, 151, 145, 146, 208, 215, 209, 210, 20, 19, 22, 21, 84, 83, 86, 85,
    148, 147, 150, 149, 212, 211, 214, 213, 24, 31, 25, 26, 88, 95, 89, 90, 152, 159, 153, 154,
    216, 223, 217, 218, 28, 27, 30, 29, 92, 91, 94, 93, 156, 155, 158, 157, 220, 219, 222, 221, 32,
    39, 33, 34, 96, 103, 97, 98, 160, 167, 161, 162, 224, 231, 225, 226, 36, 35, 38, 37, 100, 99,
    102, 101, 164, 163, 166, 165, 228, 227, 230, 229, 40, 47, 41, 42, 104, 111, 105, 106, 168, 175,
    169, 170, 232, 239, 233, 234, 44, 43, 46, 45, 108, 107, 110, 109, 172, 171, 174, 173, 236, 235,
    238, 237, 48, 55, 49, 50, 112, 119, 113, 114, 176, 183, 177, 178, 240, 247, 241, 242, 52, 51,
    54, 53, 116, 115, 118, 117, 180, 179, 182, 181, 244, 243, 246, 245, 56, 63, 57, 58, 120, 127,
    121, 122, 184, 191, 185, 186, 248, 255, 249, 250, 60, 59, 62, 61, 124, 123, 126, 125, 188, 187,
    190, 189, 252, 251, 254, 253,
];

pub enum SendMessage {
    Stop, // stop the particle in-place
    Off,
    Move {
        points: Vec<Vec<Point>>,
        t_sep: Duration,
    },
}

pub struct SonicSurface {
    tx: Sender<SendMessage>,
    rx: Receiver<SendMessage>,
    position: RwLock<Vec<Point>>,
}

impl SonicSurface {
    pub fn new() -> SonicSurface {
        let (tx, rx) = channel::<SendMessage>();
        SonicSurface {
            tx,
            rx,
            position: RwLock::new(vec![]),
        }
    }

    pub fn get_tx(&self) -> Sender<SendMessage> {
        return self.tx.clone();
    }
}

pub fn convert_to_sonic_surface_output(phases: &Vec<f32>) -> Vec<u8> {
    let mut ss_phases: Vec<f32> = vec![0.0; 256];

    for i in 0..ss_phases.len() {
        ss_phases[EMITTERS_ORDER[i]] = sonic_surface_index(phases, i);
    }
    let ser_output: Vec<u8> = ss_phases
        .iter()
        .map(|p| {
            if p.is_nan() {
                // a value of PHASE_DIVS indicates an off transducer
                PHASE_DIVS as u8
            } else {
                (p / (2.0 * PI) * PHASE_DIVS as f32) as u8
            }
        })
        .collect();

    // add 0xC0 to start and 0xFD to the end
    let mut return_vec = vec![0xFE];
    return_vec.extend(ser_output);
    return_vec.push(0xFD);
    return return_vec;
}

// note that phases should be 10x10 row-major order
fn sonic_surface_index(phases: &Vec<f32>, idx: usize) -> f32 {
    // need to "pad" our 10x10 transducer array to be 16x16
    // transducers are populated such that the first 6 rows are empty
    // and the last 6 columns are empty
    // let idx = EMITTERS_ORDER[idx];

    let row = idx / 16;
    let column = idx % 16;

    // the first 6 rows are off and the last 6 columns are off
    if row < 6 || column >= 10 {
        return f32::NAN;
    }

    let row = row - 6;
    return phases[row * 10 + column];
}

#[cfg(test)]
mod tests {
    use crate::sonic_surface::convert_to_sonic_surface_output;

    #[test]
    fn check_convert() {
        let truth_vec: Vec<u8> = vec![
            0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x1A, 0x00,
            0x1C, 0x11, 0x0A, 0x04, 0x12, 0x18, 0x0C, 0x12, 0x08, 0x1D, 0x16, 0x11, 0x03, 0x1F,
            0x13, 0x19, 0x0A, 0x1F, 0x18, 0x12, 0x0A, 0x1B, 0x0F, 0x15, 0x00, 0x16, 0x0F, 0x09,
            0x06, 0x0D, 0x00, 0x06, 0x0D, 0x03, 0x1B, 0x15, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x01, 0x1D, 0x0E, 0x0D, 0x07, 0x0B, 0x04, 0x15,
            0x13, 0x0E, 0x1A, 0x1A, 0x13, 0x18, 0x15, 0x1C, 0x1A, 0x15, 0x1C, 0x1B, 0x14, 0x19,
            0x1C, 0x18, 0x16, 0x11, 0x12, 0x12, 0x0B, 0x10, 0x18, 0x09, 0x07, 0x03, 0x1E, 0x1E,
            0x18, 0x1C, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15,
            0x00, 0x00, 0x15, 0x1F, 0x00, 0x00, 0x0C, 0x06, 0x00, 0x00, 0x01, 0x0B, 0x00, 0x00,
            0x1C, 0x0D, 0x00, 0x00, 0x03, 0x0D, 0x00, 0x00, 0x03, 0x09, 0x00, 0x00, 0x1A, 0x04,
            0x00, 0x00, 0x1F, 0x1B, 0x00, 0x00, 0x07, 0x10, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0xFD,
        ];
        let test_vec: Vec<f32> = vec![1.0; 100];

        let out = convert_to_sonic_surface_output(&test_vec);
        println!("{:?}", out);
        let out_bool: Vec<bool> = out.iter().map(|&i| i != 32).collect();
        let truth_bool: Vec<bool> = truth_vec.iter().map(|&i| i != 0).collect();

        println!(
            "{:?}",
            out_bool
                .iter()
                .zip(&truth_bool)
                .map(|(&o, &t)| o != t)
                .collect::<Vec<bool>>()
        );
        assert_eq!(out_bool, truth_bool);
    }
}
