#![feature(const_fn)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros))]

#[macro_use]
extern crate fel4;

use core::panic::PanicInfo;
use fel4::exit_qemu;

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("ok");

    unsafe {
        exit_qemu();
    }
    loop {}
}

#[cfg(not(test))]
#[panic_handler] // define a function that should be called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
