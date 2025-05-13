use core::panic;

use crate::dma::DMA;
use crate::{
    bios::BIOS, cpu::mipsr3000, expansion_region2::Expansion_Region_2, gpu::GPU,
    irq::IRQController, map, ram::Ram, spu::SPU,
};

pub const CYCLES_PER_FRAME: usize = 564480;

/*pub trait Addressable {
    fn read8(&self, data: &Box<[u8]>, addr: u32) -> u8 {
        let arr: [u8; 1] = [data[addr as usize]; 1];
        u8::from_le_bytes(arr)
    }
    fn read16(&self, data: &Box<[u8]>, addr: u32) -> u16 {
        let arr: [u8; 2] = [data[addr as usize], data[(addr + 1) as usize]];
        u16::from_le_bytes(arr)
    }
    fn read32(&self, data: &Box<[u8]>, addr: u32) -> u32 {
        let mut bytes: [u8; 4] = [0; 4];

        for i in 0..=3 {
            bytes[i] = data[(addr as usize + i) as usize];
        }
        u32::from_le_bytes(bytes)
    }
}*/
pub struct PlayStation {
    pub cpu: mipsr3000::Cpu,
    pub ram: Ram,
    pub bios: BIOS,
    pub exp2: Expansion_Region_2,
    pub mem_ctrl: [u32; 9],

    //Ram Size Bios sets it to 0x00000b88
    pub mem_ctrl_2: u32,

    pub cache_ctrl: u32,
    //pub exp1: Expansion_Region,
    pub dma: DMA,
    pub gpu: GPU,
    pub spu: SPU,
    pub irq: IRQController,
    //cdrom: CDROM
    //mdec: MDEC,
    //gpu: Gpu,
    //irq
    //dma
}

impl PlayStation {
    pub fn new(bios: Box<[u8]>) -> PlayStation {
        PlayStation {
            cpu: mipsr3000::Cpu::new(),
            ram: Ram::new(),
            bios: BIOS::new(bios),
            exp2: Expansion_Region_2::new(),
            mem_ctrl: [0; 9],
            mem_ctrl_2: 0,
            cache_ctrl: 0,
            gpu: GPU::new(),
            dma: DMA::new(),
            //exp1: Expansion_Region::new(),
            spu: SPU::new(),
            irq: IRQController::new(),
        }
    }
    pub fn run_next_frame(&mut self) {
        let mut total_cycles: usize = 0;

        while total_cycles < CYCLES_PER_FRAME {
            let cycles: usize = mipsr3000::run_cycle(self);
            // assume every instruction takes one cycle, I don't know if its accurate, will fix later.
            total_cycles += cycles;
        }
    }

    pub fn run(&mut self) {
        loop {
            mipsr3000::run_instruction(self);
        }
    }

    pub fn read8(&self, address: u32) -> u8 {
        use map::*;
        let phys_address = mask_region(address);
        match phys_address {
            RAM_START..=RAM_END => self.ram.read8(phys_address),
            BIOS_START..=BIOS_END => self.bios.read8(phys_address),
            /*EXPANSION_REGION_1_START..=EXPANSION_REGION_1_END => {
                panic!("Unimplemeted Exp1 Access");
                0xff
            }*/
            0x1f000084 => 0x00,
            _ => panic!(
                "unimplemented Bus addressing at pc read8: {:08x} {:08x}",
                self.cpu.pc, address
            ),
        }
    }

    pub fn read16(&self, address: u32) -> u16 {
        use map::*;
        let phys_address = mask_region(address);
        match phys_address {
            RAM_START..=RAM_END => self.ram.read16(phys_address),
            SPU_START..=SPU_END => self.spu.read_halfword(phys_address),
            IRQ_STATUS_REG => {
                println!("IRQ Status read16 at pc: {:08x} address {:08x}", self.cpu.pc, address);
                return 0;
            }
            IRQ_MASK_REG => {
                println!("IRQ Status read16 at pc: {:08x} address {:08x}", self.cpu.pc, address);
                return 0;
            }
            // Other cases...
            _ => panic!(
                "unimplemented Bus addressing at pc read16: {:08x} {:08x}",
                self.cpu.pc, address
            ),
        }
    }
    pub fn read32(&self, address: u32) -> u32 {
        use map::*;
        let phys_address = mask_region(address);
        let result = match phys_address {
            RAM_START..=RAM_END => self.ram.read32(phys_address),
            BIOS_START..=BIOS_END => self.bios.read32(phys_address),
            IRQ_STATUS_REG => self.irq.get_status(),
            IRQ_MASK_REG => self.irq.get_mask(),
            DMA_REGISTERS_START..=DMA_REGISTERS_END => {
                //println!("DMA read32 at pc {:08x}, address {:08x}", self.cpu.pc, phys_address);
                unsafe { self.dma.read32(phys_address) }
            } //dma.read32(phys_address),
            GPU_REGISTERS_START..=GPU_REGISTERS_END => {
                //return 0;
                self.gpu.read32(phys_address)
            }

            // Other cases...
            _ => panic!(
                "unimplemented Bus addressing at pc read32: {:08x} {:08x}",
                self.cpu.pc, address
            ),
        };

        result
    }
    pub fn write8(&mut self, address: u32, byte: u8) {
        use map::*;
        let phys_address = mask_region(address);
        match phys_address {
            0x00000500 => panic!("at pc {}", self.cpu.pc),
            RAM_START..=RAM_END => self.ram.write8(phys_address, byte),

            SPU_START..=SPU_END => self.spu.write_byte(address, byte),

            /*EXPANSION_REGION_2_START..=EXPANSION_REGION_2_END => {
                println!("EXPANSION_REGION_2 Write");
                self.exp2.write_byte(phys_address, byte);
            }*/
            _ => panic!(
                "unimplemented Bus addressing at pc write8: {:08x} {:08x}",
                self.cpu.pc, address
            ),
        }
    }

    pub fn write16(&mut self, halfword: u16, address: u32) {
        use map::*;
        let phys_address = mask_region(address);
        match phys_address {
            RAM_START..=RAM_END => self.ram.write16(phys_address, halfword),

            SPU_START..=SPU_END => self.spu.write_halfword(phys_address, halfword),

            TIMERS_START..=TIMERS_END => println!(
                "Timers Write16, at pc: {:08x} address {:08x} value {:08x}",
                self.cpu.pc, address, halfword
            ),

            IRQ_STATUS_REG => println!(
                "IRQ Status Write16, at pc: {:08x} address {:08x} value {:08x}",
                self.cpu.pc, address, halfword
            ),

            IRQ_MASK_REG => println!(
                "IRQ Mask Write16, at pc: {:08x} address {:08x} value {:08x}",
                self.cpu.pc, address, halfword
            ),
            _ => panic!(
                "unimplemented Bus addressing at pc write16: {:08x} {:08x}",
                self.cpu.pc, address
            ),
        }
    }

    pub fn write32(&mut self, word: u32, address: u32) {
        use map::*;
        let phys_address = mask_region(address);

        match phys_address {
            /*0x00000500 => {
                if self.cpu.pc != 0xBFC00254 {
                    panic!("at pc {:08X}", self.cpu.pc)
                }
            }*/
            RAM_START..=RAM_END => {
                self.ram.write32(phys_address, word);
                //println!("Write to RAM at 0x{:08X}: 0x{:08X} {:08X}", address, word, self.cpu.pc);
            }
            MEM_CTRL_START..=MEM_CTRL_END => {
                self.mem_ctrl[((phys_address & 0x000000ff) >> 2) as usize] = word
            }
            MEM_CTRL_2_START => self.mem_ctrl_2 = word,
            CACHE_CONTROL_START => self.cache_ctrl = word,
            IRQ_MASK_REG => self.irq.set_mask(word),
            IRQ_STATUS_REG => self.irq.acknowledge(word),

            DMA_REGISTERS_START..=DMA_REGISTERS_END => {
                println!("DMA write32 at pc {:08x}, address {:08x} value: {:08x}", self.cpu.pc, phys_address, word);
                self.dma.write32(phys_address, word);
            },
            GPU_REGISTERS_START..=GPU_REGISTERS_END => println!(
                "GPU write32 at pc {:08x}, address {:08x} value: {:08x}",
                self.cpu.pc, phys_address, word
            ),
            TIMERS_START..=TIMERS_END => println!(
                "Timers write32 at pc {:08x}, address {:08x} value: {:08x}",
                self.cpu.pc, phys_address, word
            ),

            // Other cases...
            _ => {
                panic!(
                    "unimplemented Bus addressing at pc write32: {:08x} {:08x}",
                    self.cpu.pc, address
                )
            }
        }
    }
}

const REGION_MASK: [u32; 8] = [
    // KUSEG: 2048MB
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff, // KSEG0 :
    0x7fffffff, // KSEG1 :
    0x1fffffff, // KSEG2
    0xffffffff, 0xffffffff,
];
pub fn mask_region(addr: u32) -> u32 {
    // Index address space in 512MB chunks
    let index = (addr >> 29) as usize;
    addr & REGION_MASK[index]
}
