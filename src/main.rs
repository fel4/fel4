#![feature(abi_x86_interrupt)]
#![feature(const_fn)]
#![feature(panic_implementation)] // required for defining the panic handler
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
use fel4::interrupts::{self, PICS};
use x86_64::structures::idt::{ExceptionStackFrame,InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(fel4::gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[usize::from(interrupts::TIMER_INTERRUPT_ID)].set_handler_fn(timer_interrupt_handler);
        idt[usize::from(interrupts::PIC1_SPURIOUS_ID)].set_handler_fn(pic1_spurious_handler);
        idt[usize::from(interrupts::PIC2_SPURIOUS_ID)].set_handler_fn(pic2_spurious_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: &mut ExceptionStackFrame) {
    print!(".");
    unsafe { PICS.lock().notify_end_of_interrupt(interrupts::TIMER_INTERRUPT_ID); }
}

extern "x86-interrupt" fn pic1_spurious_handler(_stack_frame: &mut ExceptionStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(interrupts::PIC1_SPURIOUS_ID); }
}

extern "x86-interrupt" fn pic2_spurious_handler(_stack_frame: &mut ExceptionStackFrame) {
    unsafe { PICS.lock().notify_end_of_interrupt(interrupts::PIC2_SPURIOUS_ID); }
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");
    serial_println!("Hello, Host{}", "!");

    fel4::gdt::init();
    init_idt();
    unsafe { PICS.lock().initialize(); }

    x86_64::instructions::interrupts::enable();

    println!("It did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_implementation] // define a function that should be called on panic
#[no_mangle]
pub fn panic(
    _info: &PanicInfo
) -> ! {
    loop {}
}
