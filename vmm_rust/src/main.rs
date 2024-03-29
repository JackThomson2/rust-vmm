#[macro_use]
extern crate vmm_sys_util;

mod drivers;
mod kvm_regs;
mod libc_macros;
mod mmio;
mod portio;
mod vm;

const MB_ONE: usize = 1024 * 1024 * 1024;

fn main() {
    println!("Creating the kvm now");
    let mut kvm = unsafe { vm::Vm::create(0x100000).unwrap() };
    println!("Created successfully");

    println!("Loading code into vm");
    unsafe { kvm.load_file("./miminal_app").unwrap() }

    println!("Running the KVM");
    unsafe { kvm.run().unwrap() };
}
