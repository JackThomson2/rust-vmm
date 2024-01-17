#[allow(dead_code)]
pub unsafe fn test_invalid_read() {
    println!("Test invalid memory location");

    let mmio_location = (0x1264001) as *mut u8;
    println!("Found data at: {}", mmio_location.read());
    println!("Wrote to invalid location!\n");
}
