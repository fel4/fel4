use multiboot2::BootInformation;

pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::{PhysicalAddress, remap_the_kernel, test_paging};

use super::sync;

mod area_frame_allocator;
mod paging;

pub const PAGE_SIZE: usize = 4096;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
    pub fn start_address(&self) -> PhysicalAddress { self.number * PAGE_SIZE }
    fn clone(&self) -> Frame { Frame { number: self.number }}
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }
}

pub struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

mod frame_block_consts {
    pub const BLOCK_SIZE_MASK: u64 = 0xFF;
    pub const FRAME_FLAGS_MASK: u64 = 0xFF00;
    pub const ADDRESS_MASK: u64 = 0xFFFF_FFFF_FFFF_0000;
}

// A frameblock represents a block of contiguous frames which share the same attributes.
// it's laid out according to the following rules :
// upper 48 bits -- starting frame number, aligned to 4k ( ie <real_address> = <start_address> * 0x1000 )
// bits 49-56 -- frame flags ( reserved; bit 49 == "higher half flag" )
// bits 57-64 -- block size ( power-of-two size of block.)
#[derive(PartialEq, Eq)]
pub struct FrameBlock(u64);

impl FrameBlock {


    pub fn block_size(&self) -> u64 {
        1 << (self.0 & frame_block_consts::BLOCK_SIZE_MASK)
    }

    pub fn start_address(&self) -> PhysicalAddress {
        ((self.0 & frame_block_consts::ADDRESS_MASK) >> 16) as usize
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

static INIT_LOCK: sync::OnceGuard = sync::OnceGuard::new();

pub fn init(boot_info: &BootInformation) {
    sync::call_once(&INIT_LOCK, || {
        println!("memory areas:");
        let memory_map_tag = boot_info.memory_map_tag()
            .expect("Memory map tag required");
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

        let kernel_start = elf_sections_tag.sections()
            .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap();
        let kernel_end = elf_sections_tag.sections()
            .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max().unwrap();

        let multiboot_start = boot_info.start_address();
        let multiboot_end = boot_info.end_address();
        println!("    kernel_start: {:#x}, kernel_end: {:#x}", kernel_start, kernel_end);
        println!("    multiboot_start: {:#x}, multiboot_end: {:#x}", multiboot_start, multiboot_end);

        let mut frame_allocator = AreaFrameAllocator::new(kernel_start as usize, kernel_end as usize,
            multiboot_start, multiboot_end, memory_map_tag.memory_areas());

        println!("{:?}", frame_allocator.allocate_frame());

        for i in 1..10 {
            if let Some(frame) = frame_allocator.allocate_frame() {
                println!("Frame {} at 0x{:x}", i, frame.start_address());
            }
        }

        //memory::test_paging(&mut frame_allocator);
        let mut active_table = remap_the_kernel(&mut frame_allocator, boot_info);

        use self::paging::{Page, WRITEABLE};
        use bump_allocator::{HEAP_START, HEAP_SIZE};

        let heap_start_page = Page::containing_address(HEAP_START);
        let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE - 1);

        for page in Page::range_inclusive(heap_start_page, heap_end_page) {
            active_table.map(page, paging::WRITEABLE, &mut frame_allocator);
        }
    });
}
