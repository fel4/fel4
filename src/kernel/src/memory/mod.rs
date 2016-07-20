pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::{PhysicalAddress, test_paging};

mod area_frame_allocator;
mod paging;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const PAGE_SIZE: usize = 4096;

impl Frame {
    pub fn start_address(&self) -> PhysicalAddress { self.number * PAGE_SIZE }
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
