use std::thread;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use serialport::available_ports;
use rev1::board::Board;

fn main() {
    let mut board = Board::new().unwrap();
    // sound stuff 
    let sample_rate = 50; // Sample rate (adjust as needed)
    let mut t= 0.0;
    let freq440 = 440.0; // Frequency of 440 Hz sine wave
    let freq40k = 40000.0; // Frequency of 40 kHz sine wave
    let dt = 1.0 / sample_rate as f32; // Time step for each sample
    let tick_interval = Duration::from_millis((dt * (1000.0 as f32)) as u64);// Interval for amplitude comparison (adjust as needed)
    
    let mut device_on = false; // Device state
    loop {

        let amp440 = (2.0 * std::f32::consts::PI * freq440 * t).sin(); // 440 Hz sine wave amplitude
        let amp40k = (2.0 * std::f32::consts::PI * freq40k * t).sin(); // 40 kHz sine wave amplitude

        /// Compare amplitudes and control device state
        if amp440 > amp40k {
            if !device_on {
                device_on = true;
                board.set_all_on();
            }
        } else {
            if device_on {
                device_on = false;
                board.set_all_off();
            }
        }

        t += dt;

        // Sleep to control loop frequency
        thread::sleep(tick_interval);
    }
}
