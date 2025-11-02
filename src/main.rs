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
use std::path::Path;
use std::io::Read;
use std::env::args_os;

fn main() {
    let mut command_args = args_os();
    if command_args.len() != 2 {
        Error::CommandLineArgsError.throw("must supply 2 command line arguments", true);
    }

    let path_0 = command_args.nth(1).unwrap();
    let path = path_0.to_str();
    if path.is_none() {
        Error::FileError.throw("invalid file path", true)
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
        Error::FileError.throw(&format!("the file {} does not exist", path.unwrap()), true)
    }

    let mut prog = open_success.unwrap();
    let mut buf: String = "".to_string();
    let success = prog.read_to_string(&mut buf);
    if success.is_err() {
        Error::FileError.throw("file cannot be read because it contains non-UTF-8 characters", true)
    }
    execute(tokenize(&mut buf))
}
