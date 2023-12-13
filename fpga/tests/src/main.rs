extern crate libftd2xx;

use libftd2xx::{Ft2232h, FtdiCommon, BitMode, FtStatus, DeviceTypeError, TimeoutError};
use std::thread;
use std::time::Duration;

const KIB: u32 = 1024;
const MIB: u32 = KIB * 1024;

struct FPGA {
    ftdi_serial: &'static str,
    fifo245_mode: &'static str,
    ftdev: Ft2232h,
}

impl FPGA {
    fn new(ftdi_serial: &'static str, fifo245_mode: &'static str) -> Result<Self, DeviceTypeError> {
        let ftdev = Ft2232h::with_serial_number(ftdi_serial)?;

        let fpga = FPGA {
            ftdi_serial,
            fifo245_mode,
            ftdev,
        };
        Ok(fpga)
    }

    fn open(&mut self) -> Result<(), FtStatus> {
        self.ftdev.reset()?;
        // self.ftdev.set_bit_mode(0xff, if self.fifo245_mode == "sync" { BitMode::SyncFifo } else { BitMode::Reset })?;
        self.ftdev.set_bit_mode(0xff, BitMode::SyncFifo)?;
        self.ftdev.set_timeouts(Duration::from_millis(255), Duration::from_millis(255))?;
        self.ftdev.set_usb_parameters(64 * KIB)?;
        self.ftdev.set_flow_control_rts_cts()?;
        Ok(())
    }

    fn close(&mut self) -> Result<(), FtStatus> {
        self.ftdev.close()?;
        Ok(())
    }

    fn cmd(&self, code: u16, data: u32) -> Vec<u8> {
        ((0xAA << 56) | ((code as u64) << 40) | ((data as u64) << 8) | 0x55)
            .to_le_bytes()
            .to_vec()
    }

    fn test_led(&mut self) -> Result<(), TimeoutError> {
        self.ftdev.write_all(&self.cmd(0x1ED0, 1))?;
        thread::sleep(Duration::from_secs(2));
        self.ftdev.write_all(&self.cmd(0x1ED0, 0))?;
        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    fn test_read(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
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

    fn test_write(&mut self, total_bytes: Option<u32>) -> Result<(), TimeoutError> {
        // Prepare data
        let total_bytes = total_bytes.unwrap_or(1 * MIB);
        let data: Vec<u8> = (0..total_bytes).map(|i| (i % 256) as u8).collect();

        // Start write test
        self.ftdev.write_all(&self.cmd(0xCAFE, total_bytes - 1))?;
        thread::sleep(Duration::from_secs(2));

        // Transmit data
        let mut offset = 0;
        let mut data_len = total_bytes;
        let mut result = 0u8;
        let start_time = std::time::Instant::now();

        while data_len > 0 {
            let chunk_len = std::cmp::min(1 * MIB, data_len);
            match self.ftdev.write_all(&data[offset..offset + chunk_len]) {
                Ok(written_len) => {
                    data_len -= written_len;
                }
                Err(err) => {
                    return
                }
            }
            data_len -= written_len;
            offset += written_len;
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
    }
}

fn main() {
    match FPGA::new("FT7TEQ7VA", "sync") {
        Ok(mut de1_soc) => {
            match de1_soc.test_led() {
                Ok(()) => {
                    println!("test_led completed");
                }
                Err(timeout_error) => {
                    println!("test_led failed with TimeoutError: {}", timeout_error);
                    let _ = de1_soc.close();
                    return
                }
            }
            match de1_soc.test_read(Some(10 * MIB)) {
                Ok(()) => {
                    println!("test_read completed");
                }
                Err(timeout_error) => {
                    println!("test_read failed with TimeoutError: {}", timeout_error);
                    let _ = de1_soc.close();
                    return
                }
            }
            // de1_soc.test_write(20 * MIB);
            let _ = de1_soc.close();
        }
        Err(device_type_error) => {
            println!("Initialization failed with error: {}", device_type_error)
        }
    }
}
