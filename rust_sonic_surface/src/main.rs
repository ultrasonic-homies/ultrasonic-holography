// this file lets you send on and off messages to the arduino
// install rust
// rename this to main.rs
// cargo build
// ./target/debug/rust_sonic_surface

use std::io::{self, Write};
use serialport::available_ports;

fn list_serial_ports() -> Result<Vec<String>, serialport::Error> {
    println!("Available Serial Ports:");

    let mut port_names = Vec::new();

    let Ok(ports) = available_ports() else {
        eprintln!("Error listing serial ports");
        return Err(serialport::Error::new(serialport::ErrorKind::Unknown, "Error listing serial ports"));
    };

    for (index, port) in ports.iter().enumerate() {
        println!("{}: {}", index + 1, port.port_name);
        port_names.push(port.port_name.clone());
    }

    Ok(port_names)
}

fn choose_serial_port(port_names: &[String]) -> Option<String> {
    print!("Choose a serial port by entering its number: ");
    io::stdout().flush().unwrap();
 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let Ok(index) = input.trim().parse::<usize>() else {
        println!("Invalid choice. Please enter a valid number.");
        return None;
    };
    
    let name = port_names.get(index.checked_sub(1)?)?;
    Some(name.clone())
}


fn main() {
    // Define the messages as byte arrays
    let on_message: Vec<u8> = vec![
        0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x08, 0x1A, 0x00, 0x1C, 0x11, 0x0A, 0x04, 0x12, 0x18, 0x0C, 0x12, 0x08, 0x1D, 0x16, 0x11, 0x03, 0x1F, 0x13, 0x19, 0x0A, 0x1F, 0x18, 0x12, 
        0x0A, 0x1B, 0x0F, 0x15, 0x00, 0x16, 0x0F, 0x09, 0x06, 0x0D, 0x00, 0x06, 0x0D, 0x03, 0x1B, 0x15, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0x01, 0x1D, 0x0E, 0x0D, 0x07, 0x0B, 
        0x04, 0x15, 0x13, 0x0E, 0x1A, 0x1A, 0x13, 0x18, 0x15, 0x1C, 0x1A, 0x15, 0x1C, 0x1B, 0x14, 0x19, 0x1C, 0x18, 0x16, 0x11, 0x12, 0x12, 0x0B, 0x10, 
        0x18, 0x09, 0x07, 0x03, 0x1E, 0x1E, 0x18, 0x1C, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x15, 0x1F, 0x00, 0x00, 0x0C, 0x06, 0x00, 0x00, 0x01, 0x0B, 0x00, 0x00, 
        0x1C, 0x0D, 0x00, 0x00, 0x03, 0x0D, 0x00, 0x00, 0x03, 0x09, 0x00, 0x00, 0x1A, 0x04, 0x00, 0x00, 0x1F, 0x1B, 0x00, 0x00, 0x07, 0x10, 0x00, 0x00, 
        0x11, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFD
    ];
    let off_message: Vec<u8> = vec![
        0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 
        0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 
        0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFD
    ];

    // Open the serial connection
    let Ok(port_names) = list_serial_ports() else {
        eprintln!("Error: Unable to list serial ports.");
        return;
    };
    let Some(selected_port) = choose_serial_port(&port_names) else {
        eprintln!("Invalid selected port, exiting.");
        return;
    };
    println!("You selected serial port: {}", selected_port);

    let baud_rate = 230_400;

    let builder = serialport::new(selected_port.clone(), baud_rate);
    println!("{:?}", &builder);
    let mut port = builder.open().unwrap_or_else(|e| {
        eprintln!("Failed to open \"{}\". Error: {}", selected_port, e);
        ::std::process::exit(1);
    });

    let mut input = String::new();
    loop {
        print!("Press enter to send on message: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        }

        port.write_all(&on_message).unwrap();
        port.flush().unwrap();
        println!("On message sent successfully.");

        print!("Press enter to send off message: ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() != "" {
            break;
        }

        port.write_all(&off_message).unwrap();
        port.flush().unwrap();
        println!("Off message sent successfully.");
    }
}
