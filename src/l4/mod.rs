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

// core sel4 syscalls

/// Send to a capability
pub fn send(dest: caps::CPtr, msg_info: ipc::MessageInfo) {
    unimplemented!();
}
/// Block until a message is received on an endpoint.
pub fn recv(src: caps::CPtr, sender: Option<*mut usize>) -> ipc::MessageInfo {
    unimplemented!();
}
/// Call a capability.
pub fn call(dest: caps::CPtr, msg_info: ipc::MessageInfo) -> ipc::MessageInfo {
    unimplemented!();
}
/// Perform a send to a one-o reply capability stored when the thread was last called.
pub fn reply(msg_info: ipc::MessageInfo) {
    unimplemented!();
}
/// Perform a non-blocking send to a capability.
pub fn nb_send(dest: caps::CPtr, msg_info: ipc::MessageInfo) {
    unimplemented!();
}
/// Perform a reply followed by a receive in one system call
pub fn reply_recv(
    dest: caps::CPtr,
    msg_info: ipc::MessageInfo,
    sender: Option<*mut usize>,
) -> ipc::MessageInfo {
    unimplemented!();
}
/// Receive a message from an endpoint but do not block in the case that no messages are
/// pending.
pub fn nb_recv(src: caps::CPtr, sender: Option<*mut usize>) -> ipc::MessageInfo {
    unimplemented!();
}
/// Donate the remaining timeslice to a thread of the same priority.
pub fn yield_thread() {
    unimplemented!();
} // yield is a keyword in rust.
