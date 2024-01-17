use self::{console::ConsoleDevice, rng::RngDevice, binary_write::BinaryDriver, sleep::SleepDevice, virtio::VirtIODevice};

pub mod console;
pub mod rng;
pub mod sleep;
pub mod binary_write;
pub mod virtio;

pub trait Driver {
    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]);

    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]);
}

pub struct Drivers {
    pub console: ConsoleDevice,
    pub rng: RngDevice,
    pub binary: BinaryDriver,
    pub sleep: SleepDevice,
    pub virtio: VirtIODevice
}

impl Drivers {

    pub fn new_drivers(memory_loc: *mut u8, memory_amount: u64) -> Self {
        Self {
            console: Default::default(),
            rng: Default::default(),
            binary: Default::default(),
            sleep: Default::default(),

            virtio: VirtIODevice::new(memory_loc, memory_amount),
        }
    }
}

impl Drivers {

}
