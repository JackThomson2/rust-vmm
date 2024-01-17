#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

#[macro_use]
mod print;
mod mmio;
mod cpu;
mod tests;
mod virtio;
mod memory;

use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::mmio::rng::get_rng;
use crate::mmio::sleep;

use crate::cpu::cpu_exceptions;
use crate::cpu::gdt;

use crate::virtio::virtqueue::VirtQueue;

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

    println!("Making a new virtio queue!!");

    let virt_queue: VirtQueue<100> = VirtQueue::new_with_size(0x60000);

    println!("VirtQueue looks like: \n {:?}", virt_queue);

    for _ in 0..100 {
        let rng_pos = get_rng() % 100;
        let descriptor_cell_one = virt_queue.get_descriptor_from_idx(rng_pos as u16);
        println!("\n\nFound descriptor_cell: {:?}", descriptor_cell_one);

        mmio::virtio::mmio_write_loc(rng_pos);
        println!("descriptor_cell after write: {:?}", descriptor_cell_one);
    }

    loop {
        sleep::sleep(1);
    }
}
