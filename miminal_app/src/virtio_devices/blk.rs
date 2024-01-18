use virtio::driver::DeviceDriver;
use virtio::virtqueue::VirtQueue;

use crate::mmio::virtio::mmio_write_loc;

pub struct BlockVirtioDevice {
    driver: DeviceDriver<128>
}

impl BlockVirtioDevice {
    pub fn new_from_loc(virt_queue: *mut VirtQueue<128>) -> Self {
        Self {
            driver: DeviceDriver::new(virt_queue)
        }
    }

    pub unsafe fn post_message_to_queue(&mut self, flag: u16) {
        self.driver.write_pointer_to_queue(0 as *const u8, 0, flag);

        mmio_write_loc(1);
    }

    pub unsafe fn check_for_messages(&mut self) {
        while let Some(message) = self.driver.check_used_queue() {
            println!("Got a virio message on the OS!");
        }
    }
}
