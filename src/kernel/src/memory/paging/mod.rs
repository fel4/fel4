use core::ptr::Unique;
use x86;

use self::entry::*;
use self::table::{Table, Level4};

use memory::{PAGE_SIZE, Frame, FrameAllocator};

mod entry;
mod table;

const ENTRY_COUNT: usize = 512;

const LOWER_HALF_MAX: usize = 0x0000_8000_0000_0000;
const HIGHER_HALF_MIN: usize = 0xffff_8000_0000_0000;
const INDEX_MASK: usize = 0o777;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct ActivePageTable {
    p4: Unique<Table<Level4>>,
}

pub struct Page {
    number: usize,
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new(table::P4),
        }
    }

    fn p4(&self) -> &Table<Level4> { unsafe { self.p4.get() } }

    fn p4_mut(&mut self) -> &mut Table<Level4> { unsafe { self.p4.get_mut() } }

    pub fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator {
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;

        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        use self::entry::HUGE_PAGE;

        let p3 = self.p4().next_table(page.p4_index());

        // fallback logic to handle huge pages.
        let huge_page = || {
            //println!("huge_page fallback");
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];

                // 1 GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        // address *must* be 1 GiB aligned ..
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p2_index() *
                                ENTRY_COUNT + page.p1_index(),
                        });
                    }
                }

                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    // 2 MiB page?
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(HUGE_PAGE) {
                            // address *must* be 2 MiB aligned ..
                            assert!(start_frame.number % ENTRY_COUNT == 0);
                            return Some(Frame {
                                number: start_frame.number + page.p1_index(),
                            });
                        }
                    }
                }
                None
            })
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(huge_page)
    }

    fn unmap<A>(&mut self, page: Page, allocator: &mut A) where A: FrameAllocator {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages.");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unsed();
        unsafe {
            x86::tlb::flush(page.start_address());
        }
        //allocator.deallocate_frame(frame);
    }

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
