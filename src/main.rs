use std::io;

pub mod lexer;
pub mod location;
pub mod token;

use crate::token::TokenTy;

fn main() {
    let mut lexer = lexer::build_lexer(String::from("<stdin>"), io::stdin());
    lexer.dump_chars();
}
