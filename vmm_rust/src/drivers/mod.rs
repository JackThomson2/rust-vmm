use self::{console::ConsoleDevice, rng::RngDevice, binary_write::BinaryDriver};

pub mod console;
pub mod rng;
pub mod binary_write;

pub trait Driver {
    unsafe fn write_to_buffer(&mut self, buffer: &mut [u8]);

    unsafe fn read_to_buffer(&mut self, buffer: &mut [u8]);
}

#[derive(Default)]
pub struct Drivers {
    pub console: ConsoleDevice,
    pub rng: RngDevice,
    pub binary: BinaryDriver,
}

impl Drivers {

}
