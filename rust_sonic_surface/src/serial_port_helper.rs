use serialport::available_ports;
use std::io::prelude::*;
use std::io::Error;
use std::io::{self, Write};

pub fn list_serial_ports() -> Result<Vec<String>, serialport::Error> {
    println!("Available Serial Ports:");

    let mut port_names = Vec::new();

    let Ok(ports) = available_ports() else {
        eprintln!("Error listing serial ports");
        return Err(serialport::Error::new(
            serialport::ErrorKind::Unknown,
            "Error listing serial ports",
        ));
    };

    for (index, port) in ports.iter().enumerate() {
        println!("{}: {}", index + 1, port.port_name);
        port_names.push(port.port_name.clone());
    }

    Ok(port_names)
}

pub fn choose_serial_port(port_names: &[String]) -> Option<String> {
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
