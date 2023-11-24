fn main() {
    println!("cargo:rerun-if-changed=src/*.S");
    println!("cargo:rerun-if-changed=linker.ld");
}
