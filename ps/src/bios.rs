use crate::map; //playstation::Addressable};
pub struct BIOS {
    pub data: Box<[u8]>,
}
//impl Addressable for BIOS {}

impl BIOS {
    pub fn new(bin: Box<[u8]>) -> BIOS {
        // Unbreakable check!
        if bin.len() == map::BIOS_SIZE as usize {
            BIOS { data: bin }
        } else {
            panic!("Corrupted Bios");
        }
    }

    fn get_offset(&self, addr: u32) -> usize {
        // BIOS is typically 512KB, so mask with 0x7FFFF
        let masked_addr = addr & 0x7FFFF;
        masked_addr as usize
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
}
