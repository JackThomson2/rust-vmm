#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod outputter;

use core::arch::asm;
use core::panic::PanicInfo;

use core::arch::global_asm;

global_asm!(include_str!("start.S"));

global_asm!(include_str!("page_fault.S"));
global_asm!(include_str!("interrupts_blank.S"));

use crate::outputter::{write_string, write_bytes, get_number};

extern "C" {
    fn get_page_loc() -> u64;
}

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
        asm!("out dx, al", in("dx") port, in("al") value, options());
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

#[inline(always)]
unsafe fn debug_line() {
    write_to_port(0x1000, b'.');
    write_to_port(0x1000, b'\n');
}

#[inline(never)]
unsafe fn dump_64_bit(incoming: u64) -> [u8; 20] {
    let mut res = [0; 20];
    let mut idx = 0;

    for y in 0..4 {
        let mut using = incoming;

        let to_shift = (3 - y) * 4;
        using >>= to_shift;
        using &= 0b1111;

        for i in 0..4 {
            let inner_shift = (3 - i);
            let result = (using >> inner_shift) & 1;

            res[idx] = (result + 48) as u8;
            idx += 1;
        }

        if y < 3 {
            res[idx] = b'_';
            idx += 1;
        }
    }

    res
}

#[inline(never)]
unsafe fn int_to_str(incoming: u64) -> [u8; 20] {
    let mut res = [0; 20];
    let mut idx = 0;

    let mut offsetter = 1000;

    for i in 0..4 {
        let divisor = incoming / offsetter;
        let result = divisor % 10;

        res[idx] = (result + 48) as u8;
        idx += 1;

        offsetter /= 10;
    }

    res
}

const HELLO_WORLD: &[u8] = b"Hello world port, from testing rust!!\n";
const SAMPLE_STACK: [u8;4] = [1,2,3,4];

#[no_mangle]
pub unsafe extern "C" fn not_main() -> ! {
    // let res = get_page_loc();

    // let le_bytes = res.to_le_bytes();
    // let byte_ref = le_bytes.as_ref();
    // write_string_to_port(byte_ref.as_ptr(), le_bytes.len(), 0x2000);


    // // let as_str = int_to_str(res);

    // // for i in as_str {
    // //     write_to_port(0x1000, i);
    // // }

    // // debug_line();

    // let as_str = dump_64_bit(res);

    // for i in as_str {
    //     write_to_port(0x1000, i);
    // }

    // debug_line();

    let mmio_location = (0x20000) as *mut u8;
    mmio_location.write_volatile(1);

    // write_to_port(0x1000, get_number(virt_home));

    // let playbound_ptr = 0x4000 as *mut u8;

    // // let mut stack = unsafe { core::slice::from_raw_parts_mut(playbound_ptr, 20) };
    // let mut stack = SAMPLE_STACK.clone();

    // // let mut stack = [0u8; 2];
    // let mut sum = 0;

    // debug_line();

    // for &c in HELLO_WORLD.iter() {
    //     write_to_port(0x1000, c);
    // }

    // write_string_to_port(HELLO_WORLD.as_ptr(), HELLO_WORLD.len(), 0x1000);

    // let mmio_location = (0x4000) as *mut u8;

    // for &letter in HELLO_WORLD.iter() {
    //     mmio_location.write_volatile(letter);
    // }

    // let mut hello_iter = HELLO_WORLD.chunks_exact(8);
    // while let Some(letter) = hello_iter.next() {
    //     core::ptr::copy_nonoverlapping(letter.as_ptr(), mmio_location, letter.len());
    // }

    // for &letter in hello_iter.remainder().iter() {
    //     mmio_location.write_volatile(letter);
    // }

    // for letter in HELLO_WORLD.chunks(8) {
    //     core::ptr::copy_nonoverlapping(letter.as_ptr(), mmio_location, letter.len());
    // }

    // write_bytes(b"Now using our fancy message\n");
    // write_bytes(HELLO_WORLD);

    // write_string("Printing is much easier now..\n");

    loop {}
}
