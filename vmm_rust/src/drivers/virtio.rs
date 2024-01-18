use std::{io::{Read, Seek, SeekFrom}, convert::TryInto, mem::ManuallyDrop, borrow::BorrowMut, ops::DerefMut};

use super::Driver;

use virtio::{virtqueue::{VirtQueue, DescriptorCell}, driver::DeviceDriver};

pub struct VirtIODevice {
    pub device_driver: DeviceDriver<128>,
}

impl VirtIODevice {

    pub fn new(memory_loc: *mut u8, memory_amount: u64) -> Self {
        // let memory_pos = (memory_loc as usize) + memory_amount as usize + 0x60000;
        let memory_pos = (memory_loc as usize) + 0x65000;

        unsafe {
            let mut queue = ManuallyDrop::new(VirtQueue::from_memory(memory_pos));

            Self {
                device_driver: DeviceDriver::new(queue.deref_mut() as *mut _)
            }
        }
    }

    unsafe fn check_for_messages(&mut self) {
        {
            let queue = self.device_driver.queue.as_mut().unwrap();
            let available_ring = queue.available.as_mut().unwrap();

            let avail_idx = available_ring.get_idx();

            println!("Available ring: {available_ring:?}. Avail idx: {avail_idx}");
        }

        if let Some((cell, idx)) = self.device_driver.poll_available_queue() {
            let cell_ptr = cell as *mut ManuallyDrop<DescriptorCell>;

            let real_cell = cell.as_ref();
            println!("Idx is {idx}");

            println!("The cell is {real_cell:?}");
        }
    }
}

impl Driver for VirtIODevice {
    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]) {
        println!("We got a mmio notification for virtio");

        self.check_for_messages();
    }

    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]) {
    }
}
