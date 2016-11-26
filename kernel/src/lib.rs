#![feature(alloc)]
#![feature(asm)]
#![feature(collections)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(unique)]
#![no_std]

extern crate alloc;
extern crate bit_field;
#[macro_use]
extern crate bitflags;
extern crate bump_allocator;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate lazy_static;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
#[macro_use]
extern crate x86;

#[macro_use]
mod debug;
mod interrupts;
mod memory;
mod sync;

fn divide_by_zero() {
    unsafe {
        asm!("mov dx, 0; div dx" ::: "ax", "dx" : "volatile", "intel")
    }
}

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    debug::vga::clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };

    println!("Hello, World!");
    // enable the no-execute bit.
    enable_nxe_bit();
    // enable kernel-mode write protection.
    enable_write_protect_bit();

    // initialize memory layout.
    memory::init(boot_info);

    // initialize IDT.
    interrupts::init();

    // cause a breakpoint.
    unsafe { int!(3) };
    // test divide by zero.
    //divide_by_zero();
    // test invalid opcode.
    //unsafe { asm!("ud2") }
    // provoke a page fault
    //unsafe { *(0xdeadbeaf as *mut u64) = 42 };

    println!("It didn't crash!");

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("It allocated and didn't crash!");

    loop {}
}

fn enable_nxe_bit() {
    use x86::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::controlregs::{cr0, cr0_write};

    let wp_bit = 1 << 16;
    unsafe { cr0_write(cr0() | wp_bit) };
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    unsafe {
        debug::vga::print_error(format_args!("\n\nPANIC in {} at line {}:", file, line));
        debug::vga::print_error(format_args!("      {}", fmt));
    }
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
