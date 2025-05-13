use ps::cpu::instruction;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn disassemble(ins: &instruction::Instruction) -> String {
    use instruction::Operation::*;
    let op = ps::cpu::instruction::Instruction::operation(&ins);
    match op {
        RType(r_op) => r_op.to_string(),
        IType(i_op) => i_op.to_string(),
        JType(j_op) => j_op.to_string(),
        COP0(cop0_op) => cop0_op.to_string(),
        GTE(gte_op) => gte_op.to_string(),
        NOOP => format!("noop"),
        ILLEGAL => format!("illegal"),
    }
}

pub fn file_output(mut bin_file: &File, output_path: String) -> File {
    use instruction::Instruction;
    let mut file_buf: Vec<u8> = Vec::new();
    let mut output = File::create(output_path).unwrap();
    bin_file.read_to_end(&mut file_buf).unwrap();
    for i in (0..file_buf.len()).step_by(4) {
        let ins = Instruction(read_u32_from_file(&file_buf, i));
        let line = format!("0x{:08X}, 0x{:08X}, {}\n", i, ins.all(), disassemble(&ins));
        output.write_all(line.as_bytes()).unwrap();
    }
    output
}

pub fn read_u32_from_file(file_buf: &Vec<u8>, address: usize) -> u32 {
    let mut bytes: [u8; 4] = [0; 4];

    for i in 0..=3 {
        bytes[i] = file_buf[(address as usize + i) as usize];
    }
    u32::from_le_bytes(bytes)
}
