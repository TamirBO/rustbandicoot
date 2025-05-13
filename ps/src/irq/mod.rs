use crate::map;

enum Interrupt {
    Vblank = 0,
    GPU = 1,
    CDROM = 2,
    DMA = 3,
    Timer0 = 4,
    Timer1 = 5,
    Timer2 = 6,
    ControllerMemCard = 7,
    SIO = 8,
    SPU = 9,
    Lightpen = 10,
}

pub struct IRQController {
    pub status: u32, // Current status of IRQs
    pub mask: u32,   // Mask of enabled IRQs
}

impl IRQController {
    pub fn new() -> IRQController {
        IRQController { status: 0, mask: 0 }
    }

    pub fn get_status(&self) -> u32 {
        self.status
    }
    pub fn get_mask(&self) -> u32 {
        self.mask
    }

    pub fn acknowledge(&mut self, value: u32) {
        // When writing to I_STAT, 0s clear the bits, 1s leave them unchanged
        self.status &= value;
    }

    pub fn set_mask(&mut self, value: u32) {
        self.mask = value;
    }
    pub fn interrupt_pending(&self) -> bool {
        return (self.status & self.mask) != 0;
    }
}
