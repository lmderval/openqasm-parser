pub mod ast;
pub mod bind;
pub mod parse;
pub mod utils;

use std::process;

use crate::ast::pretty_printer::PrettyPrinter;

use crate::bind::binder::Binder;

use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;

use crate::utils::error::{CompoundError, Error};

fn main() {
    let lexer = Lexer::new(String::from("<stdin>"), std::io::stdin());
    let mut parser = Parser::new(lexer);
    let mut binder = Binder::new();

    if let Some(program) = &mut parser.parse_input() {
        binder.bind(program);

        PrettyPrinter.pretty_print(program);
    }

    let mut error = CompoundError::new();
    error.consume(parser.get_error_mut());
    error.consume(binder.get_error_mut());

    if !error.empty() {
        eprintln!("{}", error.get_desc());
    }

    process::exit(error.get_exit_code());
}
