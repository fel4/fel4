#![feature(const_fn)]
#![feature(lang_items)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate volatile;

#[macro_use]
mod vga_buffer;

#[lang = "panic_fmt"] // define a function that should be called on panic
#[no_mangle]
pub extern "C" fn rust_begin_panic(
    _msg: core::fmt::Arguments,
    _file: &'static str,
    _line: u32,
    _column: u32,
) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    //write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    println!("Hello, World{}", "!");
    loop {}
}