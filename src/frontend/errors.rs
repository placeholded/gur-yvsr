#[path="prelude.rs"]
mod prelude;
pub mod err {
    use std::process::exit;

    /// Error that can be thrown when something goes wrong.
    #[derive(Debug)]
    pub enum Error {
        FileError,
        UnknownSymbolError,
        OpError,
        AccumulatorError,
        SyntaxError,
        InputError,
        OutOfBoundsError,
        OverflowError,
    }

    impl Error {
        pub(crate) fn throw(&self, msg: &str, terminate: bool) {
            eprintln!("\x1b[31;1m{:?}:\x1b[0m {msg}", self);
            if terminate { exit(1) }
        }
    }
}