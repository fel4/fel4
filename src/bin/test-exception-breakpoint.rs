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
use core::sync::atomic::{AtomicUsize, Ordering};
use fel4::exit_qemu;
use x86_64::structures::idt::{ExceptionStackFrame, InterruptDescriptorTable};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

static BREAKPOINT_HANDLER_CALLED: AtomicUsize = AtomicUsize::new(0);

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(_: &mut ExceptionStackFrame) {
    BREAKPOINT_HANDLER_CALLED.fetch_add(1, Ordering::SeqCst);
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();

    x86_64::instructions::int3();

    match BREAKPOINT_HANDLER_CALLED.load(Ordering::SeqCst) {
        1 => serial_println!("ok"),
        0 => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was never called");
        },
        other => {
            serial_println!("failed");
            serial_println!("Breakpoint handler was called {} times", other);
        }
    }
    unsafe { exit_qemu(); }
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
