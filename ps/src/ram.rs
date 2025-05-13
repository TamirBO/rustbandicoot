use super::map;
//use super::playstation::Addressable;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Clone)]
pub struct Ram {
    pub data: Box<[u8]>,
}

/*impl fmt::Display for Ram{
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ram = String::new();
        for i in self.data.to_vec(){
        write!(ram, "{} ", i)?;
        }
        write!(f, "{}", ram)

    }
}*/
//impl Addressable for Ram {}
impl Ram {
    pub fn new() -> Ram {
        // Use the actual size you want (2MB standard, 8MB for expanded)
        let ram_size = map::RAM_SIZE; // 2MB or 8MB based on your configuration
        Ram { data: vec![0; ram_size as usize].into_boxed_slice() }
    }

    fn get_offset(&self, addr: u32) -> usize {
        // Mask based on the actual RAM size
        let mask = (map::RAM_SIZE - 1) as u32;
        let masked_addr = addr & mask;
        (masked_addr - map::RAM_START) as usize
    }

    pub fn read8(&self, addr: u32) -> u8 {
        let offset = self.get_offset(addr);
        self.data[offset]
    }

    pub fn read16(&self, addr: u32) -> u16 {
        let offset = self.get_offset(addr);
        (self.data[offset] as u16) | ((self.data[offset + 1] as u16) << 8)
    }

    pub fn read32(&self, addr: u32) -> u32 {
        let offset = self.get_offset(addr);
        (self.data[offset] as u32)
            | ((self.data[offset + 1] as u32) << 8)
            | ((self.data[offset + 2] as u32) << 16)
            | ((self.data[offset + 3] as u32) << 24)
    }

    pub fn write8(&mut self, addr: u32, byte: u8) {
        let offset = self.get_offset(addr);
        self.data[offset] = byte;
    }

    pub fn write16(&mut self, addr: u32, halfword: u16) {
        let offset = self.get_offset(addr);
        self.data[offset] = (halfword & 0xFF) as u8;
        self.data[offset + 1] = ((halfword >> 8) & 0xFF) as u8;
    }

    pub fn write32(&mut self, addr: u32, word: u32) {
        let offset = self.get_offset(addr);
        self.data[offset] = (word & 0xFF) as u8;
        self.data[offset + 1] = ((word >> 8) & 0xFF) as u8;
        self.data[offset + 2] = ((word >> 16) & 0xFF) as u8;
        self.data[offset + 3] = ((word >> 24) & 0xFF) as u8;
    }
    //for debugging
    pub fn dump_region(&self, start_offset: usize, length: usize) -> String {
        let mut output = String::new();
        for i in 0..length {
            if i % 16 == 0 {
                if i > 0 {
                    output.push('\n');
                }
                output.push_str(&format!("{:08X}: ", start_offset + i));
            }
            output.push_str(&format!("{:02X} ", self.data[start_offset + i]));
        }
        output
    }

    //TODO Formatting Dumped file in a readable way.
    pub fn dump(&self, path: String) -> File {
        let mut output = File::create(path).unwrap();
        let slice: &[u8] = &self.data;
        for i in 0..slice.len() {
            let mut line: String;
            if i % 0x10 == 0 {
                write!(output, "\n0x{:08X}: ", i).unwrap();
            }
            write!(output, "{:02X} ", slice[i]).unwrap();
        }
        output
    }
}
