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

pub struct CNode {}

struct Guard {
    offset: u32,
    size: u32,
    value: u32,
}

impl Guard {}

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
