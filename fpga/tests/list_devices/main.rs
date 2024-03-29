use libftd2xx::{list_devices, DeviceInfo};

fn main() {
    let devices: Vec<DeviceInfo> = list_devices().unwrap();
    for device in devices {
        println!("device properties: {:?}", device);
    }
}