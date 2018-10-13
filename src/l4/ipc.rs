// Note structure must by 512-byte aligned (implies sizeof(IPCBuffer) == 512)
pub struct IPCBuffer {}

// TODO :: replace placeholder u32 types with correct types!
pub struct MessageInfo {
    label: u32,
    msg_length: u32,
    extra_caps: u32, // aka, 'number of capabilities
    caps_unwrapped: u32,
}
