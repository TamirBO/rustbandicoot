pub const BIOS_START: u32 = 0x1FC00000;
pub const BIOS_SIZE: u32 = 512 * 1024;
pub const BIOS_END: u32 = BIOS_START + BIOS_SIZE - 1;

pub const SPU_START: u32 = 0x1F801C00;
pub const SPU_SIZE: u32 = 0xFFF;
pub const SPU_END: u32 = SPU_START + SPU_SIZE - 1;

pub const EXPANSION_REGION_2_START: u32 = 0x1F802000;
pub const EXPANSION_REGION_2_SIZE: u32 = 8 * 1024;
pub const EXPANSION_REGION_2_END: u32 = EXPANSION_REGION_2_START + EXPANSION_REGION_2_SIZE - 1;

pub const IRQ_STATUS_REG: u32 = 0x1F801070;
pub const IRQ_MASK_REG: u32 = 0x1F801074;

/*
pub const EXPANSION_REGION_1_SIZE: u32 = 8192 * 1024;
pub const SCRATCHPAD_SIZE: u32 = 1 * 1024;
pub const I_O_PORTS_SIZE: u32 = 8 * 1024;
pub const EXPANSION_REGION_2_SIZE: u32 = 8 * 1024;
pub const EXPANSION_REGION_3_SIZE: u32 = 2048 * 1024;
pub const I_O_PORTS_CACHE_SIZE: u32 = (0.5 * 1024.0) as u32;
*/
pub const KUSEG0_START: u32 = 0x00000000;
pub const KUSEG0_SIZE: u32 = 2048 * 1024 * 1024;
pub const KUSEG0_END: u32 = KUSEG0_START + KUSEG0_SIZE - 1;

pub const KSEG0_START: u32 = 0x80000000;
pub const KSEG0_SIZE: u32 = 512 * 1024 * 1024;
pub const KSEG0_END: u32 = KSEG0_START + KSEG0_SIZE - 1;

pub const KSEG1_START: u32 = 0xA0000000;
pub const KSEG1_SIZE: u32 = 512 * 1024 * 1024;
pub const KSEG1_END: u32 = KSEG1_START + KSEG1_SIZE - 1;

pub const KSEG2_START: u32 = 0xC0000000;
pub const KSEG2_SIZE: u32 = 1024 * 1024 * 1024;
pub const KSEG2_END: u32 = KSEG2_START + (KSEG2_SIZE - 1);

pub const RAM_START: u32 = 0x00000000;
pub const RAM_SIZE: u32 = 1024 * 1024 * 2; // 2 MB, we have an option for  Will do it later.
pub const RAM_END: u32 = RAM_START + RAM_SIZE - 1;

pub const EXPANSION_REGION_1_START: u32 = 0x1F000000;
pub const EXPANSION_REGION_1_SIZE: u32 = 8192 * 1024;
pub const EXPANSION_REGION_1_END: u32 = EXPANSION_REGION_1_START + EXPANSION_REGION_1_SIZE - 1;

pub const SCRATCHPAD_START: u32 = 0x1F800000;
pub const SCRATCHPAD_END: u32 = 0x1F8003FF;
pub const SCRATCHPAD_SIZE: u32 = 1024;

pub const MEM_CTRL_START: u32 = 0x1F801000;
pub const MEM_CTRL_SIZE: u32 = 0x24;
pub const MEM_CTRL_END: u32 = MEM_CTRL_START + MEM_CTRL_SIZE - 1;

pub const MEM_CTRL_2_START: u32 = 0x1F801060;

pub const CACHE_CONTROL_START: u32 = 0xFFFE0130;

pub const TIMERS_START: u32 = 0x1F801100;
pub const TIMERS_SIZE: u32 = 0x30;
pub const TIMERS_END: u32 = TIMERS_START + TIMERS_SIZE - 1;

pub const DMA_REGISTERS_START: u32 = 0x1F801080;
pub const DMA_REGISTERS_SIZE: u32 = 0x80;
pub const DMA_REGISTERS_END: u32 = DMA_REGISTERS_START + DMA_REGISTERS_SIZE - 1;

pub const GPU_REGISTERS_START: u32 = 0x1F801810;
pub const GPU_REGISTERS_SIZE: u32 = 0x8;

pub const GPU_REGISTERS_END: u32 = GPU_REGISTERS_START + GPU_REGISTERS_SIZE - 1;

/*
pub const JOYSTICK_MEM_CARD: Range = Range(0x1F801040, 32);
pub const MEM_CTRL_2: Range = Range(0x1F801060, 4);
pub const INTERRUPT_CTRL: Range = Range(0x1F801070, 4);
pub const DMA_REGISTERS: Range = Range(0x1F801080, 0x80);
pub const TIMERS: Range = Range(0x1F801100, 0x30);
pub const CDROM_REGS: Range = Range(0x1F801800, 0x4);
pub const GPU_REGS: Range = Range(0x1F801810, 0x8);
pub const MDEC_REGS: Range = Range(0x1F801820, 0x8);
pub const SPU: Range = Range(0x1F801D80, 0x280);
pub const EXPANSION_REGION_2: Range = Range(0x1F802000, 0x67);
pub const BIOS: Range = Range(0x1FC00000, BIOS_SIZE);
*/
