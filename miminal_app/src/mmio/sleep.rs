const MMIO_SLEEP_LOCATION: usize = 0x65001;

use crate::mmio::write_long_to_mmio;

pub unsafe fn sleep(duration: u64) {
    write_long_to_mmio(MMIO_SLEEP_LOCATION, duration)
}

