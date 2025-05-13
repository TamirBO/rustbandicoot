use super::cop0::{self, Exception};
use super::instruction::*;
use super::utils::register_name;
use crate::map::{RAM_END, RAM_START};
use crate::playstation::PlayStation;


use crate::cpu::instruction::Operation::RType;
use std::collections::btree_map::Values;
use std::fmt;
use std::fmt::Write;

#[derive(Copy, Clone)]

//TODO change all the pub to private by adding getters and setters.
//TODO set_register function that sets registers instead of directly setting so we won't change $zero
pub struct Cpu {
    // General Purpose Registers
    pub registers: [u32; 32],
    // current pc -> points to the instruction about to be executed.
    pub current_pc: u32,
    // Program Counter - points to the next instruction about to be executed
    pub pc: u32,
    // Next for PC register - Used for delay slot emulation either Load/Branch
    pub next_pc: u32,
    // HI and LO registers used for Multiplication and Division
    pub hi: u32,
    pub lo: u32,
    // delay slots holds if an instruction occured at a delay slot, it's use to handle exceptions correctly
    // because exception handler behave differently if an exception occured at a delay slot instruction.
    delay_slot: bool,
    // branch taken holds if a branch was is taken.
    branch_taken: bool,
    // Save reg index and value when a load occures so we can execute instruction with old values.
    // Simias does it with another value for the cycles the load takes.
    pub load_delay_slot: Option<(usize, u32)>,
    pub cop0: cop0::COP0,
    //this is just for debugging
    gte: [u32; 64],
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut regs_str = String::new();
        for (i, val) in self.registers.iter().enumerate() {
            write!(&mut regs_str, "{}: {:08x} \n", register_name(i as u8), val)?;
        }
        write!(f, "pc:{:08x}\n{} ", self.pc, regs_str)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 32],
            current_pc: 0xBFC00000,
            pc: 0xBFC00000,
            next_pc: 0xBFC00004,
            hi: 0,
            lo: 0,
            delay_slot: false,
            branch_taken: false,
            load_delay_slot: None,
            cop0: cop0::COP0::new(),
            gte: [0; 64],
        }
    }
    //TODO function that executes the delay slot instruction and branches.

    pub fn get_reg(&self, index: usize) -> u32 {
        self.registers[index]
    }

    //for debugging proccesses
    pub fn set_reg(&mut self, data: u32, index: usize) {
        if index == 0 {
            return;
        }
        self.registers[index] = data;
    }
    pub fn in_delay_slot(&self) -> bool {
        self.delay_slot
    }
}

pub fn execute(ps: &mut PlayStation, ins: Instruction) {
    match ins.operation() {
        Operation::IType(i_op) => execute_itype(ps, i_op),
        Operation::JType(j_op) => execute_jtype(ps, j_op),
        Operation::RType(r_op) => execute_rtype(ps, r_op),
        Operation::COP0(cop0_op) => execute_cop0(ps, cop0_op),
        Operation::NOOP => noop(),
        _ => {
            let Instruction(inst) = ins;
            panic!("unimplemented yet at pc {:08x} {:08x}", ps.cpu.pc, inst);
        }
    }
}

pub fn execute_itype(ps: &mut PlayStation, i_op: ITypeOperation) {
    use ITypeOperation::*;
    match i_op {
        BLTZ { rs, immediate_se } => bltz(ps, rs, immediate_se),
        BGEZ { rs, immediate_se } => bgez(ps, rs, immediate_se),
        //BLTZAL { rs, immediate_se } => bltzal(ps, rs, immediate_se),
        //BGEZAL { rs, immediate_se } => bgezal(ps, rs, immediate_se),
        BEQ { rs, rt, immediate_se } => beq(ps, rs, rt, immediate_se),
        BNE { rs, rt, immediate_se } => bne(ps, rs, rt, immediate_se),
        BLEZ { rs, immediate_se } => blez(ps, rs, immediate_se),
        BGTZ { rs, immediate_se } => bgtz(ps, rs, immediate_se),
        ADDI { rt, rs, immediate_se } => addi(ps, rt, rs, immediate_se),
        ADDIU { rt, rs, immediate_se } => addiu(ps, rt, rs, immediate_se),
        SLTI { rt, rs, immediate_se } => slti(ps, rt, rs, immediate_se),
        SLTIU { rt, rs, immediate_se } => sltiu(ps, rt, rs, immediate_se),
        ANDI { rt, rs, immediate } => andi(ps, rt, rs, immediate),
        ORI { rt, rs, immediate } => ori(ps, rt, rs, immediate),
        //XORI { rt, rs, immediate } => xori(ps, rt, rs, immediate),
        LUI { rt, immediate } => lui(ps, rt, immediate),
        LB { rt, rs, immediate_se } => lb(ps, rt, rs, immediate_se),
        LH { rt, rs, immediate_se } => lh(ps, rt, rs, immediate_se),
        //LWL { rt, rs, immediate_se } => lwl(ps, rt, rs, immediate_se),
        LW { rt, rs, immediate_se } => lw(ps, rt, rs, immediate_se),
        LBU { rt, rs, immediate_se } => lbu(ps, rt, rs, immediate_se),
        LHU { rt, rs, immediate_se } => lhu(ps, rt, rs, immediate_se),
        //LWR { rt, rs, immediate_se } => lwr(ps, rt, rs, immediate_se),
        SB { rt, rs, immediate_se } => sb(ps, rt, rs, immediate_se),
        SH { rt, rs, immediate_se } => sh(ps, rt, rs, immediate_se),
        //SWL { rt, rs, immediate_se } => swl(ps, rt, rs, immediate_se),
        SW { rt, rs, immediate_se } => sw(ps, rt, rs, immediate_se),
        //SWR { rt, rs, immediate_se } => swr(ps, rt, rs, immediate_se),
        //ILLEGAL => (), COP0?
        _ => panic!("Unimplemented IType Instruction at: {:08x} {}", ps.cpu.pc, i_op.to_string()),
    }
}

pub fn execute_jtype(ps: &mut PlayStation, j_op: JTypeOperation) {
    use JTypeOperation::*;

    match j_op {
        J { target } => j(ps, target),
        JAL { target } => jal(ps, target),
        //ILLEGAL => (), COP0 Reserverd Instruction
        _ => panic!("Unimplemented JType Instruction"),
    }
}

pub fn execute_rtype(ps: &mut PlayStation, r_op: RTypeOperation) {
    use RTypeOperation::*;

    match r_op {
        SLL { rd, rt, shamt } => sll(ps, rd, rt, shamt),
        SRL { rd, rt, shamt } => srl(ps, rd, rt, shamt),
        SRA { rd, rt, shamt } => sra(ps, rd, rt, shamt),

        SLLV { rd, rt, rs } => sllv(ps, rd, rt, rs),
        SRLV { rd, rt, rs } => srlv(ps, rd, rt, rs),
        SRAV { rd, rt, rs } => srav(ps, rd, rt, rs),
        JR { rs } => jr(ps, rs),
        JALR { rd, rs } => jalr(ps, rd, rs),

        SYSCALL => syscall(ps),
        BREAK => op_break(ps),

        MFHI { rd } => mfhi(ps, rd),
        MTHI { rs } => mthi(ps, rs),
        MFLO { rd } => mflo(ps, rd),
        MTLO { rs } => mtlo(ps, rs),

        //MULT { rs, rt } => mult(ps, rs, rt),
        MULTU { rs, rt } => multu(ps, rs, rt),
        DIV { rs, rt } => div(ps, rs, rt),
        DIVU { rs, rt } => divu(ps, rs, rt),
        ADD { rd, rs, rt } => add(ps, rd, rs, rt),
        ADDU { rd, rs, rt } => addu(ps, rd, rs, rt),
        //SUB { rd, rs, rt } => sub(ps, rd, rs, rt),
        SUBU { rd, rs, rt } => subu(ps, rd, rs, rt),
        AND { rd, rs, rt } => and(ps, rd, rs, rt),
        OR { rd, rs, rt } => or(ps, rd, rs, rt),
        XOR { rd, rs, rt } => xor(ps, rd, rs, rt),
        NOR { rd, rs, rt } => nor(ps, rd, rs, rt),
        SLT { rd, rs, rt } => slt(ps, rd, rs, rt),
        SLTU { rd, rs, rt } => sltu(ps, rd, rs, rt),

        //ILLEGAL => cop0 RI
        _ => panic!(
            "Unimplemented yet RType Instruction: {} pc: {:08x}",
            r_op.to_string(),
            ps.cpu.current_pc
        ),
    }
}

pub fn execute_cop0(ps: &mut PlayStation, cop0_op: Cop0Operation) {
    use Cop0Operation::*;

    match cop0_op {
        MFC0 { rt, rd } => cop0::mfc0(ps, rt, rd),
        MTC0 { rt, rd } => cop0::mtc0(ps, rt, rd),
        RFE => cop0::rfe(ps),
        //COP0 { copfun } cop0_func(copfun),
        //ILLEGAL => cop0 RI
        _ => {
            let Instruction(ins) = fetch(ps, ps.cpu.pc);
            panic!(
                "Unimplemented yet COP0 instruction at: {:08x} {:08x} {}",
                ps.cpu.pc,
                ins,
                cop0_op.to_string()
            )
        }
    }
}

pub fn run_cycle(ps: &mut PlayStation) -> usize {
    // assume every instruction takes one cycle, I don't know if its accurate, will fix later.
    run_instruction(ps);
    return 1;
}

//TODO Change to @param Instruction
pub fn run_instruction(ps: &mut PlayStation) {
    ps.cpu.current_pc = ps.cpu.pc;

    if ps.cpu.current_pc % 4 != 0 {
        exception(ps, Exception::AddressErrorLoad);
        return;
    }

    ps.cpu.pc = ps.cpu.next_pc;
    ps.cpu.next_pc = ps.cpu.pc.wrapping_add(4);
    ps.cpu.delay_slot = ps.cpu.branch_taken;
    ps.cpu.branch_taken = false;

    let ins = fetch(ps, ps.cpu.current_pc);

    execute(ps, ins);

    // Check after executing the instruction

    // Execute any pending loads
    execute_load_delay(ps);
}
pub fn fetch(ps: &PlayStation, pc: u32) -> Instruction {
    //use crate::playstation::mask_region;
    //TODO cache handling and Exceptions at unalinged addresess
    //println!("Fetching from address: 0x{:08X}, masked to 0x{:08X}", pc, mask_region(pc));
    let x = ps.read32(pc);

    Instruction(x)
}

fn execute_load_delay(ps: &mut PlayStation) {
    if let Some((index, value)) = ps.cpu.load_delay_slot {
        ps.cpu.set_reg(value, index);
    }
    ps.cpu.load_delay_slot = None;
}
//TODO exceptions at load and store instructions.

//exception handling needs working
fn exception(ps: &mut PlayStation, e: Exception) {
    let exception_handler = cop0::handle_exception(ps, e);
    ps.cpu.pc = exception_handler;
    ps.cpu.next_pc = exception_handler.wrapping_add(4);
}

fn branch_taken(ps: &mut PlayStation, branch: u32) {
    ps.cpu.branch_taken = true;
    ps.cpu.next_pc = ps.cpu.pc.wrapping_add(branch);
}

fn add(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let rs = ps.cpu.registers[rs as usize] as i32;
    let rt = ps.cpu.registers[rt as usize] as i32;

    match rs.checked_add(rt) {
        Some(result) => ps.cpu.set_reg(result as u32, rd as usize),
        None => exception(ps, Exception::Overflow),
    }
}

fn addi(ps: &mut PlayStation, rt: u8, rs: u8, imm_se: u32) {
    let rs = ps.cpu.registers[rs as usize] as i32;
    let imm_se = imm_se as i32;
    match rs.checked_add(imm_se) {
        Some(result) => ps.cpu.set_reg(result as u32, rt as usize),
        None => exception(ps, Exception::Overflow),
    }
}

fn addiu(ps: &mut PlayStation, rt: u8, rs: u8, imm: u32) {
    let result = ps.cpu.registers[rs as usize].wrapping_add(imm);
    ps.cpu.set_reg(result, rt as usize);
}
fn addu(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize].wrapping_add(ps.cpu.registers[rt as usize]);
    ps.cpu.set_reg(result, rd as usize);
}
fn and(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize] & ps.cpu.registers[rt as usize];
    ps.cpu.set_reg(result, rd as usize);
}
fn andi(ps: &mut PlayStation, rt: u8, rs: u8, imm: u32) {
    let result = ps.cpu.registers[rs as usize] & imm;
    ps.cpu.set_reg(result, rt as usize);
}

fn beq(ps: &mut PlayStation, rs: u8, rt: u8, offset: u32) {
    if ps.cpu.registers[rs as usize] == ps.cpu.registers[rt as usize] {
        branch_taken(ps, offset << 2);
    }
}

fn bgez(ps: &mut PlayStation, rs: u8, offset: u32) {
    if (ps.cpu.registers[rs as usize] as i32) >= 0 {
        branch_taken(ps, offset << 2);
    }
}

fn bgtz(ps: &mut PlayStation, rs: u8, offset: u32) {
    if (ps.cpu.registers[rs as usize] as i32) > 0 {
        branch_taken(ps, offset << 2);
    }
}

fn blez(ps: &mut PlayStation, rs: u8, offset: u32) {
    if (ps.cpu.registers[rs as usize] as i32) <= 0 {
        branch_taken(ps, offset << 2);
    }
}

fn bltz(ps: &mut PlayStation, rs: u8, offset: u32) {
    if (ps.cpu.registers[rs as usize] as i32) < 0 {
        branch_taken(ps, offset << 2);
    }
}

fn bne(ps: &mut PlayStation, rs: u8, rt: u8, offset: u32) {
    if ps.cpu.registers[rs as usize] != ps.cpu.registers[rt as usize] {
        branch_taken(ps, offset << 2);
    }
}

fn op_break(ps: &mut PlayStation) {
    exception(ps, Exception::Breakpoint);
}

fn div(ps: &mut PlayStation, rs: u8, rt: u8) {
    let rs = ps.cpu.registers[rs as usize] as i32;
    let rt = ps.cpu.registers[rt as usize] as i32;

    if rt == 0 {
        // division by zero, result are:
        //   div     rs = 0..+7FFFFFFFh   rt = 0   -->  hi = Rs           lo = -1
        //   div     rs = -80000000h..-1  rt = 0   -->  hi = Rs           lo = +1
        ps.cpu.hi = rs as u32;

        if rs >= 0 {
            ps.cpu.lo = 0xffff_ffff; // -1
        } else {
            ps.cpu.lo = 1;
        }
    } else if rs as u32 == 0x8000_0000 && rt == -1 {
        ps.cpu.hi = 0;
        ps.cpu.lo = 0x8000_0000;
    } else {
        ps.cpu.hi = (rs % rt) as u32;
        ps.cpu.lo = (rs / rt) as u32;
    }
    // TODO timings
}

fn divu(ps: &mut PlayStation, rs: u8, rt: u8) {
    let rs = ps.cpu.registers[rs as usize];
    let rt = ps.cpu.registers[rt as usize];
    if rt == 0 {
        // divide by zero, results are:
        //  divu    rs = 0..FFFFFFFFh    rt = 0   -->  hi = Rs            lo = FFFFFFFFh
        ps.cpu.hi = rs;
        ps.cpu.lo = 0xffff_ffff;
    } else {
        ps.cpu.hi = (rs % rt);
        ps.cpu.lo = (rs / rt);
    }
    // TODO timings
}

fn j(ps: &mut PlayStation, target: u32) {
    ps.cpu.branch_taken = true;
    ps.cpu.next_pc = ps.cpu.pc & 0xf0000000 | target;
}

fn jal(ps: &mut PlayStation, target: u32) {
    ps.cpu.branch_taken = true;
    //after delay slot
    ps.cpu.registers[31] = ps.cpu.next_pc;
    ps.cpu.next_pc = (ps.cpu.pc & 0xf0000000) | target;
}

fn jalr(ps: &mut PlayStation, rd: u8, rs: u8) {
    ps.cpu.branch_taken = true;

    let return_address = ps.cpu.next_pc;
    ps.cpu.next_pc = ps.cpu.registers[rs as usize];
    ps.cpu.set_reg(return_address, rd as usize);
}

fn jr(ps: &mut PlayStation, rs: u8) {
    ps.cpu.branch_taken = true;
    ps.cpu.next_pc = ps.cpu.registers[rs as usize];
}

fn lb(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);
    // force signed extention
    let value = ps.read8(address);
    let value = value as i8;
    ps.cpu.load_delay_slot = Some((rt as usize, value as u32));
}

fn lbu(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);
    let value = ps.read8(address);
    ps.cpu.load_delay_slot = Some((rt as usize, value as u32));
}

fn lh(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);

    if address % 2 == 0 {
        let value = ps.read16(address) as i16;
        ps.cpu.load_delay_slot = Some((rt as usize, value as u32));
    } else {
        exception(ps, Exception::AddressErrorLoad);
    }
}

fn lhu(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);

    if address % 2 == 0 {
        let value = ps.read16(address);
        ps.cpu.load_delay_slot = Some((rt as usize, value as u32));
    } else {
        exception(ps, Exception::AddressErrorLoad);
    }
}

fn lui(ps: &mut PlayStation, rt: u8, imm: u32) {
    let upper = imm << 16;
    ps.cpu.set_reg(upper, rt as usize);
}

fn lw(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);

    // if address is 32bit aligned.
    if address % 4 == 0 {
        let value = ps.read32(address);
        ps.cpu.load_delay_slot = Some((rt as usize, value));
    } else {
        exception(ps, Exception::AddressErrorLoad)
    }
}

fn mfhi(ps: &mut PlayStation, rd: u8) {
    //panic!("at pc {:08x}", ps.cpu.current_pc);
    ps.cpu.set_reg(ps.cpu.hi, rd as usize);
    // TODO timings
}

fn mflo(ps: &mut PlayStation, rd: u8) {
    ps.cpu.set_reg(ps.cpu.lo, rd as usize);
    // TODO timings
}

fn mthi(ps: &mut PlayStation, rs: u8) {
    ps.cpu.hi = ps.cpu.registers[rs as usize];
}

fn mtlo(ps: &mut PlayStation, rs: u8) {
    ps.cpu.lo = ps.cpu.registers[rs as usize];
}

fn multu(ps: &mut PlayStation, rs: u8, rt: u8) {
    let a = ps.cpu.registers[rs as usize] as u64;
    let b = ps.cpu.registers[rt as usize] as u64;

    let value = a * b;

    ps.cpu.hi = (value >> 32) as u32;
    ps.cpu.lo = value as u32;
}
fn or(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize] | ps.cpu.registers[rt as usize];
    ps.cpu.set_reg(result, rd as usize);
}

fn noop() {}

fn nor(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = !(ps.cpu.registers[rs as usize] | ps.cpu.registers[rt as usize]);
    ps.cpu.set_reg(result, rd as usize);
}
fn ori(ps: &mut PlayStation, rt: u8, rs: u8, imm: u32) {
    let result = ps.cpu.registers[rs as usize] | imm;
    ps.cpu.set_reg(result, rt as usize);
}

fn sb(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);
    let byte = ps.cpu.registers[rt as usize] as u8;
    ps.write8(address, byte);
}
fn sh(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    let address = offset.wrapping_add(ps.cpu.registers[rs as usize]);
    let half_word = ps.cpu.registers[rt as usize] as u16;
    if address % 2 == 0 {
        ps.write16(half_word, address);
    } else {
        exception(ps, Exception::AddressErrorStore);
    }
}

fn sll(ps: &mut PlayStation, rd: u8, rt: u8, sa: u8) {
    let result = ps.cpu.registers[rt as usize] << sa;
    ps.cpu.set_reg(result, rd as usize);
}

fn sllv(ps: &mut PlayStation, rd: u8, rt: u8, rs: u8) {
    // take only the first 5bits, because we don't shift more than 31 bits.
    let shift_amount = ps.cpu.registers[rs as usize] & 0x1f;
    let result = ps.cpu.registers[rt as usize] << shift_amount;
    ps.cpu.set_reg(result, rd as usize);
}

fn slt(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = (ps.cpu.registers[rs as usize] as i32) < (ps.cpu.registers[rt as usize] as i32);
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn slti(ps: &mut PlayStation, rt: u8, rs: u8, immediate_se: u32) {
    //panic!("at pc {:08x}", ps.cpu.current_pc);
    let result = (ps.cpu.registers[rs as usize] as i32) < immediate_se as i32;
    ps.cpu.set_reg(result as u32, rt as usize);
}

fn sltiu(ps: &mut PlayStation, rt: u8, rs: u8, immediate_se: u32) {
    let result = ps.cpu.registers[rs as usize] < immediate_se;
    ps.cpu.set_reg(result as u32, rt as usize);
}

fn sltu(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize] < ps.cpu.registers[rt as usize];
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn sra(ps: &mut PlayStation, rd: u8, rt: u8, shamt: u8) {
    //panic!("at PC {:08x} ", ps.cpu.pc);
    let result = (ps.cpu.registers[rt as usize] as i32) >> shamt;
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn srav(ps: &mut PlayStation, rd: u8, rt: u8, rs: u8) {
    let result = (ps.cpu.registers[rt as usize] as i32) >> (ps.cpu.registers[rs as usize] & 0x1f);
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn srl(ps: &mut PlayStation, rd: u8, rt: u8, shamt: u8) {
    let result = (ps.cpu.registers[rt as usize]) >> shamt;
    ps.cpu.set_reg(result, rd as usize);
}

fn srlv(ps: &mut PlayStation, rd: u8, rt: u8, rs: u8) {
    let shift_amount = ps.cpu.registers[rs as usize] & 0x1f;
    let result = ps.cpu.registers[rt as usize] >> shift_amount;
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn subu(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize].wrapping_sub(ps.cpu.registers[rt as usize]);
    ps.cpu.set_reg(result as u32, rd as usize);
}

fn sw(ps: &mut PlayStation, rt: u8, rs: u8, offset: u32) {
    if ps.cpu.cop0.is_cache_isolated() {
        return;
        //TODO Cache work.
    }

    let address = ps.cpu.registers[rs as usize].wrapping_add(offset);
    let word = ps.cpu.registers[rt as usize];
    if address % 4 == 0 {
        ps.write32(word, address);
    } else {
        exception(ps, Exception::AddressErrorStore);
    }
}

fn syscall(ps: &mut PlayStation) {
    exception(ps, Exception::SYSCALL);
}

fn xor(ps: &mut PlayStation, rd: u8, rs: u8, rt: u8) {
    let result = ps.cpu.registers[rs as usize] ^ ps.cpu.registers[rt as usize];
    ps.cpu.set_reg(result as u32, rd as usize);
}
