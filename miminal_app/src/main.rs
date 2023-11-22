#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// mod printing;

use core::arch::asm;
use core::panic::PanicInfo;

use core::arch::global_asm;

global_asm!(include_str!("start.s"));

// use crate::printing::{Writer, WRITER, write_string, write_bytes};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline]
fn as_array_8(slice: &[u8]) -> &[u8; 8] {
    unsafe { &*(slice.as_ptr() as *const [_; 8]) }
}

unsafe fn write_to_port(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

unsafe fn write_string_to_port(pointer: *const u8, length: usize, port: u16) {
    unsafe {
        asm!("rep outsb", in("rsi") pointer, in("dx") port, in("rcx") length, options(nomem, nostack, preserves_flags));
    }
}

unsafe fn read_from_port(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    value
}

const mem_total: usize = 10 * 1024 * 1024;
const HELLO_WORLD: &[u8] = b"Hello world port, from testing rust!!\n";

#[no_mangle]
pub extern "C" fn not_main() -> ! {
    unsafe {
        for &c in HELLO_WORLD.iter() {
            write_to_port(0x1000, c);
        }

        write_string_to_port(HELLO_WORLD.as_ptr(), HELLO_WORLD.len(), 0x1000);


        let mmio_location = (mem_total + 0x2000) as *mut u8;

        // core::ptr::write(HELLO_WORLD.as_ptr(), mmio_location, 8);

        // for &letter in HELLO_WORLD.iter() {
        //     mmio_location.write_volatile(letter);
        // }

        // let mut hello_iter = HELLO_WORLD.chunks_exact(8);
        // while let Some(letter) = hello_iter.next() {
        //     core::ptr::copy_nonoverlapping(letter.as_ptr(), mmio_location, letter.len());
        // }
    }

    // for &letter in hello_iter.remainder().iter() {
    //     mmio_location.write_volatile(letter);
    // }

    // write_to_port(0x1000, 0);
    // write_to_port(0x1000, 0);
    // write_to_port(0x1000, 0);

    // for letter in HELLO_WORLD.chunks(8) {
    //     core::ptr::copy_nonoverlapping(letter.as_ptr(), mmio_location, letter.len());
    // }

    // write_bytes(HELLO_WORLD);

    // write_string("Printing is much easier now..");

    loop {}
}
