use crate::mmio::rng;

pub unsafe fn test_rng_device() {
    for _ in 0..10 {
        println!("Read {} from rng port", rng::get_rng());
    }
}
