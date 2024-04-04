use rev1::board::Board;
use libftd2xx::{list_devices, DeviceInfo};


fn main() {

    println!("Hello, world!");
    let devices: Vec<DeviceInfo> = list_devices().unwrap();
    for device in devices {
        println!("device properties: {:?}", device);
    }
    match Board::new() {
        Ok(mut board) => {
            let phases: Vec<f32> = vec![0.00, 0.79, 1.57, 2.36, 3.14, 3.97, 4.71, 5.49];
            board.set_frame(&phases);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
