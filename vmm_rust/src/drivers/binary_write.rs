use std::io::Write;
use std::io::Read;

use super::Driver;

#[derive(Default)]
pub struct BinaryDriver {
}

impl Driver for BinaryDriver {
    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]) {
        let stdin = std::io::stdin();

        let mut handle = stdin.lock();
        handle.read(buffer).expect("Error reading stdin buffer");
    }

    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]) {
        let stdout = std::io::stdout();

        let mut handle = stdout.lock();

        for byte in buffer {
            handle.write_fmt(format_args!("{byte:X}")).expect("Error writing portio to console");
        }

        let new_line = [b'\n'];

        handle.write(&new_line).expect("Error writing stdin buffer");

        handle.flush().unwrap();
    }
}
