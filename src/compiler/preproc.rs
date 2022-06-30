use crate::file::{is_such_file_in_path, read_file_in_path, write_file};
use colored::*;

use std::cmp::Ordering;
use std::process::Command;

#[derive(Debug, Clone)]
struct Macro {
    pub name:  String,
    pub value: String,
}

impl Macro {
    pub fn new() -> Self {
        Macro {
            name:  String::new(),
            value: String::new(),
        }
    }

    pub fn make(name: String, value: String) -> Self {
        Macro { name, value }
    }
}

pub struct PreProcessor {
    data:        Vec<String>,
    macro_rules: Vec<Macro>,
    mrfname:     String,
    base_path:   String,
}

impl PreProcessor {
    pub fn new(data: Vec<String>, mrfname: String, base_path: Option<String>) -> Self {
        Self {
            data,
            macro_rules: Vec::new(),
            mrfname,
            base_path: base_path.unwrap_or_else(|| "".to_string())
        }
    }

    pub fn preprocess(
        &mut self,
        tmp_fname: String,
        inc_dir: Option<String>
    ) -> Vec<String> {
        let inc_dir = inc_dir.unwrap_or_else(|| "".to_string());
        self.parse_macros(inc_dir);
        self.make_m4_rules();

        // Save all source code into one file to preprocess it wiht m4
        write_file(tmp_fname.clone(), self.data.clone());

        // Run M4 for preprocessing
        let preproc = match Command::new("m4")
            .arg("-P")
            .arg(self.mrfname.clone())
            .arg(tmp_fname.clone())
            .output() 
        {
            Ok(out) => out,
            Err(err) => Self::in_error(format!("failed to run M4 preprocess, because {:?}", err))
        };

        if !preproc.status.success() {
            Self::in_error("failed to preprocess with M4");
        }

        let output = match String::from_utf8(preproc.stdout) {
            Ok(out) => out,
            Err(err) => Self::in_error(format!("filed to parse output of M4 preprocess: `{:?}`", err.utf8_error()))
        };

        // REMOVE M4 RULES FILE
        let _ = if let Err(err) = Command::new("rm")
            .arg(self.mrfname.clone())
            .arg(tmp_fname)
            .output()
        {
            Self::in_error(format!("Failed to delete tmp files, because {:?}", err));
        };

        // Return vec of lines of preprocessed source code
        output.split('\n').map(|s| s.to_string()).collect()
    }

    fn parse_macros(&mut self, inc_dir: String) {
        let mut index: usize = 0;

        let mut is_long_macro    = false;
        let mut first_macro_line = true;
        let mut long_macro       = Macro::new();

        // let mut is_if_macro = false;
        // let mut if_macro    = Macro::new();
        // let mut else_macro  = Macro::new();

        let mut is_any_section = false;

        while index < self.data.len() {
            // Get wokring line
            let mut line = self.data[index].clone();
            index += 1;

            // Check location in any SECTION
            if line.starts_with("SECTION") {
                is_any_section = true;
            } else if line.starts_with("END") {
                is_any_section = false;
            }

            // if we aren't in LONG MACRO and skip line EXCEPT we have preprocessor operator
            // OR we are in any seciton of code or data
            if !line.starts_with('.') && !is_long_macro || is_any_section {
                continue;
            }
            
            if !is_long_macro {
                line = line.replace("\t", " ");
            }

            // split line to tokens(words)
            let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

            // If there are no tokens skip line
            if tokens.is_empty() {
                continue;
            }

            // Skip comments
            if tokens[0].starts_with(';') {
                continue;
            }
            
            // Process preprocessor operators
            match tokens[0].as_str() {
                ".DEFINE" => {
                    // Check for NAME of macro
                    if tokens.len() < 2 || tokens[1].is_empty() {
                        Self::in_error("not defined macro name");
                    }
                    self.macro_rules
                        .push(Macro::make(tokens[1].clone(), String::new()));
                },
                ".DEF" => {
                    // Check for enough quantity of arguments
                    // =2 => long macro
                    // >2 => macro
                    // <2 => without name -> error
                    match tokens.len().cmp(&2) {
                        Ordering::Greater => {
                            // check for NOT NULL macro value
                            if tokens[2].is_empty() {
                                Self::in_error(
                                    format!(
                                        "value of macro {} is {}",
                                        tokens[1].italic(),
                                        "NULL".bold()
                                    )
                                );
                            }
                            self.macro_rules
                                .push(Macro::make(tokens[1].clone(), tokens[2].clone()));
                        }
                        Ordering::Equal => {
                            is_long_macro = true;
                            long_macro.name = tokens[1].clone();
                        }
                        _ => {
                            Self::in_error("not defined macro name")
                        }
                    }
                },
                ".ENDDEF" => {
                    is_long_macro = false;
                    long_macro.value.pop();

                    self.macro_rules.push(long_macro);

                    long_macro = Macro::new();
                    first_macro_line = true;
                },
                ".INCLUDE" => {
                    if tokens.len() < 2 {
                        Self::in_error("filepath to include doesn't provided");
                    }
                    index -= 1;

                    // Substitued INCLUDE OPERATOR with contents of included file
                    self.data.remove(index);
                    if is_such_file_in_path(self.base_path.clone(), tokens[1].clone()) {
                        for (i, l) in read_file_in_path(self.base_path.clone(), tokens[1].clone())
                            .iter()
                            .enumerate()
                        {
                            self.data.insert(index + i, (*l).clone());
                        }
                    } else if is_such_file_in_path(inc_dir.clone(), tokens[1].clone()) {
                        for (i, l) in read_file_in_path(inc_dir.clone(), tokens[1].clone())
                            .iter()
                            .enumerate()
                        {
                            self.data.insert(index + i, (*l).clone());
                        }
                    } else {
                        println!("`{}{}`\n`{}{}`", self.base_path, tokens[1].clone(), inc_dir, tokens[1].clone());
                        Self::in_error(format!(
                            "no such file to include as `{}`",
                            tokens[1]
                        ));
                    }
                },
                _ => {
                    if is_long_macro && !tokens[0].starts_with('.') {
                        // For make same outline in POSTPREPROCESOR file
                        // IDK for what reasons I wanna this :)
                        if first_macro_line {
                            while !line.chars().next().unwrap().is_alphanumeric() {
                                line.remove(0);
                            }
                            first_macro_line = false;
                        }

                        long_macro.value.push_str(line.as_str());
                        long_macro.value.push('\n');
                    } else {
                        Self::in_error(format!(
                            "unknown preprocessor derictive: {}",
                            tokens[0].italic()
                        ));
                    }
                }
            }
        }
    }

    fn make_m4_rules(&mut self) {
        let mut m4 = Vec::new();
        for macro_rule in self.macro_rules.clone() {
            m4.push(format!(
                "m4_define(`{}', `{}')m4_dnl",
                macro_rule.name, macro_rule.value
            ))
        }
        write_file(self.mrfname.clone(), m4);
    }

    fn in_error<T: std::fmt::Display>(err: T) -> ! {
        eprintln!(
            "{}: {}",
            "ERROR".bright_red(),
            err
        );
        std::process::exit(1)
    }
}   
