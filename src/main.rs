use colored::*;

mod file;
use file::{read_file, write_file_bin};

mod compiler;
use compiler::build;

mod cli;
use cli::CliSet;

fn main() {
    let cli_settings = CliSet::get_cli_settings();
    let cli_settings = match cli_settings {
        Ok(cs) => cs,
        Err(e) => {
            use cli::CliSet_Error::*;
            match e {
                NO_INPUT_FILE => unreachable!(),
                NO_OUTPUT_FILE => unreachable!(),
                NO_SUCH_CMD => {
                    eprintln!(
                        "{}: wasn't provided {}\nFor help run `{}`",
                        "ERROR".bright_red(),
                        "SUBCOMMAND".cyan(),
                        "zas --help".green()
                    );
                    std::process::exit(1);
                }
            }
        }
    };

    use cli::CmdType::*;
    match cli_settings.get_cmd_type() {
        BUILD => build(
            cli_settings.get_input_filename(),
            cli_settings.get_output_filename(),
            cli_settings.get_base_path(),
            cli_settings.get_include_path()
        ),
        DISASSEMBLE => unreachable!(),
        NONE        => unreachable!(),
    }

    // if verbosity { bin_dump(compiler.get_binary()) }
}

fn _bin_dump(data: Vec<u8>) {
    print!("{}: Binary output\n\t", "INFO".cyan());
    let mut line = 0;
    for byte in data {
        print!("{:>0w$X} ", byte, w = 2);
        line += 1;
        if line == 16 {
            print!("\n\t");
            line = 0
        }
    }
    println!()
}
