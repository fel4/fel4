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
use x86_64::structures::idt::{ExceptionStackFrame,InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");
    serial_println!("Hello, Host{}", "!");

    init_idt();

    x86_64::instructions::int3();

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
