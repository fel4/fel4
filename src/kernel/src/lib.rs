#![feature(const_fn)]
#![feature(lang_items)]
#![feature(unique)]
#![no_std]

#[macro_use]
extern crate bitflags;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate x86;

use self::memory::FrameAllocator;

#[macro_use]
mod debug;
mod memory;

#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

    debug::vga::clear_screen();

    let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("Hello, World!");
    println!("memory areas:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: 0x{:x}, length: 0x{:x}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf sections tag required");

    println!("kernel sections");
    //for section in elf_sections_tag.sections() {
    //    println!("      addr: 0x{:x}, size: 0x{:x}, flags: 0x:{:x}",
    //        section.addr, section.size, section.flags);
    //}

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);
    println!("    kernel_start: 0x{:x}, kernel_end: 0x{:x}", kernel_start, kernel_end);
    println!("    multiboot_start: 0x{:x}, multiboot_end: 0x{:x}", multiboot_start, multiboot_end);

    let mut frame_allocator = memory::AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize,
        multiboot_start, multiboot_end, memory_map_tag.memory_areas());

    println!("{:?}", frame_allocator.allocate_frame());

    for i in 1..10 {
        if let Some(frame) = frame_allocator.allocate_frame() {
            println!("Frame {} at 0x{:x}", i, frame.start_address());
        }
    }

    // enable the no-execute bit.
    enable_nxe_bit();
    // enable kernel-mode write protection.
    enable_write_protect_bit();
    //memory::test_paging(&mut frame_allocator);
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    println!("It didn't crash!");

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
