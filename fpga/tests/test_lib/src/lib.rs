extern crate libftd2xx;

use libftd2xx::{Ft2232h, FtdiCommon, BitMode, FtStatus, DeviceTypeError, TimeoutError};
use std::thread;
use std::time::Duration;

pub const KIB: u32 = 1024;
pub const MIB: u32 = KIB * 1024;
pub const PHASE_OPER: u8 = 0x01;

pub struct FPGA {
    ftdi_serial: &'static str,
    fifo245_mode: &'static str,
    ftdev: Ft2232h,
}

impl FPGA {
    pub fn new(ftdi_serial: &'static str, fifo245_mode: &'static str) -> Result<Self, DeviceTypeError> {
        let ftdev = Ft2232h::with_serial_number(ftdi_serial)?;

        let fpga = FPGA {
            ftdi_serial,
            fifo245_mode,
            ftdev,
        };
        Ok(fpga)
    }

    pub fn open(&mut self) -> Result<(), FtStatus> {
        self.ftdev.reset()?;
        // self.ftdev.set_bit_mode(0xff, if self.fifo245_mode == "sync" { BitMode::SyncFifo } else { BitMode::Reset })?;
        self.ftdev.set_bit_mode(0xff, BitMode::SyncFifo)?;
        self.ftdev.set_timeouts(Duration::from_millis(255), Duration::from_millis(255))?;
        self.ftdev.set_usb_parameters(64 * KIB)?;
        self.ftdev.set_flow_control_rts_cts()?;
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), FtStatus> {
        self.ftdev.close()?;
        Ok(())
    }

    // the write_all command writes the byte in the lowest memory address first.
    // Hence, 0x55 is written first 
    pub fn cmd(&self, code: u16, data: u32) -> Vec<u8> {
        ((0xAA << 56) | ((code as u64) << 40) | ((data as u64) << 8) | 0x55)
            .to_le_bytes()
            .to_vec()
    }

    pub fn test_led(&mut self) -> Result<(), TimeoutError> {
        self.ftdev.write_all(&self.cmd(0x1ED0, 1))?;
        thread::sleep(Duration::from_secs(2));
        self.ftdev.write_all(&self.cmd(0x1ED0, 0))?;
        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    pub fn test_read(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
        // Prepare data
        let total_bytes = total_bytes.unwrap_or(1 * MIB);
        let golden_data: Vec<u8> = (0..total_bytes).map(|i| (i % 256) as u8).collect();

        // Start read test
        self.ftdev.write_all(&self.cmd(0xBEEF, total_bytes - 1))?;
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

    pub fn test_write(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
        // Prepare data
        let total_bytes = total_bytes.unwrap_or(1 * MIB);
        let data: Vec<u8> = (0..total_bytes).map(|i| (i % 256) as u8).collect();

        // Start write test
        self.ftdev.write_all(&self.cmd(0xCAFE, total_bytes - 1))?;
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

    pub fn set_phase(&mut self, address: u8, phase: u8, enable:bool) -> Result<(), TimeoutError> {
        let data: u32 = (enable as u32) << 16 | (address as u32) << 8 | phase as u32;
        self.ftdev.write_all(&self.cmd(0x0001, data))?;
        Ok(())
    }

    pub fn set_phase_multi(&mut self, num_writes: u32) -> Result<(), TimeoutError> {
        let num_channels: u32 = 4;
        let max_phase: u32 = 256;
        let mut buf = Vec::<u8>::new();
        // Write repeating pattern to buf
        for i in 0..num_writes {
            let data: u32 = 1 << 16 | (i % num_channels as u32) << 8 | (i % max_phase as u32);
            buf.extend(self.cmd(0x0001, data));
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

    pub fn set_phase_multi_v2(&mut self, num_writes: u32) -> Result<(), TimeoutError> {
        let num_channels: u32 = 4;
        let max_phase: u32 = 256;
        let mut buf = Vec::<u8>::new();
        let num_bytes: u32 = num_writes * 2;
        // Set fpga into burst mode, fpga will expect buffer size = num_bytes
        self.ftdev.write_all(&self.cmd(0x0002, num_bytes))?;
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
        for i in 0..num_frames {
            // Set fpga into burst mode, fpga will expect frame size
            self.ftdev.write_all(&self.cmd(0x0002, frame_bytes))?;
            // Write buf to FPGA
            self.ftdev.write_all(&buf)?;
        }
        let exec_time = start_time.elapsed().as_secs_f64();
        println!("set_phase_frame_buf: Wrote {} phases ({} bytes) in {}s to {}", num_writes, num_bytes, exec_time, self.ftdi_serial);
        Ok(())
    }
}
