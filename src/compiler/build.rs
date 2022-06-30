use super::assembler::Assembler;
use super::preproc::PreProcessor;

use crate::{read_file, write_file_bin};

pub fn build(
    input_fname: String,
    output_fname: String,
    base_path: Option<String>,
    include_path: Option<String>,
    verbosity: bool,
) {
    let mut preproc = PreProcessor::new(
        read_file(input_fname),
        "macro_rules.m4".to_string(),
        base_path,
    );
    let code = preproc.preprocess("output.zas".to_string(), include_path, verbosity);

    let mut assembler= Assembler::new(code);
    assembler.assembly(verbosity);

    write_file_bin(output_fname, assembler.get_binary());
}
