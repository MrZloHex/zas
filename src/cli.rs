use clap::{load_yaml, App};

#[derive(Copy, Clone)]
pub enum CmdType {
    BUILD,
    DISASSEMBLE,
    NONE,
}

#[allow(non_camel_case_types)]
pub enum CliSet_Error {
    NO_SUCH_CMD,
    NO_INPUT_FILE,
    NO_OUTPUT_FILE,
}

#[allow(non_camel_case_types)]
pub type CliSet_Result = Result<CliSet, CliSet_Error>;

pub struct CliSet {
    type_of_proc:    CmdType,
    input_filename:  String,
    output_filename: String,
    base_path:       Option<String>,
    include_path:    Option<String>,
    verbosity:       bool,
}

impl CliSet {
    pub fn new() -> Self {
        Self {
            type_of_proc:    CmdType::NONE,
            input_filename:  String::new(),
            output_filename: String::new(),
            base_path:       None,
            include_path:    None,
            verbosity:       false,
        }
    }

    pub fn get_cli_settings() -> CliSet_Result {
        use CliSet_Error::*;

        let mut cliset = CliSet::new();

        // catch all setting from stack
        let yaml = load_yaml!("cli.yaml");
        let matches = App::from(yaml).get_matches();

        // take all argumetns of BUILD cmd
        if let Some(matches) = matches.subcommand_matches("build") {
            cliset.type_of_proc = CmdType::BUILD;

            cliset.input_filename = if let Some(in_fn) = matches.value_of("input") {
                in_fn.to_string()
            } else {
                return Err(NO_INPUT_FILE);
            };

            cliset.output_filename = if let Some(out_fn) = matches.value_of("output") {
                out_fn.to_string()
            } else {
                return Err(NO_OUTPUT_FILE);
            };

            // get base path relative input file, for includes in project
            cliset.base_path = if let Some(base) = cliset.input_filename.rsplit_once('/') {
                let mut base = base.0.to_string();
                base.push('/');
                Some(base)
            } else {
                None
            };

            cliset.verbosity = matches.is_present("verbose");

            // get include path and convert to String type
            cliset.include_path = matches.value_of("include").map(|i| i.to_string());
        } else if let Some(_matches) = matches.subcommand_matches("disassemble") {
            cliset.type_of_proc = CmdType::DISASSEMBLE;
            todo!("IN PROGRESS");
        } else {
            return Err(NO_SUCH_CMD);
        };

        Ok(cliset)
    }

    pub fn get_cmd_type(&self) -> CmdType {
        self.type_of_proc
    }

    pub fn get_input_filename(&self) -> String {
        self.input_filename.clone()
    }

    pub fn get_output_filename(&self) -> String {
        self.output_filename.clone()
    }

    pub fn get_base_path(&self) -> Option<String> {
        self.base_path.clone()
    }

    pub fn get_include_path(&self) -> Option<String> {
        self.include_path.clone()
    }

    pub fn get_verbosity(&self) -> bool {
        self.verbosity
    }
}
