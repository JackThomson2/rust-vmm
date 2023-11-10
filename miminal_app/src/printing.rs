use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

lazy_static! {
    pub static ref WRITER: Writer = Writer {};
}


pub struct Writer();

impl Writer {
    pub fn write_string(&self, data: &str) {
        unsafe {
            // let mmio_location = 0x4000 as *mut u8;

            // for letter in data.as_bytes().chunks(8) {
            //     core::ptr::copy(letter.as_ptr(), mmio_location, letter.len());
            // }
        }
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
    ($($arg:tt)*) => ($crate::printing::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
     let mmio_location = 0x4000 as *mut u8;

     for letter in data.as_bytes().chunks(8) {
         core::ptr::copy(letter.as_ptr(), mmio_location, letter.len());
     }
    
     return;

    // use core::fmt::Write;
    // WRITER.lock().write_fmt(args).unwrap();
}
