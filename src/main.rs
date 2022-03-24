use clap::{load_yaml, App};
use colored::*;

mod dictionary;
mod file;
use file::{read_file, write_file_bin};
mod compiler;
use compiler::Compiler;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("build") {
        let input_fname = matches.value_of("input").unwrap().to_string();
        let output_fname = matches.value_of("output").unwrap().to_string();
        let verbosity = matches.is_present("verbose");

        if verbosity { println!("{}: Starting compiling", "INFO".cyan()) }

        let mut compiler = Compiler::new(read_file(input_fname));
        compiler.compile(true);
        if verbosity { println!("{}: Finishing compiling", "INFO".cyan()) }
        write_file_bin(output_fname, compiler.get_binary());
        if verbosity { bin_dump(compiler.get_binary()) }
    } else {
        eprintln!("{}: wasn't provided {}\nFor help run `{}`", "ERROR".bright_red(), "SUBCOMMAND".cyan(), "zas --help".green());
        std::process::exit(1);
    }
}


fn bin_dump(data: Vec<u8>) {
    print!("{}: Binary output\n\t", "INFO".cyan());
    let mut line = 0;
    for byte in data {
        print!("{:>0w$X} ", byte, w=2);
        line += 1;
        if line == 16 {
            print!("\n\t");
            line = 0
        }
    }
}