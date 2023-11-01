use std::io::Write;
use std::io::Read;

use super::Driver;

#[derive(Default)]
pub struct ConsoleDevice {
}

impl Driver for ConsoleDevice {
    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]) {
        let stdin = std::io::stdin();

        let mut handle = stdin.lock();
        handle.read(buffer).expect("Error reading stdin buffer");
    }

    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]) {
        let stdout = std::io::stdout();

        let mut handle = stdout.lock();
        handle.write_all(buffer).expect("Error writing portio to console");
        handle.flush().unwrap();
    }
}
