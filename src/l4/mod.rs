pub mod caps;
pub mod ipc;
pub mod thread;

pub enum MemoryType {
    Device,
    General,
}

pub struct Untyped {}

impl Untyped {
    pub fn retype() {
        unimplemented!();
    }
}
