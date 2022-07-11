mod disassembler;
mod dictionary;

use disassembler::Disassembler;

use crate::file::read_file_bin;

pub fn disassembly(binary_fname: String) {
    let mut disassembler = Disassembler::new(read_file_bin(binary_fname));
    disassembler.disassembly();
}
