use std::{io::{Read, Seek, SeekFrom}, convert::TryInto};

use super::Driver;

use crate::virtio::virtqueue::VirtQueue;

pub struct VirtIODevice {
    pub virtqueue: VirtQueue<100>
}

impl VirtIODevice {

    pub fn new(memory_loc: *mut u8, memory_amount: u64) -> Self {
        // let memory_pos = (memory_loc as usize) + memory_amount as usize + 0x60000;
        let memory_pos = (memory_loc as usize) + 0x65000;

        unsafe {
            Self {
                virtqueue: VirtQueue::new_with_size(memory_pos)
            }
        }
    }
}

impl Driver for VirtIODevice {
    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]) {
        if buffer.len() != 8 {
            panic!("Incorrect write length");
        }

        let mmio_loc = u64::from_le_bytes(buffer.try_into().expect("Incorrect length"));
        let virt_pos = self.virtqueue.get_descriptor_from_idx(mmio_loc as u16);
        virt_pos.length += 1;
    }

    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]) {
    }
}
