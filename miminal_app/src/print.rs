use core::{arch::asm, fmt::{self, Write}};

use lazy_static::lazy_static;
use spin::Mutex;

use crate::mmio::write_silce_to_mmio_port;

const MMIO_COM_LOCATION: usize = 0x64000;

lazy_static! {
    static ref WRITER: Mutex<Writer> = {
        Mutex::new(Writer {})
    };
}

pub struct Writer();

impl Writer {
    pub fn write_string(&self, data: &str) {
        unsafe {
            write_str_to_port(data)
        }
    }
}

unsafe fn write_str_to_port(string: &str) {
    write_silce_to_mmio_port(MMIO_COM_LOCATION, string.as_bytes());
}

unsafe fn write_to_port(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options());
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}
