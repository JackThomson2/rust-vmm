#[allow(dead_code)]
pub unsafe fn test_stack_overflows() {
    println!("Triggering a stack overflow");

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
        volatile::Volatile::new(0).read();
    }

    // trigger a stack overflow
    stack_overflow();
    println!("Past the stack overflow now somehow");
}
