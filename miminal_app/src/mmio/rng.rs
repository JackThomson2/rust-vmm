const MMIO_RNG_LOCATION: usize = 0x65000;

use crate::mmio::read_from_mmio;

pub unsafe fn get_rng() -> u64 {
    read_from_mmio(MMIO_RNG_LOCATION)
}

