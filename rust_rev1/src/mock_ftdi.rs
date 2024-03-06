extern crate libftd2xx;

use libftd2xx::{FtStatus, TimeoutError};
use std::time::Duration;

const READ_RESULT: usize = 10;
const WRITE_RESULT: usize = 10;

pub struct Ft2232h {
    last_write: Vec<u8>
}

impl Ft2232h {
    pub fn with_serial_number(_serial_number: &str) -> Result<Ft2232h, FtStatus> {
        // Do nothing
        Ok(Ft2232h { last_write: Vec::new()})
    }

    pub fn reset(&mut self) -> Result<(), FtStatus> {
        // Do nothing
        Ok(())
    }

    pub fn set_timeouts(&mut self, _read_timeout: Duration, _write_timeout: Duration) -> Result<(), FtStatus>  {
        Ok(())
    }

    pub fn set_usb_parameters(&mut self, _in_transfer_size: u32) -> Result<(), FtStatus> {
        Ok(())
    }

    pub fn set_flow_control_rts_cts(&mut self) -> Result<(), FtStatus> {
        Ok(())
    }

    pub fn close(&mut self) -> Result<(), FtStatus> {
        Ok(())
    }

    pub fn write(&mut self, _buf: &[u8]) -> Result<usize, FtStatus> {
        Ok(WRITE_RESULT)
    }

    pub fn write_all(&mut self, buf: &[u8]) -> Result<(), TimeoutError> {
        self.last_write = buf.to_vec();
        Ok(())
    }

    pub fn read(&mut self, _buf: &[u8]) -> Result<usize, FtStatus> {
        Ok(READ_RESULT)
    }

    pub fn read_all(&mut self, _buf: &[u8]) -> Result<(), TimeoutError> {
        Ok(())
    }

    pub fn get_last_write(&mut self) -> &[u8] {
        return &self.last_write
    }

}
