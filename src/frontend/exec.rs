#[path="prelude.rs"]
mod prelude;
pub mod exec {
    use std::process::exit;
    use std::io;
    use std::io::{stdout, Write};
    use crate::errors::err::*;
    use crate::lexer::lex::*;
    use crate::exec::prelude::prelude::*;
    pub fn execute(tokens: Vec<Token>, details: bool) {
        let stdin = io::stdin();
        let mut data_ptr_index: isize = 0;
        let mut code_ptr_index: usize = 0;
        let mut data_ptr_dir: isize = 1;
        let mut moving: isize;
        let mut tape = Tape::new();
        let mut acc = Acc::new();
        let mut creating_number = false;

        let mut last_executed: &Token = &Token::Nothing;
        while code_ptr_index < tokens.len() {
            moving = 1;
            let current = tokens.get(code_ptr_index).unwrap();
            creating_number = match *current {
                Token::CreatingNumber | Token::Digit(_) => creating_number,
                _ => false
            };

            match *current {
                Token::NoOp | Token::DestinationIfTrue | Token::Nothing => {}
                Token::Stop => {
                    details_success(details, data_ptr_index, code_ptr_index, data_ptr_dir, &mut tape, &mut acc, last_executed, current);
                    exit(0);
                }
                Token::CreatingNumber => {
                    if creating_number {
                        Error::SyntaxError.throw("already creating number", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `#` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    creating_number = true;
                    moving = 0;
                }
                Token::Digit(n) => {
                    if !creating_number {
                        Error::SyntaxError.throw(&format!("execution of `{n}` went wrong"), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let success = acc.append(n);
                    if success.is_err() { print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true); }
                    moving = 0;
                }
                Token::Unload => {
                    if acc.is_empty() {
                        Error::AccumulatorError.throw("execution of `U` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    tape.set(data_ptr_index, acc.clear().unwrap());
                }
                Token::Distribute => {
                    if acc.is_empty() {
                        Error::AccumulatorError.throw("execution of `u` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    tape.set(data_ptr_index, acc.get_value().unwrap())
                }
                Token::Recall => {
                    if tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `R` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(tape.clear(data_ptr_index).unwrap())
                }
                Token::Copy => {
                    if tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `r` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(tape.get(data_ptr_index).unwrap())
                }
                Token::ClearAcc => {
                    acc.clear();
                }
                Token::ClearCurrCell => {
                    tape.clear(data_ptr_index);
                }
                Token::ZeroOrEmpty => {
                    if tape.cell_is_empty(data_ptr_index) || tape.get(data_ptr_index).unwrap() == 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `?` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                }
                Token::NotZeroOrEmpty => {
                    if tape.cell_is_full(data_ptr_index) && tape.get(data_ptr_index).unwrap() != 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `!` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                }
                Token::TgtZeroOrEmpty => {
                    if acc.is_empty() {
                        Error::AccumulatorError.throw("execution of `T` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let pos = tape.get(acc.get_value().unwrap());
                    if pos.is_none() || pos.unwrap() == 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `T` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                    acc.clear();
                }
                Token::TgtNotZeroOrEmpty => {
                    if acc.is_empty() {
                        Error::AccumulatorError.throw("execution of `t` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let pos = tape.get(acc.get_value().unwrap());
                    if pos.is_some() && pos.unwrap() != 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `t` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                    acc.clear();
                }
                Token::AccZeroOrEmpty => {
                    if acc.is_empty() || acc.get_value().unwrap() == 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `A` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                }
                Token::AccNotZeroOrEmpty => {
                    if !acc.is_empty() && acc.get_value().unwrap() != 0 {
                        let result = scope_check(code_ptr_index, &tokens);
                        if result.is_none() {
                            Error::SyntaxError.throw("conditional `a` does not have a corresponding `@`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        } else {
                            code_ptr_index = result.unwrap();
                        }
                    }
                }
                Token::JumpCellsC => {
                    if acc.is_empty() {
                        Error::OpError.throw("execution of `J` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }

                    let will_overflow = if acc.get_value().unwrap() < 0 {
                        code_ptr_index.overflowing_sub((-acc.clear().unwrap()) as usize)
                    } else {
                        code_ptr_index.overflowing_add(acc.clear().unwrap() as usize)
                    };

                    if will_overflow.1 || will_overflow.0 >= tokens.len() {
                        Error::OverflowError.throw("code pointer went out of bounds when executing `J`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    code_ptr_index = will_overflow.0
                }
                Token::JumpToCellC => {
                    if acc.is_empty() {
                        Error::OpError.throw("execution of `j` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.get_value().unwrap() >= tokens.len() as isize || acc.get_value().unwrap() < 0 {
                        Error::OverflowError.throw("code pointer went out of bounds when executing `j`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    code_ptr_index = acc.clear().unwrap() as usize;
                }
                Token::JumpCellsD => {
                    if acc.is_empty() {
                        Error::OpError.throw("execution of `K` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let will_overflow = data_ptr_index.overflowing_add(acc.clear().unwrap());
                    if will_overflow.1 {
                        Error::OverflowError.throw("data pointer went out of bounds when executing `K`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    data_ptr_index = will_overflow.0;
                    moving = 0
                }
                Token::JumpToCellD => {
                    if acc.is_empty() {
                        Error::OpError.throw("execution of `k` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    data_ptr_index = acc.clear().unwrap();
                    moving = 0
                }
                Token::MoveDUntilEmpty => {
                    while tape.cell_is_full(data_ptr_index) {
                        let will_overflow = data_ptr_index.overflowing_add(data_ptr_dir);
                        if will_overflow.1 {
                            Error::OverflowError.throw("data pointer went out of bounds when executing `M`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        }
                        data_ptr_index = will_overflow.0
                    }
                    moving = 0
                }
                Token::MoveDUntilFull => {
                    while tape.cell_is_empty(data_ptr_index) {
                        let will_overflow = data_ptr_index.overflowing_add(data_ptr_dir);
                        if will_overflow.1 {
                            Error::OverflowError.throw("data pointer went out of bounds when executing `m`", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        }
                        data_ptr_index = will_overflow.0
                    }
                    moving = 0
                }
                Token::FlipD(_) => {
                    data_ptr_dir *= -1
                }
                Token::Add => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `+` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `+` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let result = tape.get(data_ptr_index - 1).unwrap().overflowing_add(tape.get(data_ptr_index).unwrap());
                    if result.1 {
                        Error::OverflowError.throw("command `+` caused overflow", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(result.0)
                }
                Token::Neg => {
                    if acc.is_empty() {
                        Error::AccumulatorError.throw("execution of `-` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(-acc.get_value().unwrap())
                }
                Token::Mul => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `*` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `*` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let result = tape.get(data_ptr_index - 1).unwrap().overflowing_mul(tape.get(data_ptr_index).unwrap());
                    if result.1 {
                        Error::OverflowError.throw("command `*` caused overflow", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(result.0)
                }
                Token::Div => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `/` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if tape.get(data_ptr_index).unwrap() == 0 {
                        Error::OpError.throw("division by zero caused by `/`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `/` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let result = tape.get(data_ptr_index - 1).unwrap().overflowing_div_euclid(tape.get(data_ptr_index).unwrap());
                    if result.1 {
                        Error::OverflowError.throw("command `/` caused overflow", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(result.0)
                }
                Token::Mod => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `%` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if tape.get(data_ptr_index).unwrap() == 0 {
                        Error::OpError.throw("division by zero caused by `%`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `%` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    let result = tape.get(data_ptr_index - 1).unwrap().overflowing_rem(tape.get(data_ptr_index).unwrap());
                    if result.1 {
                        Error::OverflowError.throw("command `%` caused overflow", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(result.0)
                }
                Token::Eq => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `=` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `=` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() == tape.get(data_ptr_index).unwrap()))
                }
                Token::NotEq(n) => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw(&format!("execution of `{}` went wrong", if n {'N'} else {'n'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw(&format!("execution of `{}` went wrong", if n {'N'} else {'n'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() != tape.get(data_ptr_index).unwrap()))
                }
                Token::Gt => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `>` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `>` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() > tape.get(data_ptr_index).unwrap()))
                }
                Token::GE(n) => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw(&format!("execution of `{}` went wrong", if n {'G'} else {'g'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw(&format!("execution of `{}` went wrong", if n {'G'} else {'g'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() >= tape.get(data_ptr_index).unwrap()))
                }
                Token::Lt => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `<` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `<` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() < tape.get(data_ptr_index).unwrap()))
                }
                Token::LE(n) => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw(&format!("execution of `{}` went wrong", if n {'L'} else {'l'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw(&format!("execution of `{}` went wrong", if n {'L'} else {'l'}), false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(isize::from(tape.get(data_ptr_index - 1).unwrap() <= tape.get(data_ptr_index).unwrap()))
                }
                Token::BitAnd => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `&` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `&` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(tape.get(data_ptr_index - 1).unwrap() & tape.get(data_ptr_index).unwrap())
                }
                Token::BitOr => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `|` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `|` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(tape.get(data_ptr_index - 1).unwrap() | tape.get(data_ptr_index).unwrap())
                }
                Token::BitNot => {
                    if tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `~` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(!tape.get(data_ptr_index).unwrap())
                }
                Token::BitXor => {
                    if tape.left_of(data_ptr_index).is_err() || tape.left_of(data_ptr_index).unwrap().is_none() || tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("execution of `^` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if acc.is_not_empty() {
                        Error::AccumulatorError.throw("execution of `^` went wrong", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    acc.set_value(tape.get(data_ptr_index - 1).unwrap() ^ tape.get(data_ptr_index).unwrap())
                }
                Token::OutputInt => {
                    if tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("something went wrong while executing `i`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    print!("{}", tape.get(data_ptr_index).unwrap());
                    stdout().flush().unwrap();
                }
                Token::OutputChar => {
                    // throw if the current cell is empty
                    if tape.cell_is_empty(data_ptr_index) {
                        Error::OpError.throw("something went wrong while executing `s`", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }

                    // check if the current cell's value can be represented as a UTF-8 character
                    if u32::try_from(tape.get(data_ptr_index).unwrap()).is_err() {
                        Error::OpError.throw("the current cell's value cannot be represented as a valid UTF-8 character", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }
                    if char::from_u32(tape.get(data_ptr_index).unwrap() as u32).is_none() {
                        Error::OpError.throw("the current cell's value cannot be represented as a valid UTF-8 character", false);
                        print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                    }

                    // print the character out
                    let chr = char::from_u32(tape.get(data_ptr_index).unwrap() as u32).unwrap();
                    print!("{chr}");
                    stdout().flush().unwrap();
                }
                Token::InputInt => {
                    // get input from stdin
                    let mut input = "".to_string();

                    // check 1: does the input have any invalid characters?
                    let input_valid_1 = stdin.read_line(&mut input);
                    if let Err(_) = input_valid_1 {
                        Error::InputError.throw("invalid input", true);
                    }
                    input.pop();

                    // check 2: can the input be represented as a sized integer?
                    let input_valid_2 = input.parse::<isize>();
                    if let Err(_) = input_valid_2 {
                        Error::InputError.throw("invalid input", true);
                    }

                    // set the accumulator's value to the input
                    acc.set_value(input_valid_2.unwrap());
                    moving = 0
                }
                Token::InputStr => {
                    // get input from stdin
                    let mut input = "".to_string();

                    // check 1: does the input have any invalid characters?
                    let input_valid_1 = stdin.read_line(&mut input);
                    if let Err(_) = input_valid_1 {
                        Error::InputError.throw("invalid input", true);
                    }
                    input.pop();

                    // place characters in input
                    let iter = input.chars();
                    let mut curr = data_ptr_index;
                    for chr in iter {
                        tape.set(curr, chr as isize);
                        if curr.overflowing_add(1).1 {
                            Error::InputError.throw("input too long; went beyond tape boundaries", false);
                            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
                        }
                        curr += 1;
                    }
                }
            }
            let out_of_bounds_c = code_ptr_index.overflowing_add(1).1;
            let out_of_bounds_d = data_ptr_index.overflowing_add(1 * moving * data_ptr_dir).1;
            if out_of_bounds_c || code_ptr_index + 1 >= tokens.len() {
                Error::OutOfBoundsError.throw("code pointer went out of bounds", false);
                print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
            }
            if out_of_bounds_d {
                Error::OutOfBoundsError.throw("data pointer went out of bounds", false);
                print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, true);
            }
            last_executed = current;
            code_ptr_index += 1;
            data_ptr_index += moving * data_ptr_dir
        }
    }

    fn details_success(details: bool, data_ptr_index: isize, code_ptr_index: usize, data_ptr_dir: isize, tape: &mut Tape, acc: &mut Acc, last_executed: &Token, current: &Token) {
        if details {
            println!();
            print_details(&acc, &tape, data_ptr_index, data_ptr_dir, code_ptr_index, current, last_executed, false);
            exit(0)
        }
    }

    fn scope_check(code_ptr_index: usize, tokens: &Vec<Token>) -> Option<usize> {
        let mut destination = code_ptr_index;
        let mut scope = 0;
        while destination < tokens.len() {
            scope += match *tokens.iter().nth(destination)? {
                Token::ZeroOrEmpty | Token::NotZeroOrEmpty | Token::TgtZeroOrEmpty | Token::TgtNotZeroOrEmpty | Token::AccZeroOrEmpty | Token::AccNotZeroOrEmpty => {
                    1
                }
                Token::DestinationIfTrue => {
                    -1
                }
                _ => 0
            };
            if scope == 0 {
                return Some(destination)
            }
            destination += 1;
        }
        None
    }

    pub(crate) fn print_details(acc: &Acc, tape: &Tape, data_ptr_index: isize, data_ptr_dir: isize, code_ptr_index: usize, current_command: &Token, last_executed: &Token, terminate: bool) {
        eprintln!("\x1b[33;1m[Details]\x1b[0m
\x1b[1m*\x1b[0m {}
\x1b[1m*\x1b[0m code pointer index: {code_ptr_index}
\x1b[1m*\x1b[0m data pointer index: {data_ptr_index}
\x1b[1m*\x1b[0m data pointer direction: {}
\x1b[1m*\x1b[0m current command: {}
\x1b[1m*\x1b[0m previous command executed: {}
\x1b[1m*\x1b[0m current cell value: {}
\x1b[1m*\x1b[0m left cell value: {}",
                  acc.get_details(),
                  if data_ptr_dir == 1 {"positive"} else {"negative"},
                  token_to_symbol(current_command),
                  token_to_symbol(last_executed),
                  if let Some(_) = tape.get(data_ptr_index) {
                      tape.get(data_ptr_index).unwrap().to_string()
                  } else {
                      "<none>".to_string()
                  },
                  if let Ok(n) = tape.left_of(data_ptr_index) {
                      if let Some(_) = n {
                          tape.left_of(data_ptr_index).unwrap().unwrap().to_string()
                      } else {
                          "<none>".to_string()
                      }
                  } else {
                      "<cell does not exist>".to_string()
                  }
        );
        if terminate { exit(1) }
    }
}