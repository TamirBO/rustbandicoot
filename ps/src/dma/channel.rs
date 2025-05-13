use modular_bitfield::bitfield;
use modular_bitfield::prelude::{B1, B2, B3, B5, B6};

pub struct Channel {
    //device: DMAPort,

    pub base_address: u32,
    pub block_control: u32,

    pub control_register: ChannelControlRegister,
}

impl Channel {
    pub fn new() -> Channel {
        Channel{
            base_address: 0,
            block_control: 0,
            control_register: ChannelControlRegister{register: 0}
        }
    }
}
pub enum DMAPort {
    MDECIN = 0,
    MDECOUT = 1,
    GPU = 2,
    CDROM = 3,
    SPU = 4,
    PIO = 5,
    OTC = 6,
    Registers = 7
}

pub union ChannelControlRegister {
    pub bits: ChannelControlBits,
    pub register: u32,
}
#[bitfield]
#[derive(Copy, Clone)]
pub struct ChannelControlBits {
    // 0 - device to RAM 1 - RAM to device
    transfer_direction: B1,
    // 0 - Increase 1 - Decrease
    address_increment_or_decrement: B1,
    #[skip]
    unused: B6,

    chopping_enabled: bool,
    /*
    0=Burst (transfer data all at once after DREQ is first asserted)
    1=Slice (split data into blocks, transfer next block whenever DREQ is asserted)
    2=Linked-list mode
    3=Reserved
    */
    transfer_mode: B2,
    #[skip]
    unused: B5,
    //  (1 << N words)
    chopping_dma_window_size: B3,

    #[skip]
    unused: B1,
    //  (1 << N cycles)
    chopping_cpu_window_size: B3,
    #[skip]
    unused: B1,
    //(0=stopped/completed, 1=start/busy)
    start_transfer: B1,
    #[skip]
    unused: B3,
    start_trigger: B1,
    #[skip]
    unused: B3,
}
