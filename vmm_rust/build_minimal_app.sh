cd ../miminal_app

cargo build --release

strip -s ./target/minimal_compile/release/miminal_app

llvm-objcopy -I elf64-x86-64 -O binary ./target/minimal_compile/release/miminal_app ../vmm_rust/miminal_app
