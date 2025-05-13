use ps::cpu::mipsr3000::run_instruction;
use ps::playstation::PlayStation;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, fs};
fn main() {
    //env::set_var("RUST_BACKTRACE", "1");

    let bios = fs::read("./binaries/SCPH1001.BIN").unwrap().into_boxed_slice();

    let mut ps = PlayStation::new(bios);
    //println!("{:08x}", ps.read_word(0xbfc06f0c));

    let output_path = "pc_trace_rust.txt";
    let file = File::create(output_path).unwrap();
    let mut writer = BufWriter::new(file);

    println!("Starting emulation trace...");
    const START_PC: u32 = 0xbfc0127b;

    // Tracking variables
    let mut recording = false;
    let mut instruction_count = 0;
    let mut recorded_count = 0;
    let mut visited_pcs = HashSet::new();

    // Execute instructions until we've recorded enough after the target PC
    while instruction_count < 1_000_000 && recorded_count < 5000 {
        let current_pc = ps.cpu.pc;

        // Start recording once we hit our target PC
        if current_pc == START_PC {
            recording = true;
            println!("Found target PC 0xBFC003CC at instruction #{}", instruction_count);
        }

        // Record this PC if we're in recording mode
        if recording {
            writeln!(writer, "{}: 0x{:08X}", recorded_count, current_pc);
            recorded_count += 1;
            visited_pcs.insert(current_pc);
        }

        // Execute the instruction
        run_instruction(&mut ps);
        instruction_count += 1;

        // Break if we've been running too long without finding the target
        /*if !recording && instruction_count > 100_000 {
            writeln!(writer, "Couldn't find target PC after 100,000 instructions");
            break;
        }*/
    }

    // Write summary
    writeln!(writer, "\n------ EXECUTION SUMMARY ------");
    writeln!(writer, "Instructions executed until target: {}", instruction_count - recorded_count);
    writeln!(writer, "Instructions recorded after target: {}", recorded_count);
    writeln!(writer, "Unique PC addresses visited after target: {}", visited_pcs.len());
}
