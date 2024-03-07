use std::f32::consts::PI;

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

pub fn convert_to_sonic_surface_output(phases: &Vec<f32>) -> Vec<u8> {
    let mut ss_phases: Vec<f32> = vec![0.0; 256];

    for i in 0..ss_phases.len() {

        ss_phases[i] = sonic_surface_index(phases, i);
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
    let mut return_vec = vec![0xC0];
    return_vec.extend(ser_output);
    return_vec.push(0xFD);
    return return_vec
}

// note that phases should be 10x10 row-major order
fn sonic_surface_index(phases: &Vec<f32>, idx: usize) -> f32 {
    let idx = EMITTERS_ORDER[idx];
    // need to "pad" our 10x10 transducer array to be 16x16
    // transducers are populated such that the first 6 rows are empty
    // and the last 6 columns are empty

    let row = idx / 16;
    let column = idx % 16;

    // the first 6 rows are off and the last 6 columns are off
    if row < 6 || column >= 10 {
        return f32::NAN;
    }

    let row = row - 6;
    return phases[row * 10 + column];
}
