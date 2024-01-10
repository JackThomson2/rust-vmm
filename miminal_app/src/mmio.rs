#[inline]
pub unsafe fn write_to_mmio_port(location: usize, data: u8) {
    let ptr = location as *mut u8;
    ptr.write_volatile(data);
}

#[inline]
pub unsafe fn write_long_to_mmio(location: usize, data: u64) {
    let data_arr = data.to_ne_bytes();
    core::ptr::copy(data_arr.as_ptr(), location as *mut u8, data_arr.len());
}

pub unsafe fn write_silce_to_mmio_port(location: usize, data: &[u8]) {
    for letter in data.chunks(8) {
        core::ptr::copy(letter.as_ptr(), location as *mut u8, letter.len());
    }
}

#[inline]
pub unsafe fn read_from_mmio(location: usize) -> u64 {
    let ptr = location as *mut u64;
    ptr.read_volatile()
}
