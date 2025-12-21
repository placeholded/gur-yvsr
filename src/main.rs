use clap::Parser;
#[path="frontend/lexer.rs"]
mod lexer;
use lexer::lex::*;

#[path="frontend/errors.rs"]
mod errors;
use errors::err::*;

#[path="frontend/exec.rs"]
mod exec;
use exec::exec::*;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::Read;

/// Programming language inspired by brainfuck and Emmental.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to run.
    #[arg(required = true)]
    input: Vec<PathBuf>,
    /// Print details after the program ends.
    /// The details will contain the accumulator's value,
    /// the code pointer's index (starting at 0), the data pointer's index (starting at 0),
    /// the direction of the data pointer (positive or negative), the current command, the previous command executed,
    /// the value of the current cell, and the value of the cell 1 unit in the negative direction of the current cell.
    #[arg(short, long)]
    details: bool
}

fn main() {
    let command_args = Cli::parse();
    let path = command_args.input.iter().nth(0);
    if path.is_none() {
        Error::FileError.throw("invalid file path", true)
    }
    if !path.unwrap().is_file() {
        Error::FileError.throw(&format!("the file {:?} does not exist or is invalid", path.unwrap()), true)
    }

    let extension_check_0 = Path::new(path.unwrap()).extension();
    if extension_check_0.is_none() {
        Error::FileError.throw("invalid file extension", true)
    }
    let extension_check_1 = extension_check_0.unwrap().to_str().unwrap();
    if extension_check_1.ne("gur") {
        Error::FileError.throw("file extension must be .gur", true)
    }

    let open_success = File::open(path.unwrap());
    if open_success.is_err() {
        Error::FileError.throw(&format!("the file {:?} does not exist", path.unwrap()), true)
    }

    let mut prog = open_success.unwrap();
    let mut buf: String = "".to_string();
    let success = prog.read_to_string(&mut buf);
    if success.is_err() {
        Error::FileError.throw("file cannot be read because it contains non-UTF-8 characters", true)
    }
    execute(tokenize(&mut buf), command_args.details)
}
