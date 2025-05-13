use disassembler::disassembler;
use std::fs::File;
fn main() {
    let mut bin_file = File::open("./binaries/SCPH1001.BIN").unwrap();
    disassembler::file_output(&bin_file, "./binaries/disassembled/bios1.txt".to_string());
}
