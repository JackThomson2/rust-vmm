cd ../miminal_app

cargo build --release

strip -s ./target/minimal_compile/release/miminal_app

llvm-objcopy -I elf64-x86-64 -O binary ./target/minimal_compile/release/miminal_app ../vmm_rust/miminal_app

# cargo bootimage
# cp ./target/minimal_compile/release/bootimage-miminal_app.bin ../vmm_rust/miminal_app
