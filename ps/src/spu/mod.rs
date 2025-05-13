use crate::map;
pub struct SPU {
    pub registers: [u16; 256], // SPU has 512 bytes of registers (256 halfwords)
}

impl SPU {
    pub fn new() -> SPU {
        SPU { registers: [0; 256] }
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        let offset = ((address - map::SPU_START) & 0x3FF) >> 1;
        //println!("SPU Read: 0x{:08X}, offset: 0x{:04X}", address, offset);
        self.registers[offset as usize]
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        let offset = ((address - map::SPU_START) & 0x3FF) >> 1;
        //println!("SPU Write: 0x{:08X}, value: 0x{:04X}, offset: 0x{:04X}", address, value, offset);
        self.registers[offset as usize] = value;
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        let offset = ((address - map::SPU_START) & 0x3FF) as usize;
        println!(
            "SPU Byte Write: 0x{:08X}, value: 0x{:02X}, offset: 0x{:04X}",
            address, value, offset
        );

        // Get the corresponding halfword register
        let halfword_offset = offset & !1; // Clear the lowest bit to get the halfword boundary
        let halfword_index = halfword_offset >> 1; // Divide by 2 to get the index in registers array

        // Read the existing halfword
        let mut halfword = self.registers[halfword_index];

        // Update the appropriate byte in the halfword
        if (offset & 1) == 0 {
            // Even address: update the low byte
            halfword = (halfword & 0xFF00) | (value as u16);
        } else {
            // Odd address: update the high byte
            halfword = (halfword & 0x00FF) | ((value as u16) << 8);
        }

        // Write the updated halfword back
        self.registers[halfword_index] = halfword;
    }
}
