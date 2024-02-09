#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(abi_x86_interrupt)]

#[macro_use]
mod print;
mod mmio;
mod cpu;
mod tests;
mod memory;
mod virtio_devices;

use core::arch::global_asm;
use core::panic::PanicInfo;

use crate::mmio::rng::get_rng;
use crate::mmio::sleep;

use crate::cpu::cpu_exceptions;
use crate::cpu::gdt;

use crate::virtio_devices::blk::BlockVirtioDevice;

use virtio::virtqueue::VirtQueue;

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

    let mut virt_queue: VirtQueue<128> = VirtQueue::new_with_size(0x60000);

    let mut block_device = BlockVirtioDevice::new_from_loc((&mut virt_queue) as *mut _);
    println!("VirtQueue looks like: \n {:?}", virt_queue);

    let mut idx = 1;
    loop {
        block_device.post_message_to_queue(idx);
        // block_device.check_for_messages();
        sleep::sleep(1);
        idx += 1;
    }
}
