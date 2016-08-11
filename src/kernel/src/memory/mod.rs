pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::{PhysicalAddress, remap_the_kernel, test_paging};

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
