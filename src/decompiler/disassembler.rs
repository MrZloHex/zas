use super::dictionary::Dictionary;

pub struct Disassembler {
    binary: Vec<u8>,
    dict: Dictionary<'static>
}

impl Disassembler {
    pub fn new(binary: Vec<u8>) -> Self {
        Self {
            binary,
            dict: Dictionary::new()
        }
    }

    pub fn disassembly(&mut self) {

    }
}
