use colored::*;

use std::fs::File;
use std::io::{BufRead, BufReader, Write};


pub fn read_file(filename: String) -> Vec<String> {
    let file = match File::open(filename.clone()) {
        Ok(fl) => fl,
        Err(why) => {
            eprintln!(
                "{}: couldn't open {} cause {}",
                "ERROR".bright_red(),
                filename.italic().bold(),
                why
            );
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        data.push(line);
    }
    data
}

pub fn read_file_in_path(path: String, filename: String) -> Vec<String> {
    read_file(format!("{}{}", path, filename))
}

pub fn write_file(filename: String, data: Vec<String>) {
    let mut file = match File::create(filename.clone()) {
        Ok(f) => f,
        Err(why) => {
            eprintln!(
                "{}: couldn't create file {} cause {}",
                "ERROR".bright_red(),
                filename.italic().bold(),
                why
            );
            std::process::exit(1);
        }
    };
    let mut val = Vec::new();
    for line in data {
        for byte in line.as_bytes() {
            val.push(*byte);
        }
        val.push(b'\n');
    }
    if let Err(why) = file.write_all(&val) {
        eprintln!(
            "{}: couldn't create file {} cause {}",
            "ERROR".bright_red(),
            filename.italic().bold(),
            why
        );
        std::process::exit(1);
    }
}

pub fn write_file_bin(filename: String, data: Vec<u8>) {
    let mut file = match File::create(filename.clone()) {
        Ok(f) => f,
        Err(why) => {
            eprintln!(
                "{}: couldn't create file {} cause {}",
                "ERROR".bright_red(),
                filename.italic().bold(),
                why
            );
            std::process::exit(1);
        }
    };
    if let Err(why) = file.write_all(&data) {
        eprintln!(
            "{}: couldn't create file {} cause {}",
            "ERROR".bright_red(),
            filename.italic().bold(),
            why
        );
        std::process::exit(1);
    }
}

pub fn is_such_file(filename: String) -> bool {
    std::path::Path::new(filename.as_str()).exists()
}


pub fn is_such_file_in_path(filename: String, path: String) -> bool {
    is_such_file(format!("{}{}", filename, path))
}
