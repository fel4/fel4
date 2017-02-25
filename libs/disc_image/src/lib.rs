use std::io;

#[macro_use]
extern crate nom;

pub const SECTOR_SIZE: usize = 2048;

struct Sector(pub [u8; SECTOR_SIZE]);

struct SectorIterator<'a> {
    data_source: &'a io::Read,
    buffer: Vec<u8>
}

pub struct ImageFile {

}

impl<'a> Iterator for SectorIterator<'a> {
    type Item = Sector;

    fn next(&mut self) -> Option<Self::Item> {
        while self.need {
        }
        None
    }
}

named!(take_sector, take!(SECTOR_SIZE));


#[cfg(test)]
mod tests {
    #[test]
    fn test_file_sanity_check() {
        use super::SECTOR_SIZE;

        let data = include_bytes!("../../../shard.iso");
        let len = data.len();
        let sector_sz: usize = SECTOR_SIZE as usize;
        let sector_count = len / sector_sz;
        println!("file size: {}", len);
        println!("number of sectors: {}", sector_count);
        assert_eq!(len, sector_count * sector_sz);
    }
}
