use crate::drivers::Drivers;
use crate::drivers::Driver;

use kvm_bindings::*;


const MMAP_COM_DEVICE:u64 = 0x1000;
const COM_SIZE:u64 = 0x1000;
const MMAP_COM_END:u64 = MMAP_COM_DEVICE + COM_SIZE - 1;

const RNG_MMIO_DEVICE:u64 =   0x2000;
const SLEEP_MMIO_DEVICE:u64 = 0x2001;

const VIRTIO_MMIO_DEVICE:u64 = 0x2002;

unsafe fn get_driver<'d>(run: &mut kvm_run, drivers: &'d mut Drivers, memory: u64) -> Option<&'d mut dyn Driver> {
    let address = run.__bindgen_anon_1.mmio.phys_addr - memory;
    match address {
        MMAP_COM_DEVICE..=MMAP_COM_END => {
            Some(&mut drivers.console)
        },
        RNG_MMIO_DEVICE => {
            Some(&mut drivers.rng)
        },
        SLEEP_MMIO_DEVICE => {
            Some(&mut drivers.sleep)
        },
        VIRTIO_MMIO_DEVICE => {
            Some(&mut drivers.virtio)
        }
        x => {
            println!("Unkown address {x:0x}");
            None
        }
    }
}

pub unsafe fn handle_mmio(run: &mut kvm_run, drivers: &mut Drivers, memory: u64) {
    let driver = match get_driver(run, drivers, memory) {
        Some(driver) => { driver},
        None => {
            return;
        }
    };

    if run.__bindgen_anon_1.mmio.is_write == 0 {
        mmio_in(run, driver);
    } else {
        mmio_out(run, driver);
    }
}

unsafe fn get_io_buffer(run: &mut kvm_run) -> &mut [u8] {
    let mmio = run.__bindgen_anon_1.mmio;

    &mut run.__bindgen_anon_1.mmio.data[..(mmio.len as usize)]
}

unsafe fn mmio_in(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.read_to_buffer(buffer);
}

unsafe fn mmio_out(run: &mut kvm_run, driver: &mut dyn Driver) {
    let buffer = get_io_buffer(run);
    driver.write_to_buffer(buffer);
}
