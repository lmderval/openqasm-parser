pub mod ast;
pub mod bind;
pub mod parse;
pub mod utils;

use crate::ast::pretty_printer::PrettyPrinter;

use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;

fn main() {
    let lexer = Lexer::new(String::from("<stdin>"), std::io::stdin());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_input();
    program.iter().for_each(|it| PrettyPrinter.pretty_print(it));
    parser
        .get_error()
        .iter()
        .for_each(|it| println!("{}", it.to_string()));
}
