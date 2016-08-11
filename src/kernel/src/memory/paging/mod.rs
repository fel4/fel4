use core::ops::{Deref, DerefMut};
use multiboot2::BootInformation;

use self::entry::*;
use self::mapper::Mapper;
use self::table::{Table, Level4};
use self::temporary_page::TemporaryPage;

use memory::{PAGE_SIZE, Frame, FrameAllocator};

mod entry;
mod mapper;
mod table;
mod temporary_page;

const ENTRY_COUNT: usize = 512;

const LOWER_HALF_MAX: usize = 0x0000_8000_0000_0000;
const HIGHER_HALF_MIN: usize = 0xffff_8000_0000_0000;
const INDEX_MASK: usize = 0o777;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: Mapper::new()
        }
    }
    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86::controlregs;

        let old_table = InactivePageTable {
            p4_frame: Frame::containing_address(
                unsafe { controlregs::cr3() } as usize
            ),
        };

        unsafe {
            controlregs::cr3_write(new_table.p4_frame.start_address() as u64);
        }
        old_table
    }
    pub fn with<F>(&mut self, table: &mut InactivePageTable, temporary_page: &mut TemporaryPage, f: F)
        where F: FnOnce(&mut Mapper) {
        use x86::{controlregs, tlb};

        let flush_tlb = || unsafe { tlb::flush_all() };

        {
            let backup = Frame::containing_address(unsafe { controlregs::cr3() } as usize);

            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITEABLE);
            flush_tlb();

            f(self);

            p4_table[511].set(backup, PRESENT | WRITEABLE);
            flush_tlb();
        }

        temporary_page.unmap(self);
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame, active_table: &mut ActivePageTable, temporary_page: &mut TemporaryPage)
        -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            table[511].set(frame.clone(), PRESENT | WRITEABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}


#[derive(Clone, Copy, Debug)]
pub struct Page {
    number: usize,
}

impl Page {

    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < LOWER_HALF_MAX || address >= HIGHER_HALF_MIN, "invalid address: 0x{:x}", address);
        Page { number: address / PAGE_SIZE }
    }

    pub fn start_address(&self) -> VirtualAddress {
        return self.number * PAGE_SIZE
    }

    fn p4_index(&self) -> usize {
        (self.number >> 27) & INDEX_MASK
    }

    fn p3_index(&self) -> usize {
        (self.number >> 18) & INDEX_MASK
    }

    fn p2_index(&self) -> usize {
        (self.number >> 9 ) & INDEX_MASK
    }

    fn p1_index(&self) -> usize {
        (self.number >> 0) & INDEX_MASK
    }
}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation)
    where A: FrameAllocator {

    let mut temporary_page = TemporaryPage::new(Page { number: 0xcafebabe }, allocator);
    let mut active_table = unsafe { ActivePageTable::new() };

    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");
        for section in elf_sections_tag.sections() {
            use self::entry::WRITEABLE;

            if !section.is_allocated() {
                // section is not loaded into memory.
                continue;
            }

            assert!((section.addr as usize) % PAGE_SIZE == 0, "sections need to be page aligned");

            println!("mapping section at addr: {:#x}, size: {:#x}", section.addr, section.size);

            let flags = EntryFlags::from_elf_section_flags(section);

            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }
        // identity map the vga buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, WRITEABLE, allocator);

        // identity map the multiboot info
        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, PRESENT, allocator);
        }
    });

    let old_table = active_table.switch(new_table);
    println!("NEW TABLE!");

    // remap the old p4 page and turn it into a guard page.
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address());
    active_table.unmap(old_p4_page, allocator);
    println!("guard page at {:#x}", old_p4_page.start_address());
}

pub fn test_paging<A>(allocator: &mut A) where A: FrameAllocator {
    //println!("obtaining reference to page table.");
    let mut page_table = unsafe { ActivePageTable::new() };
    //println!("got reference to page table!");

    // address 0 is mapped
    println!("Some = {:?}", page_table.translate(0));
     // second P1 entry
    println!("Some = {:?}", page_table.translate(4096));
    // second P2 entry
    println!("Some = {:?}", page_table.translate(512 * 4096));
    // 300th P2 entry
    println!("Some = {:?}", page_table.translate(300 * 512 * 4096));
    // second P3 entry
    println!("None = {:?}", page_table.translate(512 * 512 * 4096));
    // last mapped byte
    println!("Some = {:?}", page_table.translate(512 * 512 * 4096 - 1));

    // test page mapping
    let addr = 42 * 512 * 512 * 4096;
    let page = Page::containing_address(addr);
    let frame = allocator.allocate_frame().expect("no more frames");
    println!("None = {:?}, map to {:?}", page_table.translate(addr), frame);
    page_table.map_to(page, frame, EntryFlags::empty(), allocator);
    println!("Some = {:?}", page_table.translate(addr));
    println!("next free frame: {:?}", allocator.allocate_frame());

    // read data in mapped page.
    println!("{:#x}", unsafe { *(Page::containing_address(addr).start_address() as *const u64)} );

    // unmap the page.
    page_table.unmap(Page::containing_address(addr), allocator);
    println!("None = {:?}", page_table.translate(addr));

    // read data in (un-)mapped page.
    //println!("{:#x}", unsafe { *(Page::containing_address(addr).start_address() as *const u64)} );
}
