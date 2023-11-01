use crate::drivers::Drivers;
use crate::drivers::Driver;

use kvm_bindings::*;

const RNG_MMIO_DEVICE:u32 = 0x2000;
const MMAP_COM_DEVICE:u32 = 0x3000;

unsafe fn get_driver<'d>(run: &mut kvm_run, drivers: &'d mut Drivers) -> Option<&'d mut dyn Driver> {
    return match run.__bindgen_anon_1.mmio.phys_addr as u32 {
        MMAP_COM_DEVICE => {
            Some(&mut drivers.console)
        },
        RNG_MMIO_DEVICE => {
            Some(&mut drivers.rng)
        },
        _ => {
            None
        }
    }
}

pub unsafe fn handle_mmio(run: &mut kvm_run, drivers: &mut Drivers) {
    let driver = match get_driver(run, drivers) {
        Some(driver) => { driver},
        None => {
            println!("Unknown address access");
            return;
        }
    };

    match run.__bindgen_anon_1.io.direction as u32 {
        KVM_EXIT_IO_IN => {
            mmio_in(run, driver);

        },
        KVM_EXIT_IO_OUT => {
            mmio_out(run, driver);
        },
        _ => {
            panic!("Unexpected port IO exit")
        }
    }
}

unsafe fn get_io_buffer(run: &mut kvm_run) -> &mut [u8] {
    let mut mmio = run.__bindgen_anon_1.mmio;
    let root_ptr = (&mut mmio.data) as *mut u8;

    std::slice::from_raw_parts_mut(root_ptr, mmio.len as usize)
}

unsafe fn mmio_in(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.read_to_buffer(buffer);
}

unsafe fn mmio_out(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.write_to_buffer(buffer);
}
