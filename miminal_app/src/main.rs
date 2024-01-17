#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

#[macro_use]
mod print;
mod cpu_exceptions;
mod gdt;
mod interrupts;
mod mmio;
mod rng;
mod sleep;

use core::arch::global_asm;
use core::panic::PanicInfo;

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
    // interrupts::PICS.lock().initialize();
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

    println!("IDT is at {:#?}", x86_64::instructions::tables::sidt());
    println!("Testing a debug breakpoint");
    x86_64::instructions::interrupts::int3();

    // println!("Triggering a stack overflow");
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    //     volatile::Volatile::new(0).read();
    // }

    // // trigger a stack overflow
    // stack_overflow();
    // println!("Past the stack overflow now somehow");

    for _ in 0..10 {
        println!("Read {} from rng port", rng::get_rng());
    }

    // println!("Rip is at {:?}", x86_64::instructions::read_rip());

    // println!("Testing invalid location");


    // let mmio_location = (0x1264001) as *mut u8;
    // println!("Found data at: {}", mmio_location.read());
    // println!("Wrote to invalid location!\n");

    loop {
        crate::sleep::sleep(10);
        crate::sleep::sleep(11);
    }
}
