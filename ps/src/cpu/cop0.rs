use super::{mipsr3000::Cpu, utils::cop0_register_name};
use crate::playstation::PlayStation;
use log::warn;

const EXECODE_MASK: u32 = 0b00000000000000000000000001111100;
const EXCEPTION_MASK: u32 = 0b11111111111111111111111110000011;

#[derive(Copy, Clone)]
pub struct COP0 {
    //System status register 12
    pub status: u32,

    //Cause of last exception 13
    pub cause: u32,

    //Return address from trap 14
    epc: u32,
}
// TODO figure out default regs initialization.

impl COP0 {
    pub fn new() -> COP0 {
        COP0 { status: 0, cause: 0, epc: 0 }
    }

    pub fn get_status(&self) -> u32 {
        return self.status;
    }

    pub fn is_cache_isolated(&self) -> bool {
        self.status & 0x10000 != 0
    }
    //TODO mtc0, irq, mfc0
}

pub fn handle_exception(ps: &mut PlayStation, exception: Exception) -> u32 {
    let pc = ps.cpu.current_pc;
    // the 6 LSB in status register are for determining where the exception occured,
    //i.e. Kernel or User mode
    let mode = ps.cpu.cop0.status & 0x3F;

    // The status register 6 LSB act as at stack for detecting nested interrupts
    // this 2 commands "pops" the stack
    ps.cpu.cop0.status &= !0x3f;
    ps.cpu.cop0.status |= (mode << 2) & 0x3f;

    ps.cpu.cop0.cause &= EXCEPTION_MASK;
    ps.cpu.cop0.cause |= (exception as u32) << 2;

    if ps.cpu.in_delay_slot() {
        // if an exception occures in a delay slot we save in EPC the address of the branch
        // we also set the BD bit
        ps.cpu.cop0.epc = pc.wrapping_sub(4);
        ps.cpu.cop0.cause |= 0x80000000;
    } else {
        ps.cpu.cop0.epc = pc;
        ps.cpu.cop0.cause &= !0x80000000;
    }

    //BEV (22nd bit) determines if to go to bios or ram.
    if (ps.cpu.cop0.status >> 21) & 1 == 1 { 0xBFC00180 } else { 0x80000080 }
}

pub fn rfe(ps: &mut PlayStation) {
    //panic!("at pc {:08x}", ps.cpu.current_pc);
    let mode = ps.cpu.cop0.status & 0x3F;
    ps.cpu.cop0.status &= !0xf;
    ps.cpu.cop0.status |= mode >> 2;
}

pub fn mtc0(ps: &mut PlayStation, rt: u8, rd: u8) {
    match rd {
        12 => ps.cpu.cop0.status = ps.cpu.registers[rt as usize], //irq???
        13 => ps.cpu.cop0.cause = ps.cpu.registers[rt as usize],
        _ => println!("Unhandeled write to cop0 at: {:08x} {}", ps.cpu.pc, cop0_register_name(rd)),
    }
}

pub fn mfc0(ps: &mut PlayStation, rt: u8, rd: u8) {
    match rd {
        12 => ps.cpu.load_delay_slot = Some((rt as usize, ps.cpu.cop0.status)),

        13 => ps.cpu.load_delay_slot = Some((rt as usize, ps.cpu.cop0.cause)),
        14 => ps.cpu.load_delay_slot = Some((rt as usize, ps.cpu.cop0.epc)),
        _ => println!("Unhandeled read to cop0 at: {:08x} {}", ps.cpu.pc, cop0_register_name(rd)),
    }
}
/*enum COP0_reg_names {
    INDX = 0,
    RAND = 1,
    TLBL = 2,
    BPC = 3,
    CTXT = 4,
    BDA = 5,
    PIDMASK = 6,
    DCIC = 7,
    BADV = 8,
    BDAM = 9,
    TLBH = 10,
    BPCM = 11,
    SR = 12,
    CAUSE = 13,
    EPC = 14,
    PRID = 15,
}*/

pub enum Exception {
    //External Interrupt
    Interrupt = 0x0,
    //TLB Modification Exception
    MOD = 0x1,
    //TLB miss Exception (Load or instruction fetch)
    TLBL = 0x2,
    //TLB miss exception (Store)
    TLBS = 0x3,
    //Address Error Exception (Load or instruction fetch)
    AddressErrorLoad = 0x4,
    //Address Error Exception (Store)
    AddressErrorStore = 0x5,
    //Bus Error Exception (for Instruction Fetch)
    BusErrorFetch = 0x6,
    //Bus Error Exception (for data Load or Store)
    BusErrorLoad = 0x7,
    //SYSCALL Exception
    SYSCALL = 0x8,
    //Breakpoint Exception
    Breakpoint = 0x9,
    //Reserved Instruction Exception
    ReservedInstruction = 0xA,
    //Co-Processor Unusable Exception
    CoProcessorUnusable = 0xB,
    //Arithmetic Overflow Exception
    Overflow = 0xC,
}

//When an Exception occurres, the cpu goes into kernel mode to handle the exception.
//The COP0 is for detection the exception reason
