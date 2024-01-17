#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

#[macro_use]
mod print;
mod mmio;
mod cpu;
mod tests;

use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::mmio::sleep;

use crate::cpu::cpu_exceptions;
use crate::cpu::gdt;

global_asm!(include_str!("start.S"));

use cpu_exceptions::init_idt;
use gdt::init_gdt;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Error: {info:#?}");
    loop {}
}

unsafe fn init() {
    println!("Enabling interrupts \n");

    init_gdt();
    init_idt();

    x86_64::instructions::interrupts::enable();

    if x86_64::instructions::interrupts::are_enabled() {
        println!("Interrupts have been enabled\n");
    } else {
        panic!("Error enabling exceptions!");
    }
}


#[no_mangle]
pub unsafe extern "C" fn not_main() -> ! {
    init();

    tests::break_point::test_sidt_breakpoint();

    tests::mmio::test_rng_device();

    loop {
        sleep::sleep(1);
        sleep::sleep(1);
    }
}
