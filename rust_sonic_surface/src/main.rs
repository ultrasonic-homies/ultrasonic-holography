#![deny(clippy::all)]
#![forbid(unsafe_code)]
mod hat;
mod sonic_surface;

use eframe::egui;
use ndarray::{s, Array1, Array2};
use ndarray_linalg::norm;
use redis::Commands;
use reqwest::Client;
use serde_derive::{Deserialize, Serialize};
use serialport::available_ports;
use serialport::SerialPort;
use std::f32::consts::PI;
use std::io::prelude::*;
use std::io::Error;
use std::io::{self, Write};
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::time::Instant;
use std::{thread, time, vec};
use tokio::runtime::Runtime;

use hat::{HatRunner, Point};
use sonic_surface::convert_to_sonic_surface_phases;

const N_EMMITERS: usize = 256;
// IMPORTANT: measure height of board and update this constant before running
const Z_HEIGHT: f32 = 0.24; // m

fn list_serial_ports() -> Result<Vec<String>, serialport::Error> {
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
    positions: Vec<Point>,
    phases: Vec<f32>,
}

impl PositionPhases {
    // Constructor method to create a new instance of PositionPhases
    fn new(positions: Vec<Point>, phases: Vec<f32>) -> PositionPhases {
        PositionPhases { positions, phases }
    }
}

struct ControlGUI {
    // Sender/Receiver for async notifications.
    tx: Sender<PositionPhases>,
    rx: Receiver<PositionPhases>,

    // Silly app state.
    value: u32,
    count: u32,
    position_phases: PositionPhases,
    serial_conn: Box<dyn SerialPort>,
    redis_conn: redis::Connection,
    looping: bool,
}

impl ControlGUI {
    fn new(serial_conn: Box<dyn SerialPort>, redis_con: redis::Connection) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            value: 1,
            count: 0,
            position_phases: PositionPhases::new(vec![Point::new(0.0, 0.0, 0.0)], vec![0.0; 256]),
            serial_conn: serial_conn,
            redis_conn: redis_con,
            looping: false,
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

fn main() {
    // build serial port to send to sonic surface
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

    // build redis connection to send to blender
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis");
    let mut redis_con = client
        .get_connection()
        .expect("Failed to establish connection");

    // build tokio runtime for egui
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
        Box::new(|_cc| Box::new(ControlGUI::new(port, redis_con))),
    );
}

impl eframe::App for ControlGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update the blender sim with the current position over redis
        if let Ok(current_pos_phase) = self.rx.try_recv() {
            self.position_phases = current_pos_phase;
            let _: () = self
                .redis_conn
                .publish(
                    "positions",
                    format!("{:?}", self.position_phases.positions[0].print()),
                )
                .unwrap();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("");
            ui.label("Howdy");
            // ui.add(egui::Slider::new(&mut self.value, 1..=120).text("value"));
            ui.label(format!(
                "Current position: {}",
                self.position_phases.positions[0].print()
            ));
            // checkbox for looping
            ui.checkbox(&mut self.looping, "Looping");
            if ui.button("Test turn on").clicked() {
                test_turn_on(&mut self.serial_conn, self.tx.clone(), ctx.clone());
            }
            if ui.button("Test turn off").clicked() {
                test_turn_off(&mut self.serial_conn, self.tx.clone(), ctx.clone());
            }
            if ui.button("Starting circling").clicked() {
                send_req(
                    self.serial_conn.try_clone().unwrap(), // TODO: cloning the serial connection is a bad idea
                    "circle".to_owned(),
                    self.tx.clone(),
                    ctx.clone(),
                    self.looping,
                );
            }
        });
    }
}

fn send_req(
    mut serial_conn: Box<dyn SerialPort>, // need to take ownership of the serial connection for the thread
    mode: String,
    tx: Sender<PositionPhases>,
    ctx: egui::Context,
    looping: bool,
) {
    tokio::spawn(async move {
        if mode == "circle" {
            // generate control points for circling
            let radius = 0.02;
            let initial_x = 0.05;
            let initial_y = 0.05;
            let initial_z = 0.14;

            let freq: f32 = 1.0;
            let t_sep: f32 = 0.01;
            let num_t = (freq / t_sep).round() as u32;
            let ts: Vec<f32> = (0..num_t).map(|t| t as f32 * t_sep).collect();
            let xs: Vec<f32> = ts
                .iter()
                .map(|t| radius * (2.0 * PI * freq * t).sin() + initial_x)
                .collect();
            let ys: Vec<f32> = ts
                .iter()
                .map(|t| radius * (2.0 * PI * freq * t).sin() + initial_y)
                .collect();
            let zs: Vec<f32> = ts
                .iter()
                .map(|t| radius * (2.0 * PI * freq * t).sin() + initial_z)
                .collect();

            let cps: Vec<Vec<Point>> = xs
                .iter()
                .zip(ys.iter())
                .zip(zs.iter())
                .map(|((x, y), z)| {
                    vec![Point {
                        x: *x,
                        y: *y,
                        z: *z,
                    }]
                })
                .collect();

            let runner = HatRunner::new(32.0, Z_HEIGHT);
            println!("Starting circling");
            loop {
                run_control_points(&cps, t_sep, &runner, &mut serial_conn, tx.clone());
            }
        }
        // // Send a request with an increment value.
        // let body: HttpbinJson = Client::default()
        //     .post("https://httpbin.org/anything")
        //     .json(&Body { incr })
        //     .send()
        //     .await
        //     .expect("Unable to send request")
        //     .json()
        //     .await
        //     .expect("Unable to parse response");

        // // After parsing the response, notify the GUI thread of the increment value.
        // let _ = tx.send(body.json.incr);
        ctx.request_repaint();
    });
}

fn test_turn_on(
    serial_conn: &mut Box<dyn SerialPort>,
    tx: Sender<PositionPhases>,
    ctx: egui::Context,
) {
    let on_message: Vec<u8> = vec![
        0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x1A, 0x00, 0x1C, 0x11,
        0x0A, 0x04, 0x12, 0x18, 0x0C, 0x12, 0x08, 0x1D, 0x16, 0x11, 0x03, 0x1F, 0x13, 0x19, 0x0A,
        0x1F, 0x18, 0x12, 0x0A, 0x1B, 0x0F, 0x15, 0x00, 0x16, 0x0F, 0x09, 0x06, 0x0D, 0x00, 0x06,
        0x0D, 0x03, 0x1B, 0x15, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
        0x01, 0x1D, 0x0E, 0x0D, 0x07, 0x0B, 0x04, 0x15, 0x13, 0x0E, 0x1A, 0x1A, 0x13, 0x18, 0x15,
        0x1C, 0x1A, 0x15, 0x1C, 0x1B, 0x14, 0x19, 0x1C, 0x18, 0x16, 0x11, 0x12, 0x12, 0x0B, 0x10,
        0x18, 0x09, 0x07, 0x03, 0x1E, 0x1E, 0x18, 0x1C, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x15, 0x00, 0x00, 0x15, 0x1F, 0x00, 0x00, 0x0C, 0x06, 0x00, 0x00, 0x01,
        0x0B, 0x00, 0x00, 0x1C, 0x0D, 0x00, 0x00, 0x03, 0x0D, 0x00, 0x00, 0x03, 0x09, 0x00, 0x00,
        0x1A, 0x04, 0x00, 0x00, 0x1F, 0x1B, 0x00, 0x00, 0x07, 0x10, 0x00, 0x00, 0x11, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xFD,
    ];
    serial_conn.write_all(&on_message).unwrap();
    serial_conn.flush().unwrap();
    // this on message was made for holding the position at (0.05, 0.05, 0.14) cm, I think, whatever, this is just for testing anyway
    // let pos_phase = PositionPhases::new([0.05, 0.05, 0.14], [0.0; N_EMMITERS]);
    let pos_phase = PositionPhases::new(vec![Point::new(0.05, 0.05, 0.14)], vec![0.0; N_EMMITERS]);
    let _ = tx.send(pos_phase);
    ctx.request_repaint();
}

fn test_turn_off(
    serial_conn: &mut Box<dyn SerialPort>,
    tx: Sender<PositionPhases>,
    ctx: egui::Context,
) {
    let off_message: Vec<u8> = vec![
        0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20,
        0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00,
        0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x20, 0x00, 0x00, 0x20, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xFD,
    ];
    serial_conn.write_all(&off_message).unwrap();
    serial_conn.flush().unwrap();
    // let's just say off is 0.05, 0.05, 0.00
    // let pos_phase = PositionPhases::new([0.05, 0.05, 0.00], [0.0; N_EMMITERS]);
    let pos_phase = PositionPhases::new(vec![Point::new(0.05, 0.05, 0.0)], vec![0.0; N_EMMITERS]);
    let _ = tx.send(pos_phase);
    ctx.request_repaint();
}

fn solver_runtime(
    serial_conn: &mut Box<dyn SerialPort>,
    rx: Receiver<(Box<Vec<Vec<Point>>>, f32)>, // (control_points, time_step)
    tx: Sender<PositionPhases>,
    z: f32,
) {
    let mut control_points: Box<Vec<Vec<Point>>> = Box::new(vec![]);
    let mut time_step: f32 = 0.0;
    let runner = HatRunner::new(32.0, z);

    loop {
        // update control_points if new ones are sent
        match rx.try_recv() {
            Ok((cps, t)) => {
                control_points = cps;
                time_step = t;
            }
            // exit out if the channel is disconnected
            Err(TryRecvError::Disconnected) => return,
            Err(TryRecvError::Empty) => (),
        };

        run_control_points(&control_points, time_step, &runner, serial_conn, tx.clone());
    }
}

fn run_control_points(
    control_points: &Vec<Vec<Point>>,
    time_step: f32,
    runner: &HatRunner,
    serial_conn: &mut Box<dyn SerialPort>,
    tx: Sender<PositionPhases>,
) {
    // solve for phases in batch
    let phases = runner.run(control_points);

    // send phases to the SonicSurface
    for (ps, cps) in phases.iter().zip(control_points) {
        let start = Instant::now();
        let ss_phases = convert_to_sonic_surface_phases(&ps);

        serial_conn.write_all(&ss_phases).unwrap();
        serial_conn.flush().unwrap();

        tx.send(PositionPhases::new(cps.to_vec(), ps.to_vec()));

        let elapsed = Instant::now() - start;
        thread::sleep(Duration::from_secs_f32(time_step) - elapsed);
    }
}
