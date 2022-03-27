use crate::dictionary::Dictionary;
use colored::*;

use std::collections::HashMap;

pub struct Compiler {
    data: Vec<String>,
    line: usize,
    binary: Vec<u8>,
    dict: Dictionary<'static>,
    labels: HashMap<String, u16>,
    label_address: u16
}

impl Compiler {
    pub fn new(data: Vec<String>) -> Compiler {
        Compiler {
            data,
            line: 0,
            binary: Vec::new(),
            dict: Dictionary::new(),
            labels: HashMap::new(),
            label_address: 0
        }
    }

    pub fn compile(&mut self, verbosity: bool) {
        if verbosity { println!("{}:\t{}\t{}\t{}", "INFO".cyan(), "INSTR".bright_white().bold(), "OPCODE".bright_white().bold(), "IMM".bright_white().bold()) }
    
        for cycle in 0..2 {
            println!("{}", cycle);
            while self.line < self.data.len() {
                let mut line = self.data[self.line].clone();
                self.line += 1;

                if line.is_empty() { continue; }

                line = line.replace("\t", " ");

                let mut tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

                if tokens.is_empty() { continue; }

                if tokens[0].starts_with(';') { continue; }

                let is_label = if tokens[0].ends_with(':') {
                    tokens[0].pop();
                    let first_char = tokens[0].chars().next().unwrap();
                    if first_char.is_numeric() || first_char == '.' {
                        eprintln!("{}: labels can't starts with {} or {} - `{}`", "ERROR".bright_red(), "NUMBER".bold(), "DOT".bold(), tokens[0]);
                        std::process::exit(1);
                    }
                    if verbosity {
                        println!("GOT A LABEL `{}` at address {:X}", tokens[0], self.label_address)
                    }
                    if cycle == 0 {
                        self.labels.insert(tokens[0].clone(), self.label_address);
                    }
                    true
                } else {
                    false
                };

                // TODO: MAKE PRETIFY AND OPTIMISE
                let (instr, op_data, sec_byte_str) = if is_label {
                    print!("INSTR: {}", tokens[1]);
                    let (op_data, byte) = self.dict.get_opcode(tokens[1].as_str());
                    if byte {
                        (tokens[1].clone(), op_data, Some(tokens[2].clone()))
                    } else {
                        (tokens[1].clone(), op_data, None)
                    }
                } else {
                    print!("INSTR: {}", tokens[0]);
                    let (op_data, byte) = self.dict.get_opcode(tokens[0].as_str());
                    if byte {
                        (tokens[0].clone(), op_data, Some(tokens[1].clone()))
                    } else {
                        (tokens[0].clone(), op_data, None)
                    }
                };

                println!(" OP {:?} IMM {:?}", op_data, sec_byte_str);

                let op = match op_data {
                    Some(op) => *op,
                    None => {
                        eprintln!("{}: no such instruction {} at line {}", "ERROR".bright_red(), instr.bold(), self.line);
                        std::process::exit(1);
                    }
                };

                match sec_byte_str {
                    Some(imm_str) => {
                        if cycle == 0 {
                            self.label_address += 1;
                        } else {
                            self.binary.push(op);
                        }
                        let imm = if imm_str.chars().next().unwrap().is_numeric() {
                            let (radix, val_str, error_str) = if imm_str.starts_with("0x") {
                                let hex_val = imm_str.strip_prefix("0x").unwrap();
                                (16, hex_val, "HEX".to_string())
                            } else if imm_str.starts_with("0b") {
                                let bin_val = imm_str.strip_prefix("0b").unwrap();
                                (2, bin_val, "BIN".to_string())
                            } else if imm_str.starts_with("0o") {
                                let oct_val = imm_str.strip_prefix("0o").unwrap();
                                (8, oct_val, "OCT".to_string())
                            } else {
                                let dec_val = match imm_str.strip_prefix("0d") {
                                    Some(val) => val,
                                    None => imm_str.as_str()
                                };
                                (10, dec_val, "DEC".to_string())
                            };
                            let imm: u8 = match u8::from_str_radix(val_str, radix) {
                                Ok(val) => val,
                                Err(_) => {
                                    eprintln!("{}: incorrect immediate {} value {} at line {}", "ERROR".bright_red(), error_str, tokens[1].bold(), self.line);
                                    std::process::exit(1);
                                }
                            };
                            imm
                        } else if cycle != 0 {
                            if imm_str.contains('%') {
                                let label_toks: Vec<&str> = imm_str.split('%').collect();
                                if label_toks[1].is_empty() {
                                    eprintln!("{}: not specifed SIGNIFICANCE of address at line {}", "ERROR".bright_red(), self.line);
                                    std::process::exit(1);
                                }
                                let imm = match label_toks[1] {
                                    "H" => {
                                        let address = match self.labels.get(label_toks[0]) {
                                            Some(&add) => add,
                                            None => {
                                                eprintln!("{}: no such label {} at line {}", "ERROR".bright_red(), label_toks[0].bold(), self.line);
                                                std::process::exit(1);
                                            }
                                        };
                                        (address >> 8) as u8
                                    },
                                    "L" => {
                                        let address = match self.labels.get(label_toks[0]) {
                                            Some(&add) => add,
                                            None => {
                                                eprintln!("{}: no such label {} at line {}", "ERROR".bright_red(), label_toks[0].bold(), self.line);
                                                std::process::exit(1);
                                            }
                                        };
                                        address as u8
                                    },
                                    _ => {
                                        eprintln!("{}: undefined SIGNIFICANCE of address at line {}", "ERROR".bright_red(), self.line);
                                        std::process::exit(1);
                                    }
                                };
                                imm
                            } else {
                                // FOR FUTURE for simple LABELS and maybe SOME DATA
                                // TODO: normal error message - formatted
                                eprintln!("ERROR: not specifed SIGNIFICANCE of address");
                                std::process::exit(1);
                            }
                        } else {
                            0
                        };
                        if cycle == 0 {
                            self.label_address += 1;
                        } else {
                            self.binary.push(imm);
                        }
                    },
                    None => {
                        if cycle == 0 {
                            self.label_address += 1;
                        } else {
                            self.binary.push(op)
                        }
                    }
                }

            }
            self.line = 0;
        }
    }

    pub fn get_binary(&self) -> Vec<u8> {
        self.binary.clone()
    }
}

fn correct_first_char(ch: char) -> bool {
    match ch {
        ';' => true,
        'A'..='Z' => true,
        'a'..='z' => true,
        '_' => true,
        _ => false
    }
}
