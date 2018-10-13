#![feature(abi_x86_interrupt)]
#![feature(const_fn)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros))]

#[macro_use]
extern crate fel4;
#[macro_use]
extern crate lazy_static;
extern crate x86_64;

use core::panic::PanicInfo;
use fel4::exit_qemu;
use x86_64::structures::idt::{ExceptionStackFrame,InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(fel4::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(_stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    serial_println!("ok");
    unsafe { exit_qemu(); }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {

    fel4::gdt::init();
    init_idt();

    #[allow(unconditional_recursion)]
    fn stack_overflow() { stack_overflow(); }

    stack_overflow();
    loop {}
}

#[cfg(not(test))]
#[panic_handler] // define a function that should be called on panic
fn panic(
    _info: &PanicInfo
) -> ! {
    loop {}
}
