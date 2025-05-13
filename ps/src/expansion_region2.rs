#[derive(Copy, Clone)]
pub struct Expansion_Region_2 {
    post: u8,
}

impl Expansion_Region_2 {
    pub fn new() -> Expansion_Region_2 {
        Expansion_Region_2 { post: 0 }
    }
    pub fn write_byte(&mut self, addr: u32, byte: u8) {
        if addr == 0x1f802041 {
            self.post = byte;
        }
    }
    pub fn read_byte(&self, addr: u32) -> u8 {
        return 0;
    }
}
