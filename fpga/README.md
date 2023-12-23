# FPGA

## Generating Quartus Project

The Quartus project (including .qpf file) is automatically generated using the .tcl file:

```
cd ultrasonic_holography/fpga
./create_proj.sh
```

Quartus project files are ignored in this folder. To clean project files, run:
```
cd ultrasonic_holography/fpga
./clean.sh
```

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

```
cd ultrasonic_holography/fpga/tests/de1_soc_top
cargo run
```