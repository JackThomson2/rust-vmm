use std::{convert::TryInto, time::Duration};

use super::Driver;

#[derive(Default)]
pub struct SleepDevice {
}

impl Driver for SleepDevice {
    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]) {
        if buffer.len() != 8 {
            panic!("Incorrect write length");
        }

        println!("Slot 1 is {}", buffer[0]);

        let sleep_time = u64::from_le_bytes(buffer.try_into().expect("Incorrect length"));
        let duration = Duration::from_secs(sleep_time);
        println!("Sleeping for {:#?}", duration);
        std::thread::sleep(duration);
    }

    unsafe fn read_to_buffer(&mut self, _buffer: &mut [u8]) {
        panic!("Attempt to read from sleep device")
    }
}
