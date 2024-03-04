extern crate libftd2xx;

use libftd2xx::{Ft2232h, FtdiCommon, BitMode, FtStatus, DeviceTypeError, TimeoutError};
use std::thread;
use std::time::Duration;


#[repr(u64)]
enum CommandEnum {
    PhaseData = 0x0001,
    BurstPhase = 0x0002,
    TestLED = 0x1ED0,
    ReadTest = 0xBEEF,
    WriteTest = 0xCAFE
}

const KIB: u32 = 1024;
pub const MIB: u32 = KIB * 1024;
const COMMAND_PREFIX: u64 = 0xAA;
const COMMAND_SUFFIX: u64 = 0x55;
const PHASE_CONV_FACTOR: f32 = 128.0 / (2.0 * 3.14159);

pub struct FPGA {
    ftdi_serial: &'static str,
    ftdev: Ft2232h,
}

impl FPGA {
    /** new
     * Creates a new FPGA object that wraps and handles the FT device object
     */
    pub fn new(ftdi_serial: &'static str) -> Result<Self, DeviceTypeError> {
        let ftdev = Ft2232h::with_serial_number(ftdi_serial)?;

        let fpga = FPGA {
            ftdi_serial,
            ftdev,
        };
        Ok(fpga)
    }

    /** open
     * Resets and configures connection to FT device
     */
    pub fn open(&mut self) -> Result<(), FtStatus> {
        self.ftdev.reset()?;
        // If using Sync mode, uncomment below. Sync mode is only single channel
        // self.ftdev.set_bit_mode(0x40, BitMode::SyncFifo)?;
        // Sets timeouts to flush receive buffer. Default 16ms, ranges from 2ms to 255ms
        self.ftdev.set_timeouts(Duration::from_millis(255), Duration::from_millis(255))?;
        // Sets USB buffer size, range 64B to 64kB
        self.ftdev.set_usb_parameters(64 * KIB)?;
        // Set flow control to RTS_CTS to prevent data loss and optimize performance
        self.ftdev.set_flow_control_rts_cts()?;
        Ok(())
    }

    /** close
     *  Closes connection to FT2232H
     */
    pub fn close(&mut self) -> Result<(), FtStatus> {
        self.ftdev.close()?;
        Ok(())
    }

    /** get_serial
     * Fetches the device serial ID
     * First 8 chars are the device ID
     * Ends in "A" if using channel 0
     * Ends in "B" if using channel 1
     * @returns &str: Pointer to device ID string
     */
    pub fn get_serial(&mut self) -> &'static str {
        return self.ftdi_serial;
    }

    /** cmd
     * @return the command in a vector of bytes of length 8 to send to the FPGA
     */
    fn cmd(&self, command: CommandEnum, data: u32) -> Vec<u8> {
        ((COMMAND_PREFIX << 56) | ((command as u64) << 40) | ((data as u64) << 8) | COMMAND_SUFFIX)
        // the write_all command writes the byte at the lowest index of the vector first
        // but, the fpga interprets the command bytes in big endian
        // so we need to shuffle the suffix byte to the lowest index, etc.
        .to_le_bytes()
            .to_vec()
    }

    /** test_led
     * Toggles the `receive_err` net in FPGA over 4 seconds
     */
    pub fn test_led(&mut self) -> Result<(), TimeoutError> {
        self.ftdev.write_all(&self.cmd(CommandEnum::TestLED, 1))?;
        thread::sleep(Duration::from_secs(2));
        self.ftdev.write_all(&self.cmd(CommandEnum::TestLED, 0))?;
        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    /** set_phase
     * Sets the phase of an individual transducer, and whether it is on or off
     * @param address: u8, transducer address to modify
     * @param phase: u8, phase between [0, 127] which map to [0, 2pi]
     * @param enable: bool, sets whether the transducer is pulsing or not
     */
    pub fn set_phase(&mut self, address: u8, phase: u8, enable:bool) -> Result<(), TimeoutError> {
        let data: u32 = (enable as u32) << 16 | (address as u32) << 8 | phase as u32;
        self.ftdev.write_all(&self.cmd(CommandEnum::PhaseData, data))?;
        Ok(())
    }

    /** set_multiple
     * Sets multiple phases and enables transducers at the specified transducer addresses
     * @param phases: the phases between [0, 2pi]
     * @param addresses: the transducer address that corresponds with each phase at the same index.
     * If either vector is longer, only indices up to the shorter length is handled.
     */
    pub fn set_multi(&mut self, phases: &[f32], addresses: &[u8]) -> Result<(), TimeoutError> {
        // determine size of phase and address data in bytes
        let payload_bytes: u32 = (phases.len().min(addresses.len()) * 2) as u32;

        // first prepend command
        // then zip together transducer addresses and phases into a buffer
        let buf = (self.cmd(CommandEnum::BurstPhase, payload_bytes)).into_iter()
            .chain(phases.into_iter()
                .map(|&phi| phi)
                .map(|phi| (phi * PHASE_CONV_FACTOR).round() as u8)
                .zip(addresses.into_iter()
                    .map(|&adr| adr))
                .flat_map(|(phi, adr)| vec![phi, adr]))
            .collect::<Vec<u8>>();

        // write the buffer to the fpga
        self.ftdev.write_all(&buf)?;
        Ok(())
    }

    /** test_read
     * testing use only
     */
    pub fn test_read(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
        // Prepare data
        let total_bytes = total_bytes.unwrap_or(1 * MIB);
        let golden_data: Vec<u8> = (0..total_bytes).map(|i| (i % 256) as u8).collect();

        // Start read test
        self.ftdev.write_all(&self.cmd(CommandEnum::ReadTest, total_bytes - 1))?;
        thread::sleep(Duration::from_secs(2));

        // Receive data
        let mut chunks = Vec::new();
        let start_time = std::time::Instant::now();
        let mut remaining_bytes = total_bytes;

        while remaining_bytes > 0 {
            let mut chunk = Vec::new();
            match self.ftdev.read(&mut chunk) {
                Ok(len_chunk) => {
                    chunks.push(chunk);
                    remaining_bytes -= len_chunk as u32;
                }
                Err(timeout_error) => {
                    println!("read timed out: {} ", timeout_error);
                    break
                }
            }
        }
        // If the above doesn't work:

        // let mut data = Vec::<u8>::new();
        // self.ftdev.read_all(&mut data)?;

        let exec_time = start_time.elapsed().as_secs_f64();

        // Print statistics
        let data: Vec<u8> = chunks.into_iter().flatten().collect();
        let data_len = data.len();
        let data_len_mb = data_len as f64 / MIB as f64;
        println!(
            "Read {:.02} MiB ({} bytes) from FPGA in {} seconds ({:.02} MiB/s)",
            data_len_mb,
            data_len,
            exec_time,
            data_len_mb / exec_time
        );

        // Verify data
        println!("Verify data: {}", if golden_data == data { "ok" } else { "error" });
        Ok(())
    }

    /** test_write
     * testing use only
     */
    pub fn test_write(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
        // Prepare data
        let total_bytes = total_bytes.unwrap_or(1 * MIB);
        let data: Vec<u8> = (0..total_bytes).map(|i| (i % 256) as u8).collect();

        // Start write test
        self.ftdev.write_all(&self.cmd(CommandEnum::WriteTest, total_bytes - 1))?;
        thread::sleep(Duration::from_secs(2));

        // Transmit data
        let mut offset: usize = 0;
        let mut data_len = total_bytes;
        let result = 0u8;
        let start_time = std::time::Instant::now();

        while data_len > 0 {
            let chunk_len: usize = std::cmp::min(1 * MIB, data_len) as usize;
            match self.ftdev.write(&data[offset..offset + chunk_len]) {
                Ok(written_len) => {
                    data_len -= written_len as u32;
                    offset += written_len;
                }
                Err(err) => {
                    return Err(TimeoutError::FtStatus(err))
                }
            }
        }

        self.ftdev.read_all(&mut [result])?;

        let exec_time = start_time.elapsed().as_secs_f64();

        // Print statistics
        let data_len_mb = total_bytes as f64 / MIB as f64;
        println!(
            "Wrote {:.02} MiB ({} bytes) to FPGA in {} seconds ({:.02} MiB/s)",
            data_len_mb,
            total_bytes,
            exec_time,
            data_len_mb / exec_time
        );

        // Verify data
        let result = if result == 0 { "ok" } else { "error" };
        println!("Verify data: {}", result);
        Ok(())
    }

    /** set_phase_multi
     * Testing only
     */
    pub fn set_phase_multi(&mut self, num_writes: u32) -> Result<(), TimeoutError> {
        let num_channels: u32 = 4;
        let max_phase: u32 = 256;
        let mut buf = Vec::<u8>::new();
        // Write repeating pattern to buf
        for i in 0..num_writes {
            let data: u32 = 1 << 16 | (i % num_channels as u32) << 8 | (i % max_phase as u32);
            buf.extend(self.cmd(CommandEnum::PhaseData, data));
        }
        let num_bytes = buf.len();
        // Time the write
        let start_time = std::time::Instant::now();
        // Write buf to FPGA
        self.ftdev.write_all(&buf)?;
        let exec_time = start_time.elapsed().as_secs_f64();
        println!("set_phase_multi: Wrote {} phases ({} bytes) in {}s to {}", num_writes, num_bytes, exec_time, self.ftdi_serial);
        Ok(())
    }

    /** set_phase_multi_v2
     * Testing only
     */
    pub fn set_phase_multi_v2(&mut self, num_writes: u32) -> Result<(), TimeoutError> {
        let num_channels: u32 = 4;
        let max_phase: u32 = 256;
        let mut buf = Vec::<u8>::new();
        let num_bytes: u32 = num_writes * 2;
        // Set fpga into burst mode, fpga will expect buffer size = num_bytes
        self.ftdev.write_all(&self.cmd(CommandEnum::BurstPhase, num_bytes))?;
        for i in 0..num_writes {
            let data: Vec::<u8> = vec![(i % max_phase).try_into().unwrap(), (i % num_channels).try_into().unwrap()];
            buf.extend(data);
        }
        // Time the write
        let start_time = std::time::Instant::now();
        self.ftdev.write_all(&buf)?;
        let exec_time = start_time.elapsed().as_secs_f64();
        println!("set_phase_multi_v2: Wrote {} phases ({} bytes) in {}s to {}", num_writes, num_bytes, exec_time, self.ftdi_serial);
        Ok(())
    }

    /** set_phase_frame_buf
     * Testing only
     */
    pub fn set_phase_frame_buf(&mut self, num_writes: u32) -> Result<(), TimeoutError> {
        let num_channels_real: u32 = 4;
        let num_channels_sim: u32 = 128;
        let max_phase: u32 = 256;
        let num_bytes_per_channel: u32 = 2;

        let num_bytes: u32 = num_writes * num_bytes_per_channel;
        let num_frames: u32 = num_writes / num_channels_sim;
        let frame_bytes: u32 = num_channels_sim * num_bytes_per_channel;

        // Crete repeating pattern in buf
        let mut buf = Vec::<u8>::new();
        for i in 0..num_channels_sim {
            let data: Vec::<u8> = vec![(i % max_phase).try_into().unwrap(), (i % num_channels_real).try_into().unwrap()];
            buf.extend(data);
        }

        // Time the write
        let start_time = std::time::Instant::now();
        for _ in 0..num_frames {
            // Set fpga into burst mode, fpga will expect frame size
            self.ftdev.write_all(&self.cmd(CommandEnum::BurstPhase, frame_bytes))?;
            // Write buf to FPGA
            self.ftdev.write_all(&buf)?;
        }
        let exec_time = start_time.elapsed().as_secs_f64();
        println!("set_phase_frame_buf: Wrote {} phases ({} bytes) in {}s to {}", num_writes, num_bytes, exec_time, self.ftdi_serial);
        Ok(())
    }
}
