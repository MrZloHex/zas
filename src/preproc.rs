use colored::*;
use crate::file::{write_file, read_file};

use std::process::Command;


#[derive(Debug, Clone)]
struct Macro {
    pub name: String,
    pub value: String
}

impl Macro {
    pub fn new() -> Macro {
        Macro { name: String::new(), value: String::new() }
    }

    pub fn make(name: String, value: String) -> Macro {
        Macro { name, value }
    }
}

pub struct PreProcessor {
    data: Vec<String>,
    macro_rules: Vec<Macro>,
    mrfname: String,
    base_path: String
}

impl PreProcessor {
    pub fn new(data: Vec<String>, mrfname: String, base_path: String) -> PreProcessor {
        PreProcessor {
            data,
            macro_rules: Vec::new(),
            mrfname,
            base_path
        }
    }

    pub fn preprocess(&mut self, tmp_fname: String) -> Vec<String> {
        self.parse_macros();
        self.make_m4_rules();
        write_file(tmp_fname.clone(), self.data.clone());

        let preproc = Command::new("m4")
                                        .arg("-P")
                                        .arg(self.mrfname.clone())
                                        .arg(tmp_fname)
                                        .output()
                                        .expect("Failed to preprocess with M4");

        if !preproc.status.success() {
            eprintln!("{}: failed to preprocess with M4", "ERROR".bright_red());
            std::process::exit(1);
        }

        let output = String::from_utf8(preproc.stdout).unwrap();
        println!("Preproc output:\n{}\nEND", output);

        // REMOVE M4 RULES FILE

        // let _ = Command::new("rm")
        //                         .arg(self.mrfname.clone())
        //                         .output()
        //                         .expect("Failed to delete tmp files");
                                
        output.split('\n').map(|s| s.to_string()).collect()
    }

    fn parse_macros(&mut self) {
        let mut index: usize = 0;

        let mut is_long_macro = false;
        let mut first_macro_line = true;
        let mut long_macro = Macro::new();

        let mut is_section = false;

        while index < self.data.len() {
            let mut line = self.data[index].clone();
            index += 1;

            if line.starts_with("SECTION") {
                is_section = true;
            } else if line.starts_with("END") {
                is_section = false;
            }

            if !line.starts_with('.') && !is_long_macro || is_section { continue; }

            if !is_long_macro {
                line = line.replace("\t", " ");
            }
            
            let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

            if tokens[0].starts_with(';') { continue; }

            if tokens[0] == ".DEF" {
                if tokens.len() > 2 {
                    if tokens[2].is_empty() {
                        eprintln!("{}: value of macro {} is {}", "ERROR".bright_red(), tokens[1].italic(), "NULL".bold());
                        std::process::exit(1);
                    }
                    self.macro_rules.push(Macro::make(tokens[1].clone(), tokens[2].clone()));
                } else if tokens.len() == 2 {

                    is_long_macro = true;
                    long_macro.name = tokens[1].clone();
                } else {
                    eprintln!("{}: not defined macro name", "ERROR".bright_red());
                    std::process::exit(1);
                }
            } else if tokens[0] == ".ENDDEF" {
                is_long_macro = false;
                long_macro.value.pop();
                self.macro_rules.push(long_macro);
                long_macro = Macro::new();
                first_macro_line = true;    
            } else if tokens[0] == ".INCLUDE" {
                if tokens.len() < 1 {
                    eprintln!("{}: filepath to include doesn't provide at line {}", "ERROR".bright_red(), index);
                    std::process::exit(1);
                }
                self.data.remove(index-1);
                for (i, l) in read_file(format!("{}/{}", self.base_path, tokens[1].clone())).iter().enumerate() {
                    self.data.insert(index+i, (*l).clone());
                }
            } else {
                if is_long_macro && !tokens[0].starts_with('.') {
                    if first_macro_line {
                        while !line.chars().next().unwrap().is_alphanumeric() {
                            line.remove(0);
                        }
                        first_macro_line = false;
                    }
                    long_macro.value.push_str(line.as_str());
                    long_macro.value.push('\n');
                } else {
                    eprintln!("{}: unknown preprocessor derictive: {}", "ERROR".bright_red(), tokens[0].italic());
                    std::process::exit(1);
                }
            }
        }
    }

    fn make_m4_rules(&mut self) {
        let mut m4 = Vec::new();
        for macro_rule in self.macro_rules.clone() {
            m4.push(format!("m4_define(`{}', `{}')m4_dnl", macro_rule.name, macro_rule.value))
        }
        write_file(self.mrfname.clone(), m4);
    }
}