const OP_MASK: u32 = 0b11111100000000000000000000000000;
const SPECIAL_MASK: u32 = 0b00000000000000000000000000111111;
const RS_MASK: u32 = 0b00000011111000000000000000000000;
const RT_MASK: u32 = 0b00000000000111110000000000000000;
const RD_MASK: u32 = 0b00000000000000001111100000000000;
const SA_MASK: u32 = 0b00000000000000000000011111000000;
const IMMEDIATE_MASK: u32 = 0b00000000000000001111111111111111;
const COPN_MASK: u32 = 0b00001100000000000000000000000000;
const REGIMM_MASK: u32 = 0b00000000000111110000000000000000;
const TARGET_MASK: u32 = 0b00111111111111111111111111111111;

use std::fmt;
use std::fmt::Write;

use crate::cpu::utils::{
    cop0_register_name, cop2_control_register_name, cop2_data_register_name, register_name,
};

pub struct Instruction(pub u32);

impl Instruction {
    // Returns Operation (first 6 bits)
    pub fn op(&self) -> u32 {
        let Instruction(ins) = self;
        ins >> 26
    }
    // Returns Special func number (the last 6 bits).
    pub fn funct(&self) -> u32 {
        let Instruction(ins) = self;
        ins & SPECIAL_MASK
    }
    //Returns RS (25-21 bits)
    pub fn rs(&self) -> u8 {
        let Instruction(ins) = self;
        ((ins & RS_MASK) >> 21) as u8
    }
    //Returns RT (20-16 bits)
    pub fn rt(&self) -> u8 {
        let Instruction(ins) = self;
        ((ins & RT_MASK) >> 16) as u8
    }
    //returns RD (15-11 bits)
    pub fn rd(&self) -> u8 {
        let Instruction(ins) = self;
        ((ins & RD_MASK) >> 11) as u8
    }
    //Returns the shift amount SA (10-6)
    pub fn shamt(&self) -> u8 {
        let Instruction(ins) = self;
        ((ins & SA_MASK) >> 6) as u8
    }
    //Returns Immediate (15-0 bits)
    pub fn immediate(&self) -> u32 {
        let Instruction(ins) = self;
        ins & IMMEDIATE_MASK
    }
    //Returns offset (15-0 bits), like immediate but signed
    pub fn offset(&self) -> u16 {
        let Instruction(ins) = self;
        (ins & IMMEDIATE_MASK) as u16
    }
    pub fn offset_singed_ext(&self) -> u32 {
        let Instruction(ins) = self;
        let v = (ins & 0xffff) as i16;
        v as u32
    }

    pub fn immediate_sign_ext(&self) -> u32 {
        let Instruction(op) = self;

        let v = (op & 0xffff) as i16;

        v as u32
    }
    //Returns Co-Proccessor number (26-27 bits)
    pub fn cop_number(&self) -> u32 {
        let Instruction(ins) = self;
        (ins & COPN_MASK) >> 26
    }
    //Returns Branch func number (20-16 bits)
    pub fn reg_imm(&self) -> u32 {
        let Instruction(ins) = self;
        (ins & REGIMM_MASK) >> 16
    }
    pub fn target(&self) -> u32 {
        let Instruction(ins) = self;
        (ins & 0x3ffffff) << 2
    }
    pub fn all(&self) -> u32 {
        self.0
    }
    pub fn cofun(&self) -> u32 {
        let Instruction(ins) = self;
        ins & 0b00000001111111111111111111111111
    }

    #[allow(clippy::unreadable_literal)]
    pub fn operation(&self) -> Operation {
        let Instruction(ins) = self;

        let rd = self.rd();
        let rt = self.rt();
        let rs = self.rs();
        let immediate = self.immediate();
        let immediate_se = self.immediate_sign_ext();
        if self.all() == 0 {
            return Operation::NOOP;
        }
        match self.op() {
            0x0 => {
                // R-type instruction
                let funct = self.funct();
                let shamt = self.shamt();

                let rtype_op = match funct {
                    0x00 => RTypeOperation::SLL { rd, rt, shamt },
                    0x02 => RTypeOperation::SRL { rd, rt, shamt },
                    0x03 => RTypeOperation::SRA { rd, rt, shamt },
                    0x04 => RTypeOperation::SLLV { rd, rt, rs },
                    0x06 => RTypeOperation::SRLV { rd, rt, rs },
                    0x07 => RTypeOperation::SRAV { rd, rt, rs },

                    0x08 => RTypeOperation::JR { rs },
                    0x09 => RTypeOperation::JALR { rd, rs },

                    0x0C => RTypeOperation::SYSCALL,
                    0x0D => RTypeOperation::BREAK,

                    0x10 => RTypeOperation::MFHI { rd },
                    0x11 => RTypeOperation::MTHI { rs },
                    0x12 => RTypeOperation::MFLO { rd },
                    0x13 => RTypeOperation::MTLO { rs },

                    0x18 => RTypeOperation::MULT { rs, rt },
                    0x19 => RTypeOperation::MULTU { rs, rt },
                    0x1A => RTypeOperation::DIV { rs, rt },
                    0x1B => RTypeOperation::DIVU { rs, rt },
                    0x20 => RTypeOperation::ADD { rs, rt, rd },
                    0x21 => RTypeOperation::ADDU { rs, rt, rd },
                    0x22 => RTypeOperation::SUB { rs, rt, rd },
                    0x23 => RTypeOperation::SUBU { rs, rt, rd },
                    0x24 => RTypeOperation::AND { rs, rt, rd },
                    0x25 => RTypeOperation::OR { rs, rt, rd },
                    0x26 => RTypeOperation::XOR { rs, rt, rd },
                    0x27 => RTypeOperation::NOR { rs, rt, rd },
                    0x2A => RTypeOperation::SLT { rs, rt, rd },
                    0x2B => RTypeOperation::SLTU { rs, rt, rd },

                    _ => RTypeOperation::ILLEGAL,
                };

                Operation::RType(rtype_op)
            }
            0x01 => {
                let itype_op = match rt {
                    0x00 => ITypeOperation::BLTZ { rs, immediate_se },
                    0x01 => ITypeOperation::BGEZ { rs, immediate_se },
                    0x10 => ITypeOperation::BLTZAL { rs, immediate_se },
                    0x11 => ITypeOperation::BGEZAL { rs, immediate_se },
                    _ => ITypeOperation::ILLEGAL,
                };

                Operation::IType(itype_op)
            }

            0x02 | 0x03 => {
                // J-type instructions
                let target = self.target();

                let j_op = match self.op() {
                    0x02 => JTypeOperation::J { target },
                    0x03 => JTypeOperation::JAL { target },
                    _ => JTypeOperation::ILLEGAL,
                };

                Operation::JType(j_op)
            }

            0x04 => Operation::IType(ITypeOperation::BEQ { rs, rt, immediate_se }),
            0x05 => Operation::IType(ITypeOperation::BNE { rs, rt, immediate_se }),
            0x06 => Operation::IType(ITypeOperation::BLEZ { rs, immediate_se }),
            0x07 => Operation::IType(ITypeOperation::BGTZ { rs, immediate_se }),

            0x08 => Operation::IType(ITypeOperation::ADDI { rt, rs, immediate_se }),
            0x09 => Operation::IType(ITypeOperation::ADDIU { rt, rs, immediate_se }),
            0x0A => Operation::IType(ITypeOperation::SLTI { rt, rs, immediate_se }),
            0x0B => Operation::IType(ITypeOperation::SLTIU { rt, rs, immediate_se }),
            0x0C => Operation::IType(ITypeOperation::ANDI { rt, rs, immediate }),
            0x0D => Operation::IType(ITypeOperation::ORI { rt, rs, immediate }),
            0x0E => Operation::IType(ITypeOperation::XORI { rt, rs, immediate }),

            0x0F => Operation::IType(ITypeOperation::LUI { rt, immediate }),
            0x20 => Operation::IType(ITypeOperation::LB { rt, rs, immediate_se }),
            0x21 => Operation::IType(ITypeOperation::LH { rt, rs, immediate_se }),
            0x22 => Operation::IType(ITypeOperation::LWL { rt, rs, immediate_se }),
            0x23 => Operation::IType(ITypeOperation::LW { rt, rs, immediate_se }),
            0x24 => Operation::IType(ITypeOperation::LBU { rt, rs, immediate_se }),
            0x25 => Operation::IType(ITypeOperation::LHU { rt, rs, immediate_se }),
            0x26 => Operation::IType(ITypeOperation::LWR { rt, rs, immediate_se }),

            0x28 => Operation::IType(ITypeOperation::SB { rt, rs, immediate_se }),
            0x29 => Operation::IType(ITypeOperation::SH { rt, rs, immediate_se }),
            0x2A => Operation::IType(ITypeOperation::SWL { rt, rs, immediate_se }),
            0x2B => Operation::IType(ITypeOperation::SW { rt, rs, immediate_se }),
            0x2E => Operation::IType(ITypeOperation::SWR { rt, rs, immediate_se }),

            0x10 => {
                let cop0_op = match rs {
                    0x00 => Cop0Operation::MFC0 { rt, rd },
                    //0x02 => CFC0
                    0x04 => Cop0Operation::MTC0 { rt, rd },
                    //0x06 => CTC0
                    0x10 => {
                        if self.funct() == 0x10 {
                            Cop0Operation::RFE
                        } else {
                            Cop0Operation::ILLEGAL
                            //panic!("Unsupported COP0 instruction in the PS1 {:08x}", ins);
                        }
                    }
                    _ => Cop0Operation::ILLEGAL,
                };
                Operation::COP0(cop0_op)
            }

            0x12 => {
                //if the 7th MSB is set, Its a GTE command (Cop2)
                let gte_op = if (ins >> 25) & 1 == 1 {
                    GTEOperation::GTE
                } else {
                    match rs {
                        0x00 => GTEOperation::MFC2 { rt, rd },
                        0x02 => GTEOperation::CFC2 { rt, rd },
                        0x04 => GTEOperation::MTC2 { rt, rd },
                        0x06 => GTEOperation::CTC2 { rt, rd },
                        _ => GTEOperation::ILLEGAL,
                    }
                };
                Operation::GTE(gte_op)
            }
            //0x30 => Operation::COP0(Cop0Operation::LWC0 { rt, rs, immediate }),
            0x32 => Operation::GTE(GTEOperation::LWC2 { rt, rs, immediate_se }),
            //0x38 => Operation::COP0(Cop0Operation::SWC0 { rt, rs, immediate }),
            0x3A => Operation::GTE(GTEOperation::SWC2 { rt, rs, immediate_se }),

            _ => Operation::ILLEGAL,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Operation {
    RType(RTypeOperation),
    IType(ITypeOperation),
    JType(JTypeOperation),
    COP0(Cop0Operation),
    GTE(GTEOperation),
    NOOP,
    ILLEGAL,
}

#[derive(PartialEq, Debug)]
pub enum RTypeOperation {
    // All R-type instructions begin with 6 bits of zeros.
    // The last 6 bits (funct) determine the operation.

    // Shifts with immediate shift amount
    SLL { rd: u8, rt: u8, shamt: u8 }, // 0x00
    SRL { rd: u8, rt: u8, shamt: u8 }, // 0x02
    SRA { rd: u8, rt: u8, shamt: u8 }, // 0x03

    // Shifts with variable shift amount
    SLLV { rd: u8, rt: u8, rs: u8 }, // 0x04
    SRLV { rd: u8, rt: u8, rs: u8 }, // 0x06
    SRAV { rd: u8, rt: u8, rs: u8 }, // 0x07

    // Jumps
    JR { rs: u8 },           // 0x08
    JALR { rd: u8, rs: u8 }, // 0x09

    // System calls
    SYSCALL, // 0x0C
    BREAK,   // 0x0D

    // HI/LO register moves
    MFHI { rd: u8 }, // 0x10
    MTHI { rs: u8 }, // 0x11
    MFLO { rd: u8 }, // 0x12
    MTLO { rs: u8 }, // 0x13

    // Multiply and Divide
    MULT { rs: u8, rt: u8 },  // 0x18
    MULTU { rs: u8, rt: u8 }, // 0x19
    DIV { rs: u8, rt: u8 },   // 0x1A
    DIVU { rs: u8, rt: u8 },  // 0x1B

    // Arithmetic and Logic Instructions
    ADD { rd: u8, rs: u8, rt: u8 },  // 0x20
    ADDU { rd: u8, rs: u8, rt: u8 }, // 0x21
    SUB { rd: u8, rs: u8, rt: u8 },  // 0x22
    SUBU { rd: u8, rs: u8, rt: u8 }, // 0x23
    AND { rd: u8, rs: u8, rt: u8 },  // 0x24
    OR { rd: u8, rs: u8, rt: u8 },   // 0x25
    XOR { rd: u8, rs: u8, rt: u8 },  // 0x26
    NOR { rd: u8, rs: u8, rt: u8 },  // 0x27
    SLT { rd: u8, rs: u8, rt: u8 },  // 0x2A
    SLTU { rd: u8, rs: u8, rt: u8 }, // 0x2B

    ILLEGAL,
}

#[derive(PartialEq, Debug)]
pub enum ITypeOperation {
    //The first 6 bits (opcode) determines the operation.

    // Opcode 0x01 (REGIMM instructions), Branches
    BLTZ { rs: u8, immediate_se: u32 },   // Opcode 0x01, rt 0x00
    BGEZ { rs: u8, immediate_se: u32 },   // Opcode 0x01, rt 0x01
    BLTZAL { rs: u8, immediate_se: u32 }, // Opcode 0x01, rt 0x10
    BGEZAL { rs: u8, immediate_se: u32 }, // Opcode 0x01, rt 0x11

    // Branches
    BEQ { rs: u8, rt: u8, immediate_se: u32 }, // Opcode 0x04
    BNE { rs: u8, rt: u8, immediate_se: u32 }, // Opcode 0x05
    BLEZ { rs: u8, immediate_se: u32 },        // Opcode 0x06
    BGTZ { rs: u8, immediate_se: u32 },        // Opcode 0x07

    //Arithmetic and Logic
    ADDI { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x08
    ADDIU { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x09
    SLTI { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x0A
    SLTIU { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x0B
    ANDI { rt: u8, rs: u8, immediate: u32 },    // Opcode 0x0C
    ORI { rt: u8, rs: u8, immediate: u32 },     // Opcode 0x0D
    XORI { rt: u8, rs: u8, immediate: u32 },    // Opcode 0x0E

    // Loads
    LUI { rt: u8, immediate: u32 },            // Opcode 0x0F
    LB { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x20
    LH { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x21
    LWL { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x22
    LW { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x23
    LBU { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x24
    LHU { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x25
    LWR { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x26

    // Stores
    SB { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x28
    SH { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x29
    SWL { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x2A
    SW { rt: u8, rs: u8, immediate_se: u32 },  // Opcode 0x2B
    SWR { rt: u8, rs: u8, immediate_se: u32 }, // Opcode 0x2E

    ILLEGAL,
}
#[derive(PartialEq, Debug)]
pub enum JTypeOperation {
    J { target: u32 },   // Opcode 0x02
    JAL { target: u32 }, // Opcode 0x03
    ILLEGAL,
}

#[derive(PartialEq, Debug)]
pub enum Cop0Operation {
    MFC0 { rt: u8, rd: u8 },
    MTC0 { rt: u8, rd: u8 },
    RFE,
    COP0 { copfun: u32 },
    //bc#f bc#t branches of co-proccessors, don't know if needed right now
    ILLEGAL,
}

#[derive(PartialEq, Debug)]
pub enum GTEOperation {
    MFC2 { rt: u8, rd: u8 },
    CFC2 { rt: u8, rd: u8 },
    MTC2 { rt: u8, rd: u8 },
    CTC2 { rt: u8, rd: u8 },
    LWC2 { rt: u8, rs: u8, immediate_se: u32 },
    SWC2 { rt: u8, rs: u8, immediate_se: u32 },
    GTE,
    ILLEGAL,
}

impl fmt::Display for GTEOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use GTEOperation::*;
        match self {
            // Move from Coprocessor 2 Data Register
            MFC2 { rt, rd } => {
                write!(f, "mfc2 {}, {}", register_name(*rt), cop2_data_register_name(*rd))
            }
            // Move from Coprocessor 2 Control Register
            CFC2 { rt, rd } => {
                write!(f, "cfc2 {}, {}", register_name(*rt), cop2_control_register_name(*rd))
            }
            // Move to Coprocessor 2 Data Register
            MTC2 { rt, rd } => {
                write!(f, "mtc2 {}, {}", register_name(*rt), cop2_data_register_name(*rd))
            }
            // Move to Coprocessor 2 Control Register
            CTC2 { rt, rd } => {
                write!(f, "ctc2 {}, {}", register_name(*rt), cop2_control_register_name(*rd))
            }
            // Load Word to Coprocessor 2
            LWC2 { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lwc2 {}, 0x{:04x}({})",
                    cop2_data_register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            // Store Word from Coprocessor 2
            SWC2 { rt, rs, immediate_se } => {
                write!(
                    f,
                    "swc2 {}, 0x{:04x}({})",
                    cop2_data_register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            // GTE operation
            GTE => {
                write!(f, "gte operation")
            }
            ILLEGAL => {
                write!(f, "illegal")
            }
        }
    }
}

impl fmt::Display for Cop0Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Cop0Operation::*;
        match self {
            // Move from Coprocessor 0
            MFC0 { rt, rd } => {
                write!(f, "mfc0 {}, {}", register_name(*rt), cop0_register_name(*rd))
            }
            // Move to Coprocessor 0
            MTC0 { rt, rd } => {
                write!(f, "mtc0 {}, {}", register_name(*rt), cop0_register_name(*rd))
            }

            // Restore from Exception
            RFE => write!(f, "rfe"),

            // Coprocessor 0 operations (unspecified)
            COP0 { copfun } => write!(f, "cop0 operation 0x{:08x}", copfun),

            // Illegal instruction
            ILLEGAL => write!(f, "illegal"),
        }
    }
}

impl fmt::Display for JTypeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use JTypeOperation::*;
        match self {
            J { target } => write!(f, "j 0x{:08x}", target),
            JAL { target } => write!(f, "jal 0x{:08x}", target),
            ILLEGAL => write!(f, "illegal"),
        }
    }
}

impl fmt::Display for ITypeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ITypeOperation::*;
        match self {
            // Opcode 0x01 (REGIMM instructions), Branches
            BLTZ { rs, immediate_se } => {
                write!(f, "bltz {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }
            BGEZ { rs, immediate_se } => {
                write!(f, "bgez {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }
            BLTZAL { rs, immediate_se } => {
                write!(f, "bltzal {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }
            BGEZAL { rs, immediate_se } => {
                write!(f, "bgezal {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }

            // Branches
            BEQ { rs, rt, immediate_se } => {
                write!(
                    f,
                    "beq {}, {}, 0x{}",
                    register_name(*rs),
                    register_name(*rt),
                    format_offset(*immediate_se)
                )
            }
            BNE { rs, rt, immediate_se } => {
                write!(
                    f,
                    "bne {}, {}, 0x{}",
                    register_name(*rs),
                    register_name(*rt),
                    format_offset(*immediate_se)
                )
            }
            BLEZ { rs, immediate_se } => {
                write!(f, "blez {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }
            BGTZ { rs, immediate_se } => {
                write!(f, "bgtz {}, 0x{}", register_name(*rs), format_offset(*immediate_se))
            }

            // Arithmetic and Logic
            ADDI { rt, rs, immediate_se } => {
                write!(
                    f,
                    "addi {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate_se as i16
                )
            }
            ADDIU { rt, rs, immediate_se } => {
                write!(
                    f,
                    "addiu {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate_se as i16
                )
            }
            SLTI { rt, rs, immediate_se } => {
                write!(
                    f,
                    "slti {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate_se as i16
                )
            }
            SLTIU { rt, rs, immediate_se } => {
                write!(
                    f,
                    "sltiu {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate_se as i16
                )
            }
            ANDI { rt, rs, immediate } => {
                write!(
                    f,
                    "andi {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate as u16
                )
            }
            ORI { rt, rs, immediate } => {
                write!(
                    f,
                    "ori {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate as u16
                )
            }
            XORI { rt, rs, immediate } => {
                write!(
                    f,
                    "xori {}, {}, 0x{:04x}",
                    register_name(*rt),
                    register_name(*rs),
                    *immediate as u16
                )
            }

            // Loads
            LUI { rt, immediate } => {
                write!(f, "lui {}, 0x{:04x}", register_name(*rt), *immediate as u16)
            }
            LB { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lb {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LH { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lh {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LWL { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lwl {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LW { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lw {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LBU { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lbu {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LHU { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lhu {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            LWR { rt, rs, immediate_se } => {
                write!(
                    f,
                    "lwr {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }

            // Stores
            SB { rt, rs, immediate_se } => {
                write!(
                    f,
                    "sb {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            SH { rt, rs, immediate_se } => {
                write!(
                    f,
                    "sh {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            SWL { rt, rs, immediate_se } => {
                write!(
                    f,
                    "swl {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            SW { rt, rs, immediate_se } => {
                write!(
                    f,
                    "sw {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }
            SWR { rt, rs, immediate_se } => {
                write!(
                    f,
                    "swr {}, 0x{:04x}({})",
                    register_name(*rt),
                    *immediate_se as i16,
                    register_name(*rs)
                )
            }

            ILLEGAL => {
                write!(f, "illegal")
            }
        }
    }
}

impl fmt::Display for RTypeOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RTypeOperation::*;
        match self {
            // Shifts with immediate shift amount
            SLL { rd, rt, shamt } => {
                write!(f, "sll {}, {}, {}", register_name(*rd), register_name(*rt), shamt)
            }
            SRL { rd, rt, shamt } => {
                write!(f, "srl {}, {}, {}", register_name(*rd), register_name(*rt), shamt)
            }
            SRA { rd, rt, shamt } => {
                write!(f, "sra {}, {}, {}", register_name(*rd), register_name(*rt), shamt)
            }
            // Shifts with variable shift amount
            SLLV { rd, rt, rs } => {
                write!(
                    f,
                    "sllv {}, {}, {}",
                    register_name(*rd),
                    register_name(*rt),
                    register_name(*rs)
                )
            }
            SRLV { rd, rt, rs } => {
                write!(
                    f,
                    "srlv {}, {}, {}",
                    register_name(*rd),
                    register_name(*rt),
                    register_name(*rs)
                )
            }
            SRAV { rd, rt, rs } => {
                write!(
                    f,
                    "srav {}, {}, {}",
                    register_name(*rd),
                    register_name(*rt),
                    register_name(*rs)
                )
            }
            // Jumps
            JR { rs } => {
                write!(f, "jr {}", register_name(*rs))
            }
            JALR { rd, rs } => {
                write!(f, "jalr {}, {}", register_name(*rd), register_name(*rs))
            }
            // System calls
            SYSCALL => {
                write!(f, "syscall")
            }
            BREAK => {
                write!(f, "break")
            }
            // HI/LO register moves
            MFHI { rd } => {
                write!(f, "mfhi {}", register_name(*rd))
            }
            MTHI { rs } => {
                write!(f, "mthi {}", register_name(*rs))
            }
            MFLO { rd } => {
                write!(f, "mflo {}", register_name(*rd))
            }
            MTLO { rs } => {
                write!(f, "mtlo {}", register_name(*rs))
            }
            // Multiply and Divide
            MULT { rs, rt } => {
                write!(f, "mult {}, {}", register_name(*rs), register_name(*rt))
            }
            MULTU { rs, rt } => {
                write!(f, "multu {}, {}", register_name(*rs), register_name(*rt))
            }
            DIV { rs, rt } => {
                write!(f, "div {}, {}", register_name(*rs), register_name(*rt))
            }
            DIVU { rs, rt } => {
                write!(f, "divu {}, {}", register_name(*rs), register_name(*rt))
            }
            // Arithmetic and Logic Instructions
            ADD { rd, rs, rt } => {
                write!(
                    f,
                    "add {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            ADDU { rd, rs, rt } => {
                write!(
                    f,
                    "addu {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            SUB { rd, rs, rt } => {
                write!(
                    f,
                    "sub {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            SUBU { rd, rs, rt } => {
                write!(
                    f,
                    "subu {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            AND { rd, rs, rt } => {
                write!(
                    f,
                    "and {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            OR { rd, rs, rt } => {
                write!(
                    f,
                    "or {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            XOR { rd, rs, rt } => {
                write!(
                    f,
                    "xor {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            NOR { rd, rs, rt } => {
                write!(
                    f,
                    "nor {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            SLT { rd, rs, rt } => {
                write!(
                    f,
                    "slt {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            SLTU { rd, rs, rt } => {
                write!(
                    f,
                    "sltu {}, {}, {}",
                    register_name(*rd),
                    register_name(*rs),
                    register_name(*rt)
                )
            }
            ILLEGAL => {
                write!(f, "illegal")
            }
        }
    }
}

fn format_offset(offset: u32) -> String {
    let offset = offset as i16; // Convert to signed 16-bit integer
    format!("{:04x}", offset)
}
