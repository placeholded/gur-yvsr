pub mod lex {
    use regex::Regex;
    use crate::errors::err::Error;

    pub enum Token {
        NoOp,              // _
        Stop,              // .
        CreatingNumber,    // #
        Digit(isize),      // 0 1 2 3 4 5 6 7 8 9
        Unload,            // U
        Distribute,        // u
        Recall,            // R
        Copy,              // r
        ClearAcc,          // C
        ClearCurrCell,     // c
        ZeroOrEmpty,       // ?
        NotZeroOrEmpty,    // !
        TgtZeroOrEmpty,    // T
        TgtNotZeroOrEmpty, // t
        AccZeroOrEmpty,    // A
        AccNotZeroOrEmpty, // a
        DestinationIfTrue, // @
        JumpCellsC,        // J
        JumpToCellC,       // j
        JumpCellsD,        // K
        JumpToCellD,       // k
        FlipD(bool),       // F or f
        MoveDUntilEmpty,   // M
        MoveDUntilFull,    // m
        Add,               // +
        Neg,               // -
        Mul,               // *
        Div,               // /
        Mod,               // %
        Eq,                // =
        NotEq(bool),       // N or n
        Gt,                // >
        GE(bool),          // G or g
        Lt,                // <
        LE(bool),          // L or l
        BitAnd,            // &
        BitOr,             // |
        BitNot,            // ~
        BitXor,            // ^
        OutputInt,         // i
        OutputChar,        // s
        InputInt,          // I
        InputStr,          // S
    }
    pub fn tokenize(txt: &mut str) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        let ignore = Regex::new(r"`(.|\s)*?`|\s*").unwrap();
        let prog = &*ignore.replace_all(txt.trim(), "");

        let chars = prog.chars();
        let mut index = 0;
        for c in chars {
            match c {
                '_' => tokens.push(Token::NoOp),
                '.' => tokens.push(Token::Stop),
                '#' => tokens.push(Token::CreatingNumber),
                '0'..='9' => tokens.push(Token::Digit(c.to_digit(10).unwrap() as isize)),
                'U' => tokens.push(Token::Unload),
                'u' => tokens.push(Token::Distribute),
                'R' => tokens.push(Token::Recall),
                'r' => tokens.push(Token::Copy),
                'C' => tokens.push(Token::ClearAcc),
                'c' => tokens.push(Token::ClearCurrCell),
                '?' => tokens.push(Token::ZeroOrEmpty),
                '!' => tokens.push(Token::NotZeroOrEmpty),
                'T' => tokens.push(Token::TgtZeroOrEmpty),
                't' => tokens.push(Token::TgtNotZeroOrEmpty),
                'A' => tokens.push(Token::AccZeroOrEmpty),
                'a' => tokens.push(Token::AccNotZeroOrEmpty),
                '@' => tokens.push(Token::DestinationIfTrue),
                'J' => tokens.push(Token::JumpCellsC),
                'j' => tokens.push(Token::JumpToCellC),
                'K' => tokens.push(Token::JumpCellsD),
                'k' => tokens.push(Token::JumpToCellD),
                'F' => tokens.push(Token::FlipD(true)),
                'f' => tokens.push(Token::FlipD(false)),
                'M' => tokens.push(Token::MoveDUntilEmpty),
                'm' => tokens.push(Token::MoveDUntilFull),
                '+' => tokens.push(Token::Add),
                '-' => tokens.push(Token::Neg),
                '*' => tokens.push(Token::Mul),
                '/' => tokens.push(Token::Div),
                '%' => tokens.push(Token::Mod),
                '=' => tokens.push(Token::Eq),
                'N' | 'n' => tokens.push(Token::NotEq(c.is_uppercase())),
                '>' => tokens.push(Token::Gt),
                'G' | 'g' => tokens.push(Token::GE(c.is_uppercase())),
                '<' => tokens.push(Token::Lt),
                'L' | 'l' => tokens.push(Token::LE(c.is_uppercase())),
                '&' => tokens.push(Token::BitAnd),
                '|' => tokens.push(Token::BitOr),
                '~' => tokens.push(Token::BitNot),
                '^' => tokens.push(Token::BitXor),
                'i' => tokens.push(Token::OutputInt),
                's' => tokens.push(Token::OutputChar),
                'I' => tokens.push(Token::InputInt),
                'S' => tokens.push(Token::InputStr),
                _ => Error::UnknownSymbolError.throw(&format!("unrecognized symbol {c} found at index {index}"), true)
            }
            index += 1
        }
        tokens
    }
    pub fn token_to_symbol(token: Option<&Token>) -> &str {
        if let Some(c) = token {
            match c {
                Token::NoOp => "_",
                Token::Stop => ".",
                Token::CreatingNumber => "#",
                Token::Digit(n) => match n {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    5 => "5",
                    6 => "6",
                    7 => "7",
                    8 => "8",
                    9 => "9",
                    _ => ""
                },
                Token::Unload => "U",
                Token::Distribute => "u",
                Token::Recall => "R",
                Token::Copy => "r",
                Token::ClearAcc => "C",
                Token::ClearCurrCell => "c",
                Token::ZeroOrEmpty => "?",
                Token::NotZeroOrEmpty => "!",
                Token::TgtZeroOrEmpty => "T",
                Token::TgtNotZeroOrEmpty => "t",
                Token::AccZeroOrEmpty => "A",
                Token::AccNotZeroOrEmpty => "a",
                Token::DestinationIfTrue => "@",
                Token::JumpCellsC => "J",
                Token::JumpToCellC => "j",
                Token::JumpCellsD => "K",
                Token::JumpToCellD => "k",
                Token::FlipD(n) => if *n { "F" } else { "f" },
                Token::MoveDUntilEmpty => "M",
                Token::MoveDUntilFull => "m",
                Token::Add => "+",
                Token::Neg => "-",
                Token::Mul => "*",
                Token::Div => "/",
                Token::Mod => "%",
                Token::Eq => "=",
                Token::NotEq(n) => if *n { "N" } else { "n" },
                Token::Gt => ">",
                Token::GE(n) => if *n { "G" } else { "g" },
                Token::Lt => "<",
                Token::LE(n) => if *n { "L" } else { "l" },
                Token::BitAnd => "&",
                Token::BitOr => "|",
                Token::BitNot => "~",
                Token::BitXor => "^",
                Token::OutputInt => "i",
                Token::OutputChar => "s",
                Token::InputInt => "I",
                Token::InputStr => "S"
            }
        } else {
            "<none>"
        }
    }
}