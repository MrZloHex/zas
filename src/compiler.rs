use crate::dictionary::Dictionary;
use colored::*;

pub struct Compiler {
    data: Vec<String>,
    line: usize,
    binary: Vec<u8>,
    dict: Dictionary<'static>
}

impl Compiler {
    pub fn new(data: Vec<String>) -> Compiler {
        Compiler {
            data,
            line: 0,
            binary: Vec::new(),
            dict: Dictionary::new()
        }
    }

    pub fn compile(&mut self, verbosity: bool) {
        if verbosity { println!("{}:\t{}\t{}\t{}", "INFO".cyan(), "INSTR".bright_white().bold(), "OPCODE".bright_white().bold(), "IMM".bright_white().bold()) }
        while self.line < self.data.len() {
            let mut line = self.data[self.line].clone();
            self.line += 1;

            if line.is_empty() { continue; }

            while !correct_first_char(line.chars().next().unwrap()) {
                line.remove(0);
            }

            if line.chars().next().unwrap() == ';' { continue; }
            println!("LINE: `{}`", line);

            if line.contains(':') {
                
            }

            let mut tokens: Vec<&str> = Vec::new();
            let instr = if line.contains(' ') {
                tokens = line.split_whitespace().collect();
                tokens[0]
            } else {
                line.as_str()
            };

            let (op_data, sec_byte) = self.dict.get_opcode(instr);
            let op = match op_data {
                Some(op) => *op,
                None => {
                    eprintln!("{}: no such instruction {} at line {}", "ERROR".bright_red(), instr.bold(), self.line+1);
                    std::process::exit(1);
                }
            };
            if sec_byte {
                let imm: u8 = match u8::from_str_radix(tokens[1], 16) {
                    Ok(val) => val,
                    Err(_) => {
                        eprintln!("{}: incorrect immediate value {} at line {}", "ERROR".bright_red(), tokens[1].bold(), self.line+1);
                        std::process::exit(2);
                    }
                };
                if verbosity {
                    println!("\t{}\t{:>0w$X}\t{:>0w$X}", tokens[0], op, imm, w=2);
                }

                self.binary.push(op);
                self.binary.push(imm);
            } else
                if verbosity {
                    println!("\t{}\t{:>0w$X}", line, op, w=2);
                } {
                self.binary.push(op);
            }

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
