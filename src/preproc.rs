use colored::*;
use crate::file::write_file;

use std::process::Command;


#[derive(Debug, Clone)]
struct Macro {
    pub name: String,
    pub value: String
}


pub struct PreProcessor {
    data: Vec<String>,
    macro_rules: Vec<Macro>,
    mrfname: String
}

impl PreProcessor {
    pub fn new(data: Vec<String>, mrfname: String) -> PreProcessor {
        PreProcessor {
            data,
            macro_rules: Vec::new(),
            mrfname
        }
    }

    pub fn preprocess(&mut self, input_file: String) -> Vec<String> {
        self.parse_macros();
        self.make_m4_rules();

        let preproc = Command::new("m4")
                                        .arg("-P")
                                        .arg(self.mrfname.clone())
                                        .arg(input_file)
                                        .output()
                                        .expect("Failed to preprocess with M4");

        if !preproc.status.success() {
            eprintln!("{}: failed to preprocess with M4", "ERROR".bright_red());
            std::process::exit(1);
        }

        let output = String::from_utf8(preproc.stdout).unwrap();
        let _ = Command::new("rm")
                                .arg(self.mrfname.clone())
                                .output()
                                .expect("Failed to delete tmp files");
                                
        output.split('\n').map(|s| s.to_string()).collect()
    }

    fn parse_macros(&mut self) {
        let mut index: usize = 0;
        while index < self.data.len() {
            let mut line = self.data[index].clone();
            index += 1;

            if !line.starts_with('.') { continue; }

            line = line.replace("\t", " ");
            let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
            if tokens[0] == ".DEF" {
                self.macro_rules.push(Macro { name: tokens[1].clone(), value: tokens[2].clone() });
            } else {
                eprintln!("{}: unknown preprocessot derictive: {}", "ERROR".bright_red(), tokens[0].italic());
                std::process::exit(1);
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