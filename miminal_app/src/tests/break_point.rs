pub unsafe fn test_sidt_breakpoint() {
    println!("IDT is at {:#?}", x86_64::instructions::tables::sidt());
    println!("Testing a debug breakpoint");
    x86_64::instructions::interrupts::int3();
}
