cd ../miminal_app

cargo build --release

strip -s ./target/minimal_compile/release/miminal_app
# llvm-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 ../target/min_two/release/miminal_app ./min_img.bin
llvm-objcopy -I elf64-x86-64 -O binary ./target/minimal_compile/release/miminal_app ../vmm_rust/miminal_app
# llvm-objcopy -I elf64-x86-64 -O binary --binary-architecture=i386:x86-64 ../target/x86_64-unknown-none/release/miminal_app ./min_img.bin

# cp ./target/minimal_compile/release/miminal_app ../vmm_rust/miminal_app
