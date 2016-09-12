use super::{ActivePageTable, Page, VirtualAddress};
use super::table::{Level1, Table};
use memory::{Frame, FrameAllocator};

pub struct TemporaryPage {
    page: Page,
    allocator: TinyAllocator
}

impl TemporaryPage {
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable)
        -> VirtualAddress {
        use super::entry::WRITEABLE;

        assert!(active_table.translate_page(self.page).is_none(),
                "temporary_page is already mapped");
        active_table.map_to(self.page, frame, WRITEABLE, &mut self.allocator);
        self.page.start_address()
    }

    pub fn map_table_frame(&mut self, frame: Frame, active_table: &mut ActivePageTable)
        -> &mut Table<Level1> {
        unsafe { &mut *(self.map(frame, active_table) as *mut Table<Level1>) }
    }

    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator {
        TemporaryPage {
            page: page,
            allocator: TinyAllocator::new(allocator),
        }
    }

    pub fn unmap(&mut self, active_table: &mut ActivePageTable) {
        active_table.unmap(self.page, &mut self.allocator);
    }
}

struct TinyAllocator([Option<Frame>; 3]);

impl FrameAllocator for TinyAllocator {
    fn allocate_frame(&mut self) -> Option<Frame> {
        for frame_option in &mut self.0 {
            if frame_option.is_some() {
                return frame_option.take();
            }
        }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        for frame_option in &mut self.0 {
            if frame_option.is_none() {
                *frame_option = Some(frame);
                return;
            }
        }
        panic!("The allocator can only hold 3 frames!");
    }
}

impl TinyAllocator {
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}
