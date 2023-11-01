// src/main.rs
//

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::arch::asm;
use core::panic::PanicInfo;
use x86::io::{inb, outl};

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(always)]
unsafe fn write_string_to_port(data: *const u8, len: usize, port: u16) {
    asm!(
        "rep outsb",
        in("rsi") data,
        in("rcx") len,
        in("dx") port
    );
}

const hello_world: &[u8] = b"Hello world mmio";

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {

    write_string_to_port(hello_world.as_ptr(), hello_world.len(), 0x1000);

    loop {}
}
