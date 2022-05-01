//! A module for parsing the command line arguments.

use std::{env, process};
use std::collections::HashSet;

use crate::exit_code;

/// A struct containing program execution options.
pub struct Options {
    do_benchmark: bool,
}

impl Options {
    /// default `false`
    /// (see [crate::benchmark])
    pub fn do_benchmark(&self) -> bool {
        self.do_benchmark
    }
}

impl Default for Options {
    /// Create a new [Self] with the default values.
    fn default() -> Self {
        Self { do_benchmark: false }
    }
}

/// Parses the command line arguments into an [Options].\
/// Exits the program with the exit code 1 if invalid argument(s) are provided.
/// (see [exit_code::INVALID_ARGUMENTS_ERROR])
pub fn parse_from_args() -> Options {
    let mut options = Options::default();
    let mut supplied_arguments = HashSet::new();

    for arg in env::args().skip(1) {
        let arg = arg.to_lowercase();

        if supplied_arguments.contains(&arg) {
            eprintln!("[!] duplicate argument `{}`", arg);
            process::exit(exit_code::INVALID_ARGUMENTS_ERROR);
        }

        match arg.as_str() {
            "--benchmark" => {
                options.do_benchmark = true;
            }
            _ => {
                eprintln!("[!] argument `{}` does not exist", arg);
                process::exit(exit_code::INVALID_ARGUMENTS_ERROR)
            }
        };
        supplied_arguments.insert(arg);
    }
    options
}