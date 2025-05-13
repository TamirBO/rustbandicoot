use crate::dma::channel::DMAPort::{CDROM, GPU, MDECIN, MDECOUT, OTC, PIO, Registers, SPU};
use crate::dma::channel::{Channel, DMAPort};
use crate::map::DMA_REGISTERS_START;
use modular_bitfield::bitfield;
use modular_bitfield::prelude::*;

pub mod channel;

pub struct DMA {
    // Control Register
    control: ControlRegister,
    // Interrupt Register
    interrupt: InterruptRegister,

    channels: [Channel; 7],
}

impl DMA {
    pub fn new() -> DMA {
        DMA {
            control: ControlRegister { register: 0x07654321 },
            interrupt: InterruptRegister { register: 0 },
            channels: [
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
                Channel::new(),
            ],
        }
    }

    pub unsafe fn read32(&self, addr: u32) -> u32 {
        let offset = addr - DMA_REGISTERS_START;
        let (channel, reg) = dma_map(addr);

        match channel {
            0..=6 => match reg {
                0x0 => self.channels[channel].base_address,
                0x4 => self.channels[channel].block_control,
                0x8 => self.channels[channel].control_register.register,
                _ => panic!("unhandled DMA write {:08x}", addr),
            },

            7 => match reg {
                0x0 => self.control.register,
                0x4 => self.interrupt.register,
                _ => panic!("unhandled DMA write {:08x} ", addr),
            },
            _ => panic!("unhandled DMA write {:08x}", addr),
        }
    }

    pub fn write32(&mut self, addr: u32, val: u32) {
        let (channel, reg) = dma_map(addr);
        match channel {
            0..=6 => match reg {
                0x0 => {
                    self.channels[channel].base_address = val;
                    println!("DMA write base address {:08x} value: {:08x}, channel: {}", addr, val, channel);
                },
                0x4 => {
                    self.channels[channel].block_control = val;
                    println!("DMA write block control {:08x} value: {:08x}, channel: {}", addr, val, channel);
                },
                0x8 => {
                    self.channels[channel].control_register.register = val;
                    println!("DMA write control register {:08x} value: {:08x}, channel: {}",  addr,val, channel);
                },
                _ => panic!("unhandled DMA write {:08x} value: {:08x}", addr, val),
            },

            7 => match reg {
                0x0 => self.control.register = val,
                0x4 => self.interrupt.register = val,
                _ => panic!("unhandled DMA write {:08x} value: {:08x}", addr, val),
            },
            _ => panic!("unhandled DMA write {:08x} value: {:08x}", addr, val),
            /*unsafe{
                    println!("interrupt register value {:08x}", self.interrupt.register);
                    let bytes = self.interrupt.bits.into_bytes();
                    println!("interrupt register value: {:02x}{:02x}{:02x}{:02x}", bytes[3],bytes[2],bytes[1],bytes[0]);
                    println!("gpu channel mask: {} Master channel enable: {} gpu flag chanel: {}", self.interrupt.bits.enable_channel2_gpu(), self.interrupt.bits.master_enable(), self.interrupt.bits.flag_channel2_gpu())
                    }
            }*/
        }
    }
}

fn dma_map(address: u32) -> (usize, u32) {
    let reg = address & 0xf; // last nibble
    let channel = (address >> 4) & 0x7;
    (channel as usize, reg)
}
#[bitfield]
#[derive(Copy, Clone)]
pub struct InterruptRegisterBits {
    //I don't know what it does and no one seems to implement it.
    controls_channel: B7,
    #[skip]
    unused: B8,

    //nocash says it also Bus Error Flag
    force_irq: bool,

    //channels interrupt mask
    enable_channel0_mdecin: bool,
    enable_channel1_mdecout: bool,
    enable_channel2_gpu: bool,
    enable_channel3_cdrom: bool,
    enable_channel4_spu: bool,
    enable_channel5_pio: bool,
    enable_channel6_otc: bool,
    master_enable: bool,

    // interrupt flags
    flag_channel0_mdecin: bool,
    flag_channel1_mdecout: bool,
    flag_channel2_gpu: bool,
    flag_channel3_cdrom: bool,
    flag_channel4_spu: bool,
    flag_channel5_pio: bool,
    flag_channel6_otc: bool,
    master_interrupt_flag: bool,
}

union InterruptRegister {
    bits: InterruptRegisterBits,
    register: u32,
}

#[bitfield]
#[derive(Copy, Clone)]
struct ControlRegisterBits {
    mdecin_channel0_priority: B3,
    mdecin_channel0_enable: bool,

    mdecout_channel1_priority: B3,
    mdecout_channel1_enable: bool,

    gpu_channel2_priority: B3,
    gpu_channel2_enable: bool,

    cdrom_channel3_priority: B3,
    cdrom_channel3_enable: bool,

    spu_channel4_priority: B3,
    spu_channel4_enable: bool,

    pio_channel5_priority: B3,
    pio_channel5_enable: bool,

    otc_channel6_priority: B3,
    otc_channel6_enable: bool,

    cpu_priority: B3,
    #[skip]
    cpu_enable: bool,
}

union ControlRegister {
    bits: ControlRegisterBits,
    register: u32,
}
