#![allow(dead_code)]

#[macro_use]
extern crate nom;

pub const SECTOR_SIZE: usize = 2048;

#[allow(non_camel_case_types)]
enum NumberFormat {
    int8(u8),
    sint8(i8),
    int16_LSB(u16),
    int16_MSB(u16),
    int16_LSBMSB(u16, u16),
    sint16_LSB(i16),
    sint16_MSB(i16),
    sint16_LSBMSV(i16,i16),
    int32_LSB(u32),
    int32_MSB(u32),
    int32_LSBMSB(u32,u32),
    sint32_LSB(i32),
    sint32_MSB(i32),
    sint32_LSBMSB(i32,i32)
}

struct AString(String);

impl AString {
    pub fn is_valid_byte(b: u8) -> bool {
        match b {
            b'A'...b'Z' => true,
            b'0'...b'9' => true,
            b'_' => true,
            _ => false
        }
    }
}

struct DString(String);

impl DString {
    pub fn is_valid_byte(b: u8) -> bool {
        if AString::is_valid_byte(b) {
            true
        } else {
            match b {
                b'!' | b'"' | b'%' | b'&' | b'\'' | b'(' | b')' |
                b'*' | b'+' | b',' | b'-' | b'.' | b'/' | b':' |
                b';' | b'<' | b'=' | b'>' | b'?' => true,
                _ => false
            }
        }
    }
}

struct DateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    second_part: u8,
    tmz_offset: u8
}

impl DateTime {
    fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8, sp: u8, tmz: u8) -> DateTime {
        DateTime {
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second,
            second_part: sp,
            tmz_offset: tmz
        }
    }

    pub fn now() -> DateTime { unimplemented!(); }

    //named!(pub parse<&[u8], DateTime>,
    //    do_parse!(
    //        year:
    //    )
    //);
}



struct Sector(pub [u8; SECTOR_SIZE]);


pub struct ImageFile {

}


named!(take_sector, take!(SECTOR_SIZE));


#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn astring_valid_byte() {
        assert!(AString::is_valid_byte(b'A'));
    }

    #[test]
    #[should_panic]
    fn astring_invalid_byte() {
        assert!(AString::is_valid_byte(b'!'));
    }

    #[test]
    fn dstring_valid_byte() {
        assert!(DString::is_valid_byte(b'!'));
    }

    #[test]
    #[should_panic]
    fn dstring_invalid_byte() {
        assert!(DString::is_valid_byte(b'a'));
    }


    #[test]
    #[ignore]
    fn test_file_sanity_check() {
        let mut buffer: Vec<u8> = Vec::new();
        let mut file = File::open("../../shard.iso").unwrap();
        let len = file.read_to_end(&mut buffer).unwrap();
        let sector_sz: usize = SECTOR_SIZE as usize;
        let sector_count = len / sector_sz;
        println!("file size: {}", len);
        println!("number of sectors: {}", sector_count);
        assert_eq!(len, sector_count * sector_sz);
    }
}
