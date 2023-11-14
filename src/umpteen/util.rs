use std::fmt::Display;

use crate::{error::Line, repr::token::Token};

pub fn report_line<I: Display>(error: I, line: Line) {
    eprintln!("ERR: {} on line {}", error, line);
}

pub fn report_at<I: Display>(error: I, tk: Token) {
    eprintln!("ERR: {} at `{}` on line {}", error, tk.lexeme, tk.line)
}

#[macro_export]
macro_rules! boxed {
    ($e:expr) => {
        Box::new($e)
    };
}
