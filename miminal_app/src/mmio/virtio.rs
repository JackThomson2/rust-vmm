const MMIO_VIRTIO_LOCATION: usize = 0x65002;

use crate::mmio::write_long_to_mmio;

pub unsafe fn mmio_write_loc(duration: u64) {
    write_long_to_mmio(MMIO_VIRTIO_LOCATION, duration)
}
