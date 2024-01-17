use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

use super::Driver;

#[derive(Default)]
pub struct RngDevice {
    rng_offset: usize,
}

impl Driver for RngDevice {
    unsafe fn write_to_buffer(&mut self, _buffer: &mut [u8]) {
        panic!("Attempt to write to rng device")
    }

    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]) {
        let mut random = File::open("/dev/random").expect("Error opening /dev/random");

        random.seek(SeekFrom::Start(self.rng_offset as u64)).expect("Error seeking to offset");
        random.read_exact(buffer).expect("Error filling buffer");
    }
}
