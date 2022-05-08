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
        // if verbosity {  println!("{}:\t{}\t{}\t{}", "INFO".cyan(), "INSTR".bright_white().bold(), "OPCODE".bright_white().bold(), "IMM".bright_white().bold()) }
    
        let mut is_sec_text = false;
        let mut is_sec_data = false;
        let mut is_sec_bss = false;

        // 1st cycle - parsing all offsets and labels
        // 2nd cycle - actual compliling
        for cycle in 0..2 {
            while self.line < self.data.len() {
                let mut line = self.data[self.line].clone();
                self.line += 1;

                if line.is_empty() { continue; }

                line = line.replace("\t", " ");
                let mut tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

                // EMPTY
                if tokens.is_empty() { continue; }
                if tokens[0].starts_with(';') || tokens[0].starts_with('.') { continue; }

                // Defining code sections
                if tokens[0] == "SECTION" {
                    if tokens.len() < 2 {
                        eprintln!("{}: undefined section at line {}", "ERROR".bright_red(), self.line);
                        std::process::exit(1);
                    }

                    // TODO: Somehow opitimise and refoactor this
                    if tokens[1] == "TEXT" {
                        if !is_sec_bss && !is_sec_data && !is_sec_text {
                            is_sec_text = true;
                        } else {
                            eprintln!("{}: cannot be defined section in another at line {}", "ERROR".bright_red(), self.line);
                            std::process::exit(1);
                        }
                    } else if tokens[1] == "BSS" {
                        if !is_sec_bss && !is_sec_data && !is_sec_text {
                            is_sec_bss = true;
                        } else {
                            eprintln!("{}: cannot be defined section in another at line {}", "ERROR".bright_red(), self.line);
                            std::process::exit(1); 
                        }
                    } else if tokens[1] == "DATA" {
                        if !is_sec_bss && !is_sec_data && !is_sec_text {
                            is_sec_data = true;
                        } else {
                            eprintln!("{}: cannot be defined section in another at line {}", "ERROR".bright_red(), self.line);
                            std::process::exit(1);
                        }
                    } else {
                        eprintln!("{}: unknown section {} at line {}", "ERROR".bright_red(), tokens[1].italic(), self.line);
                        std::process::exit(1);
                    }
                    continue;
                } else if tokens[0] == "END" {
                    if !is_sec_bss && !is_sec_data && !is_sec_text {
                        eprintln!("{}: ending section without declaring it at line {}", "ERROR".bright_red(), self.line);
                        std::process::exit(1);
                    } else {
                        is_sec_bss = false;
                        is_sec_data = false;
                        is_sec_text = false;
                    }
                    continue;
                }
                
                if is_sec_text {
                    let is_label = if tokens[0].ends_with(':') {
                        tokens[0].pop();
                        let first_char = tokens[0].chars().next().unwrap();
                        if first_char.is_numeric() || first_char == '.' {
                            eprintln!("{}: labels can't starts with {} or {} - `{}`", "ERROR".bright_red(), "NUMBER".bold(), "DOT".bold(), tokens[0]);
                            std::process::exit(1);
                        }
                        if cycle == 0 {
                            self.labels.insert(tokens[0].clone(), self.label_address);
                            if verbosity {
                                println!("GOT A LABEL `{}` at address {:X}", tokens[0], self.label_address)
                            }
                        }
                        true
                    } else {
                        false
                    };

                    if tokens.len() < 2 && is_label {
                        eprintln!("{}: you can't point with label at nothing", "ERROR".bright_red());
                        std::process::exit(1);
                    }

                    // TODO: MAKE PRETIFY AND OPTIMISE
                    let (instr, op_data, sec_byte_str) = if is_label {
                        // print!("INSTR: {}", tokens[1]);
                        let (op_data, byte) = self.dict.get_opcode(tokens[1].as_str());
                        if byte {
                            (tokens[1].clone(), op_data, Some(tokens[2].clone()))
                        } else {
                            (tokens[1].clone(), op_data, None)
                        }
                    } else {
                        // print!("INSTR: {}", tokens[0]);
                        let (op_data, byte) = self.dict.get_opcode(tokens[0].as_str());
                        if byte {
                            (tokens[0].clone(), op_data, Some(tokens[1].clone()))
                        } else {
                            (tokens[0].clone(), op_data, None)
                        }
                    };
                    
                    if verbosity {
                        println!(" OP {:?} IMM {:?}", op_data, sec_byte_str);
                    }

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
                                indetify_number(imm_str)
                            } else if cycle != 0 {
                                if imm_str.contains('%') {
                                    let label_toks: Vec<String> = imm_str.split('%').map(|s| s.to_string()).collect();
                                    if label_toks[1].is_empty() {
                                        eprintln!("{}: not specifed SIGNIFICANCE of address at line {}", "ERROR".bright_red(), self.line);
                                        std::process::exit(1);
                                    }
                                    let address = if label_toks[0].contains('+') || label_toks[0].contains('-') {
                                        self.eval_address(label_toks[0].clone())
                                    } else {
                                        self.get_address(&label_toks[0])
                                    };

                                    let imm = match label_toks[1].as_str() {
                                        "H" => (address >> 8) as u8,
                                        "L" => address as u8,
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
                } else if is_sec_data {
                    if tokens[0].ends_with(':') {
                        tokens[0].pop();
                        if cycle == 0 {
                            self.labels.insert(tokens[0].clone(), self.label_address);
                            if verbosity {
                                // println!("GOT A DATA `{}` at address {:X}", tokens[0], self.label_address)
                            }
                        }
                        if tokens.len() < 2 {
                            eprintln!("{}: value of data wasn't provide at line {}", "ERROR".bright_red(), self.line);
                            std::process::exit(1);
                        }
                        for i in tokens.iter().skip(1) {
                            if cycle == 1 {
                                self.binary.push(indetify_number((*i).clone()));
                            }
                            self.label_address += 1;
                        }
                    } else {
                        eprintln!("{}: data section is only for labels", "ERROR".bright_red());
                        std::process::exit(69);
                    }
                } else if is_sec_bss {
                    if tokens[0].ends_with(':') {
                        tokens[0].pop();
                        if cycle == 0 {
                            self.labels.insert(tokens[0].clone(), self.label_address);
                            if verbosity {
                                // println!("GOT A BSS `{}` at address {:X}", tokens[0], self.label_address)
                            }
                        }
                        if tokens.len() < 2 {
                            eprintln!("{}: quantity of bytes wasn't provide at line {}", "ERROR".bright_red(), self.line);
                            std::process::exit(1);
                        }
                        let resb = indetify_number(tokens[1].clone());
                        for _ in 0..resb {
                            if cycle == 1 {
                                self.binary.push(0);
                            }
                            self.label_address += 1;
                        }
                    } else {
                        eprintln!("{}: data section is only for labels", "ERROR".bright_red());
                        std::process::exit(69);
                    }
                }
            }
            self.line = 0;
        }
    }

    fn get_address(&self, label: &String) -> u16 {
        match self.labels.get(label) {
            Some(&add) => add,
            None => {
                eprintln!("{}: no such label {} at line {}", "ERROR".bright_red(), label.bold(), self.line);
                std::process::exit(1);
            }
        }
    }

    fn eval_address(&self, cl: String) -> u16 {
        let mut complex_label = cl;
        let mut add = true;
        let mut address: u16 = 0;
        while complex_label.contains('+') || complex_label.contains('-') {
            let label = complex_label;

            let mut next_add = true;
            let mut t = label.split_once('+');
            if let None = t {
                if label.contains('-') {
                    t = label.split_once('-');
                    next_add = false;
                } else {
                    complex_label = label;
                    break;
                }
            } else if t.unwrap().0.contains('-') {
                t = label.split_once('-');
                next_add = false;
            }

            let terms = t.unwrap();
            complex_label = terms.1.to_string();

            let val = if terms.0.chars().next().unwrap().is_numeric() {
                terms.0.parse::<u16>().unwrap()
            } else {
                self.get_address(&(terms.0.to_string()))
            };
        
            if add {
                address += val;
            } else {
                address -= val;
            }
            add = next_add;
        }
        let val = if complex_label.chars().next().unwrap().is_numeric() {
            complex_label.parse::<u16>().unwrap()
        } else {
            self.get_address(&complex_label)
        };

        if add {
            address += val;
        } else {
            address -= val;
        }
        address
    }

    pub fn get_binary(&self) -> Vec<u8> {
        self.binary.clone()
    }
}

fn indetify_number(imm_str: String) -> u8 {
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
            eprintln!("{}: incorrect immediate {} value {}", "ERROR".bright_red(), error_str, imm_str.bold());
            std::process::exit(1);
        }
    };
    imm
}
