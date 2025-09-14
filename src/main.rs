pub mod ast;
pub mod bind;
pub mod parse;
pub mod utils;

use crate::ast::pretty_printer::PrettyPrinter;

use crate::bind::binder::Binder;

use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;

fn main() {
    let lexer = Lexer::new(String::from("<stdin>"), std::io::stdin());
    let mut parser = Parser::new(lexer);
    let mut binder = Binder::new();

    if let Some(program) = &mut parser.parse_input() {
        binder.bind(program);

        PrettyPrinter.pretty_print(program);
    }

    parser
        .get_error()
        .iter()
        .for_each(|it| println!("{}", it.to_string()));
    binder
        .get_error()
        .iter()
        .for_each(|it| println!("{}", it.to_string()));
}
