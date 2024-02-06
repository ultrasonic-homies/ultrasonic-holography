# FPGA Repository

## Compiling Quartus Project Locally

Requirements

* Intel Quartus Prime 19.1
* Python

### 1. Generating Quartus Project

The Quartus project (including `.qpf` file) is configured in the `.tcl` file, and is generated with the shell script `create_proj.sh`

```bash
cd ultrasonic_holography/fpga/<your fpga>
./create_proj.sh
```

To clean the repository of Quartus project files, run `clean.sh`

```bash
cd ultrasonic_holography/fpga/<your fpga>
./clean.sh
```

### 2. Change Pin Assignments

To change the pinout assignments:
1. Edit `\scripts\pinout.json`
2. Run `\scripts\generate_pin_assignments.py` to generate a `.qsf` file.
3. Import the `.qsf` file in Quartus: Assignments -> Import Assignments

### 3. Compile Quartus Project

1. In Quartus, click the blue triangle (or Processing -> Start Compilation)
2. Once complete, your `top.sof` configuration file can be found in `ultrasonic_holography/fpga/<your fpga>/output_files`
3. You'll need to convert this configuration file into `.jic`. Go to File -> Convert Programming Files

    * Click "Open Conversion Setup Data"
    * Select `jic_conversion_setup.cof`
    * Click "Generate"

## Testing

### `de1_soc_top` test

This test covers changing the phase of one transducer output.

Prerequisites:
- Rust is installed on the PC
- FPGA has the program loaded
- FT device EEPROM is configured to FT245 Fifo mode using D2XX drivers
- FT device is connected to PC via USB
- The correct serial number is used to initialize the FPGA object in `de1_soc_top/main.rs` (this can be found using FT_Prog util with "Scan and Parse", or using libftd2xx::list_devices() while FT device is connected via USB)
- DATA[0:7], RXF#, TXE#, RDN, WRN, SIWU pins are hooked up between FPGA and FT device

Running the test:

```bash
cd ultrasonic_holography/fpga/tests/de1_soc_top
cargo run
```