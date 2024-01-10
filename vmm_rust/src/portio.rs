use crate::drivers::{Drivers, Driver};

use kvm_bindings::*;

const COM1:u32 = 0x3f8;
const LONG_MODE_COM:u32 = 0x1000;
const LONG_MODE_HEX:u32 = 0x2000;
const COM2:u32 = 0x600;

unsafe fn get_driver<'d>(port: u32, drivers: &'d mut Drivers, memory: u32) -> Option<&'d mut dyn Driver> {
    return match port {
        COM1 | LONG_MODE_COM | COM2 => {
            Some(&mut drivers.console)
        },
        LONG_MODE_HEX => {
            Some(&mut drivers.binary)
        },
        _ => {
            None
        }
    }
}

pub unsafe fn handle_pio(run: &mut kvm_run, drivers: &mut Drivers, memory: u32) {
    let port = run.__bindgen_anon_1.io.port as u32;
    let driver = match get_driver(port, drivers, memory) {
        Some(driver) => { driver},
        None => {
            println!("Unknown PIO port: {port}.");
            return;
        }
    };

    match run.__bindgen_anon_1.io.direction as u32 {
        KVM_EXIT_IO_IN => {
            com1_pio_in(run, driver);

        },
        KVM_EXIT_IO_OUT => {
            com1_pio_out(run, driver);
        },
        _ => {
            panic!("Unexpected port IO exit")
        }
    }
}

unsafe fn get_io_buffer(run: &mut kvm_run) -> &mut [u8] {
    let io = run.__bindgen_anon_1.io;

    let root_ptr = run as *mut _ as *mut u8;
    let result_head = root_ptr.add(io.data_offset as usize);
    let size = (io.count as u64) * (io.size as u64);

    // println!("IO buffer of {size}");

    std::slice::from_raw_parts_mut(result_head, size as usize)
}

unsafe fn com1_pio_in(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.read_to_buffer(buffer);
}

unsafe fn com1_pio_out(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.write_to_buffer(buffer);
}
