#[derive(Debug, PartialEq)]
pub enum TranslationError {
    InsufficientBits(u32),
}
pub type TranslationResult = Result<(usize, u32), TranslationError>;

pub struct CPtr(usize);

impl CPtr {
    pub const fn num_bits() -> u32 {
        (::core::mem::size_of::<usize>() * 8) as u32
    }
    pub fn translate_bits(&self, start_bit: u32, bits_to_translate: u32) -> TranslationResult {
        let bits_needed = start_bit + bits_to_translate;
        if bits_needed > Self::num_bits() {
            Err(TranslationError::InsufficientBits(
                bits_needed - Self::num_bits(),
            ))
        } else {
            let mask = 2usize.pow(bits_to_translate) - 1;
            let mask = mask.rotate_right(bits_needed);
            let ret = (self.0 & mask).rotate_left(bits_needed);
            Ok((ret, Self::num_bits() - bits_needed))
        }
    }
}

enum Badge {
    None,
}

/// defines guard bits for a particular CNode
struct Guard {
    size: u32,
    value: u32,
}

impl Guard {}

pub struct CNode {
    guard: Guard,
    radix: u32,
    badge: Badge,
}

impl CNode {
    /// seL4_CNode_CancelBadgedSends() cancels any outstanding sends that use the same
    /// badge and object as the specified capability.
    pub fn cancel_badged_sends() {
        unimplemented!();
    }
    /// seL4_CNode_Copy() is similar to seL4_CNode_Mint(), but the newly created capability
    /// has the same badge and guard as the original.
    pub fn copy() {
        unimplemented!();
    }
    /// seL4_CNode_Delete() removes a capability from the specified slot.
    pub fn delete() {
        unimplemented!();
    }
    /// seL4_CNode_Mint() creates a new capability in a specified CNode slot from an existing
    /// capability. The newly created capability may have fewer rights than the original
    /// and a different guard (see Section 3.3.1). seL4_CNode_Mint() can also create a
    /// badged capability (see Section 4.2.1) from an unbadged one.
    pub fn mint() {
        unimplemented!();
    }
    /// seL4_CNode_Move() moves a capability between two specified capability slots. You
    /// cannot move a capability to the slot in which it is currently.
    pub fn move_cnode() {
        unimplemented!();
    } // move is a keyword in rust.
    /// seL4_CNode_Mutate() can move a capability similarly to seL4_CNode_Move() and also
    /// reduce its rights similarly to seL4_CNode_Mint(), although without an original
    /// copy remaining.
    pub fn mutate() {
        unimplemented!();
    }
    /// seL4_CNode_Revoke() is equivalent to calling seL4_CNode_Delete() on each derived
    /// child of the specified capability. It has no eect on the capability itself, except in
    /// very specific circumstances outlined in Section 3.2.
    pub fn revoke() {
        unimplemented!();
    }
    /// seL4_CNode_Rotate() moves two capabilities between three specified capability slots.
    /// It is essentially two seL4_CNode_Move() invocations: one from the second specified
    /// slot to the first, and one from the third to the second. The first and third
    /// specified slots may be the same, in which case the capability in it is swapped with
    /// the capability in the second slot. The method is atomic; either both or neither
    /// capabilities are moved.
    pub fn rotate() {
        unimplemented!();
    }
    /// seL4_CNode_SaveCaller() moves a kernel-generated reply capability of the current
    /// thread from the special TCB slot it was created in, into the designated CSpace
    /// slot.
    pub fn save_caller() {
        unimplemented!();
    }
}

struct Translation {
    ptr: CPtr,
    rem_bits: u32,
}

impl Translation {
    pub fn read_bits(&mut self, num_bits: u32) -> Option<usize> {
        if num_bits > self.rem_bits {
            None
        } else {
            let start_bit = CPtr::num_bits() - self.rem_bits;
            // generate a mask for the requested bits.
            let mask = 2usize.pow(num_bits) - 1;
            // and shift it into the correct position.
            let mask = mask.rotate_right(start_bit + num_bits);

            // extract the requested bits.
            let ret = (self.ptr.0 & mask).rotate_left(start_bit + num_bits);

            // decremented the remaining bits.
            self.rem_bits -= num_bits;

            Some(ret)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cptr_translate_simple() {
        let cptr = CPtr(0x1234567812345678);
        let ret = cptr.translate_bits(0, 8);
        assert!(ret.is_ok());
        let (val, rem) = ret.unwrap();
        assert_eq!(val, 0x12);
        assert_eq!(rem, CPtr::num_bits() - 8);
    }

    #[test]
    fn cptr_translate_insufficent() {
        let cptr = CPtr(0x0);
        let ret = cptr.translate_bits(64, 1);
        assert!(ret.is_err());
        assert_eq!(ret.err(), Some(TranslationError::InsufficientBits(1)));
    }
}
