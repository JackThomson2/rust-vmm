use lazy_static::lazy_static;
use x86_64::{structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode}, set_general_handler, registers::control::Cr2};

use crate::gdt;

fn my_general_handler(
   stack_frame: InterruptStackFrame,
   index: u8,
   error_code: Option<u64>,
) {
    // println!("Stack frame {:#?}, index: {}, error code: {:?}", stack_frame, index, error_code)
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        // set_general_handler!(&mut idt, my_general_handler);

        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_page_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt.invalid_tss.set_handler_fn(general_fault);
        idt.cp_protection_exception.set_handler_fn(general_fault);

        idt.debug.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(breakpoint_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.invalid_opcode.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(breakpoint_handler);
        idt.bound_range_exceeded.set_handler_fn(breakpoint_handler);
        idt.non_maskable_interrupt.set_handler_fn(breakpoint_handler);

        idt.general_protection_fault.set_handler_fn(general_protection_fault);
        idt.alignment_check.set_handler_fn(general_fault);

        idt
    };
}

pub unsafe fn init_idt() {
    // IDT.reset();
    IDT.load();
}

extern "x86-interrupt" fn general_protection_fault(
    stack_frame: InterruptStackFrame,
    error: u64)
{
    println!("Got protection general fault error {error}");
}

extern "x86-interrupt" fn general_fault(
    stack_frame: InterruptStackFrame,
    error: u64)
{
    println!("Got general fault error {error}");
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    fault_code: PageFaultErrorCode)
{
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Got fault code {fault_code:?}. Stack: {stack_frame:#?}");
}

extern "x86-interrupt" fn double_page_fault_handler(
    stack_frame: InterruptStackFrame,
    code: u64) -> !
{
    panic!("Got error code: {}\n {:#?}", code, stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("Got breakpoint.\n{stack_frame:#?}");
}
