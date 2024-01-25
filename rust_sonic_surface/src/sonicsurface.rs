#![deny(clippy::all)]
#![forbid(unsafe_code)]

use eframe::egui;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use tokio::runtime::Runtime;
use ndarray_linalg::norm;
use std::io::prelude::*;
use std::io::Error;
use serialport::SerialPort;
use ndarray::{Array2, Array1, s};
use std::io::{self, Write};
use serialport::available_ports;

const N_EMMITERS: usize = 256;


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

#[derive(Debug)]
struct PositionPhases {
    position: [f64; 3],
    phases: [f64; N_EMMITERS],
}

impl PositionPhases {
    // Constructor method to create a new instance of PositionPhases
    fn new(position: [f64; 3], phases: [f64; N_EMMITERS]) -> PositionPhases {
        PositionPhases { position, phases }
    }
}

// let my_instance = RustStruct::new(position, phases);

// // Print the instance for demonstration purposes
// println!("{:?}", my_instance);


struct ControlApp {
    // Sender/Receiver for async notifications.
    tx: Sender<u32>,
    rx: Receiver<u32>,

    // Silly app state.
    value: u32,
    count: u32,
    position_phases: Vec<PositionPhases>,
    serial_conn: SerialPort,
}

impl ControlApp {
    fn new(serial_conn: SerialPort) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            value: 1,
            count: 0,
            position_phases: Vec::new(),
            serial_conn: serial_conn
        }
    }
}

struct SonicSurface {
    phase_divs: usize,
    n_emitters: usize,
    wavelength: f64,
    emitters_pos: Array2<f64>,
    emitters_order: Vec<usize>,
    serial_conn: Option<Box<dyn SerialPort>>,
    phases: Array2<f64>,
}

impl SonicSurface {
    const PHASE_DIVS: usize = 32;
    const N_EMMITERS: usize = 256;
    const WAVELENGTH: f64 = 0.00865;
    const EMITTERS_POS: [f64; Self::N_EMMITERS * 3] = [-0.07874993,0.0,0.08074993,-0.057749946,0,0.08074993,-0.047249947,0,0.08074993,-0.06824995,0,0.070249945,-0.07874993,0,0.070249915,-0.047249954,0,0.07024993,-0.057749953,0,0.07024993,-0.06824992,0,0.08074993,-0.07874993,0,0.05974994,-0.057749946,0,0.059749942,-0.047249947,0,0.059749942,-0.06824993,0,0.04924994,-0.07874993,0,0.04924994,-0.047249947,0,0.049249943,-0.057749946,0,0.049249943,-0.06824993,0,0.05974994,-0.07874993,0,0.03874995,-0.057749946,0,0.038749944,-0.047249947,0,0.038749944,-0.06824993,0,0.028249947,-0.07874993,0,0.028249947,-0.047249954,0,0.028249951,-0.057749953,0,0.028249951,-0.06824993,0,0.03874994,-0.07874993,0,0.017749948,-0.057749953,0,0.017749952,-0.047249954,0,0.017749952,-0.068249926,0,0.0072499607,-0.07874993,0,0.007249957,-0.04724995,0,0.007249959,-0.05774995,0,0.007249959,-0.06824993,0,0.017749948,-0.07874993,0,-0.0032500343,-0.05774995,0,-0.0032500399,-0.04724995,0,-0.0032500362,-0.06824993,0,-0.013750033,-0.07874993,0,-0.013750033,-0.04724995,0,-0.013750035,-0.05774995,0,-0.013750035,-0.06824993,0,-0.0032500343,-0.07874993,0,-0.024250032,-0.05774995,0,-0.024250034,-0.04724995,0,-0.024250034,-0.06824993,0,-0.03475002,-0.07874994,0,-0.034750026,-0.047249954,0,-0.03475003,-0.057749953,0,-0.03475003,-0.06824993,0,-0.024250032,-0.07874993,0,-0.045250025,-0.057749953,0,-0.04525003,-0.047249947,0,-0.04525002,-0.06824993,0,-0.055750016,-0.07874993,0,-0.055750024,-0.047249947,0,-0.05575002,-0.057749953,0,-0.055750027,-0.06824993,0,-0.045250017,-0.078749925,0,-0.06624998,-0.05774995,0,-0.06624999,-0.04724995,0,-0.06624999,-0.06824994,0,-0.076749995,-0.07874994,0,-0.07674998,-0.04724995,0,-0.07674999,-0.05774995,0,-0.07674999,-0.068249926,0,-0.06625,-0.03674995,0,0.08074994,-0.015749965,0,0.08074993,-0.005249969,0,0.08074993,-0.026249964,0,0.07024993,-0.03674996,0,0.07024994,-0.005249969,0,0.07024993,-0.015749965,0,0.07024993,-0.026249964,0,0.08074993,-0.03674995,0,0.05974994,-0.015749961,0,0.059749935,-0.0052499655,0,0.05974994,-0.02624996,0,0.04924994,-0.036749955,0,0.04924994,-0.0052499655,0,0.04924994,-0.015749961,0,0.04924994,-0.02624996,0,0.059749935,-0.036749955,0,0.038749944,-0.01574996,0,0.03874995,-0.0052499655,0,0.038749944,-0.026249964,0,0.028249947,-0.036749955,0,0.028249947,-0.0052499664,0,0.02824995,-0.015749965,0,0.028249947,-0.026249958,0,0.03874995,-0.036749955,0,0.017749948,-0.015749965,0,0.017749948,-0.0052499664,0,0.01774995,-0.026249962,0,0.007249958,-0.036749955,0,0.007249959,-0.0052499673,0,0.007249957,-0.015749963,0,0.007249958,-0.026249964,0,0.017749948,-0.036749955,0,-0.003250037,-0.015749961,0,-0.003250038,-0.005249968,0,-0.003250038,-0.026249958,0,-0.013750037,-0.036749955,0,-0.013750033,-0.005249967,0,-0.013750033,-0.01574996,0,-0.013750037,-0.02624996,0,-0.003250038,-0.036749955,0,-0.024250032,-0.01574996,0,-0.024250036,-0.0052499673,0,-0.024250032,-0.02624996,0,-0.034750026,-0.036749955,0,-0.034750026,-0.0052499673,0,-0.034750026,-0.015749961,0,-0.034750026,-0.026249958,0,-0.024250036,-0.036749955,0,-0.04525003,-0.015749963,0,-0.04525002,-0.0052499655,0,-0.04525002,-0.026249964,0,-0.05575002,-0.036749955,0,-0.055750027,-0.0052499655,0,-0.05575002,-0.015749965,0,-0.05575002,-0.026249962,0,-0.04525002,-0.03674995,0,-0.06624999,-0.015749963,0,-0.06624999,-0.005249969,0,-0.06624999,-0.026249962,0,-0.07674999,-0.03674996,0,-0.07674999,-0.005249969,0,-0.07674999,-0.015749963,0,-0.07674999,-0.026249962,0,-0.06624999,0.0052500297,0,0.08074993,0.026250018,0,0.08074993,0.036750015,0,0.08074993,0.015750019,0,0.07024993,0.0052500297,0,0.07024993,0.036750015,0,0.07024993,0.026250018,0,0.07024993,0.015750019,0,0.08074993,0.0052500297,0,0.05974994,0.026250016,0,0.059749935,0.036750015,0,0.05974994,0.015750017,0,0.049249936,0.0052500297,0,0.04924994,0.036750015,0,0.04924994,0.026250016,0,0.049249936,0.015750017,0,0.059749935,0.005250028,0,0.038749944,0.02625002,0,0.038749944,0.036750015,0,0.038749944,0.01575002,0,0.028249951,0.005250028,0,0.028249947,0.036750015,0,0.028249951,0.02625002,0,0.028249951,0.01575002,0,0.038749944,0.0052500293,0,0.017749952,0.02625002,0,0.017749952,0.036750015,0,0.017749952,0.01575002,0,0.007249958,0.0052500293,0,0.007249957,0.036750015,0,0.007249959,0.02625002,0,0.007249958,0.01575002,0,0.017749952,0.005250029,0,-0.003250037,0.02625002,0,-0.003250038,0.036750015,0,-0.003250038,0.01575002,0,-0.013750033,0.005250028,0,-0.013750033,0.036750015,0,-0.013750037,0.02625002,0,-0.013750033,0.01575002,0,-0.003250038,0.005250028,0,-0.024250032,0.02625002,0,-0.024250034,0.036750015,0,-0.024250036,0.015750019,0,-0.034750022,0.0052500297,0,-0.034750026,0.036750015,0,-0.034750026,0.026250018,0,-0.034750022,0.01575002,0,-0.024250034,0.0052500297,0,-0.04525002,0.026250018,0,-0.04525002,0.036750015,0,-0.045250017,0.015750019,0,-0.05575002,0.0052500297,0,-0.05575002,0.03675001,0,-0.05575002,0.026250018,0,-0.05575002,0.015750019,0,-0.04525002,0.0052500297,0,-0.06624999,0.02625002,0,-0.06624999,0.03675002,0,-0.06624998,0.01575002,0,-0.07674999,0.0052500297,0,-0.07674999,0.03675001,0,-0.07674998,0.02625002,0,-0.07674999,0.01575002,0,-0.06624999,0.047250018,0,0.08074993,0.068249986,0,0.08074993,0.078749985,0,0.08074993,0.05775,0,0.07024994,0.04725001,0,0.07024992,0.078749985,0,0.07024993,0.068249986,0,0.07024993,0.05775001,0,0.08074993,0.047250006,0,0.059749946,0.068249986,0,0.05974994,0.078749985,0,0.059749946,0.057750013,0,0.049249936,0.04725001,0,0.04924994,0.078749985,0,0.049249947,0.068249986,0,0.04924994,0.05775001,0,0.05974994,0.047250006,0,0.03874995,0.068249986,0,0.03874994,0.078749985,0,0.03874995,0.057750005,0,0.028249947,0.047250006,0,0.028249947,0.078749985,0,0.028249947,0.068249986,0,0.028249947,0.057750005,0,0.038749937,0.047250006,0,0.017749948,0.068249986,0,0.017749948,0.078749985,0,0.017749948,0.05775001,0,0.007249957,0.04725001,0,0.007249959,0.078749985,0,0.0072499607,0.068249986,0,0.007249955,0.057750005,0,0.017749948,0.04725001,0,-0.0032500362,0.068249986,0,-0.003250038,0.078749985,0,-0.003250038,0.057750005,0,-0.013750033,0.047250006,0,-0.013750033,0.078749985,0,-0.013750037,0.068249986,0,-0.013750037,0.05775001,0,-0.0032500362,0.047250006,0,-0.024250032,0.068249986,0,-0.024250036,0.078749985,0,-0.024250036,0.057750005,0,-0.034750022,0.047250006,0,-0.03475003,0.078749985,0,-0.034750026,0.068249986,0,-0.034750026,0.057750005,0,-0.024250032,0.047250014,0,-0.045250017,0.068249986,0,-0.045250017,0.078749985,0,-0.045250025,0.057750013,0,-0.055750016,0.047250014,0,-0.055750016,0.078749985,0,-0.055750024,0.068249986,0,-0.055750016,0.057750013,0,-0.045250017,0.047250018,0,-0.06625,0.06824999,0,-0.06625,0.07874999,0,-0.06624998,0.05775001,0,-0.07674999,0.04725001,0,-0.07674999,0.07874998,0,-0.07674998,0.06824998,0,-0.076749995,0.057750016,0,-0.06624998]
    const EMITTERS_ORDER: [u32; Self::N_EMMITERS] = [0, 7, 1, 2, 64, 71, 65, 66, 128, 135, 129, 130, 192, 199, 193, 194, 4, 3, 6, 5, 68, 67, 70, 69,
    132, 131, 134, 133, 196, 195, 198, 197, 8, 15, 9, 10, 72, 79, 73, 74, 136, 143, 137, 138, 200,
    207, 201, 202, 12, 11, 14, 13, 76, 75, 78, 77, 140, 139, 142, 141, 204, 203, 206, 205, 16, 23, 17,
    18, 80, 87, 81, 82, 144, 151, 145, 146, 208, 215, 209, 210, 20, 19, 22, 21, 84, 83, 86, 85, 148,
    147, 150, 149, 212, 211, 214, 213, 24, 31, 25, 26, 88, 95, 89, 90, 152, 159, 153, 154, 216, 223,
    217, 218, 28, 27, 30, 29, 92, 91, 94, 93, 156, 155, 158, 157, 220, 219, 222, 221, 32, 39, 33, 34,
    96, 103, 97, 98, 160, 167, 161, 162, 224, 231, 225, 226, 36, 35, 38, 37, 100, 99, 102, 101, 164,
    163, 166, 165, 228, 227, 230, 229, 40, 47, 41, 42, 104, 111, 105, 106, 168, 175, 169, 170, 232,
    239, 233, 234, 44, 43, 46, 45, 108, 107, 110, 109, 172, 171, 174, 173, 236, 235, 238, 237, 48, 55,
    49, 50, 112, 119, 113, 114, 176, 183, 177, 178, 240, 247, 241, 242, 52, 51, 54, 53, 116, 115, 118,
    117, 180, 179, 182, 181, 244, 243, 246, 245, 56, 63, 57, 58, 120, 127, 121, 122, 184, 191, 185,
    186, 248, 255, 249, 250, 60, 59, 62, 61, 124, 123, 126, 125, 188, 187, 190, 189, 252, 251, 254,
    253];

    fn new() -> Self {
        SonicSurface {
            serial_conn: None,
            phase_divs: Self::PHASE_DIVS,
            n_emitters: Self::N_EMMITERS,
            wavelength: Self::WAVELENGTH,
            emitters_order: Self::EMITTERS_ORDER.iter().map(|&val| val as usize).collect(),
            emitters_pos: Array2::from_shape_vec((Self::N_EMMITERS, 3), Vec::from(Self::EMITTERS_POS)).unwrap(),
            phases: Array2::zeros((1, Self::N_EMMITERS)),
        }
    }

    fn list_serial() {
        let ports = serialport::available_ports().expect("Error listing serial ports");
        println!("Serial Ports:");
        for (i, port) in ports.iter().enumerate() {
            println!("{}: {}", i + 1, port.port_name);
        }
    }

    fn connect(&mut self, port_name: &str, baud_rate: u32) {
        let builder = serialport::new(port_name, baud_rate);
        println!("{:?}", &builder);
        let mut port = builder.open().unwrap_or_else(|e| {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        });
        self.serial_conn = port;
    }

    fn send_phases(&mut self, phases: &Array1<f64>, permute_to_fpga_order: bool) -> Result<(), Error> {
        assert_eq!(phases.len(), self.n_emitters);

        let mut data_to_send = Array1::zeros(self.n_emitters);
        let deactivated = phases.map(|&val| val.is_nan());
        let order = if permute_to_fpga_order {
            &self.emitters_order
        } else {
            &(0..self.n_emitters).collect::<Vec<usize>>()
        };

        data_to_send[order] = phases.map(|&val| (val % (2.0 * std::f64::consts::PI)) * self.phase_divs as f64 / (2.0 * std::f64::consts::PI));
        data_to_send[deactivated] = self.phase_divs as f64;

        let mut data_to_send_bytes: Vec<u8> = data_to_send.map(|&val| val as u8).collect();
        data_to_send_bytes.insert(0, 254); // start phases
        data_to_send_bytes.push(253); // commit

        if let Some(conn) = &mut self.serial_conn {
            conn.write_all(&data_to_send_bytes)?;
        }

        Ok(())
    }
}



fn main() {
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

    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    // Run the GUI in the main thread.
    eframe::run_native(
        "Sonic Surface GUI",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(ControlApp::new(port))),
    );
}

impl eframe::App for ControlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the counter with the async response.
        if let Ok(current_pos_phase) = self.rx.try_recv() {
            self.position_phases = current_pos_phase;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("");
            ui.label("If successful, the count will increase by the following value.");
            ui.add(egui::Slider::new(&mut self.value, 1..=120).text("value"));

            if ui.button(format!("Count: {}", self.count)).clicked() {
                send_req(self.value, self.tx.clone(), ctx.clone());
            }
        });
    }
}

fn send_req(incr: u32, tx: Sender<u32>, ctx: egui::Context) {
    tokio::spawn(async move {        
        // Send a request with an increment value.
        let body: HttpbinJson = Client::default()
            .post("https://httpbin.org/anything")
            .json(&Body { incr })
            .send()
            .await
            .expect("Unable to send request")
            .json()
            .await
            .expect("Unable to parse response");

        // After parsing the response, notify the GUI thread of the increment value.
        let _ = tx.send(body.json.incr);
        ctx.request_repaint();
    });
}