use crate::map::GPU_REGISTERS_START;

pub struct GPU {
    GPO: u32,
    GP1: u32,
}
impl GPU {
    pub fn new() -> Self {
        GPU { GPO: 0, GP1: 0 }
    }

    pub fn read32(&self, addr: u32) -> u32 {
        let address = addr - GPU_REGISTERS_START;
        match address {
            4 => 0x10000000, //hack to bypass the loop
            _ => 0,
        }
    }
}
