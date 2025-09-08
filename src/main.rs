use std::io;

pub mod ast;
pub mod chars;
pub mod lexer;
pub mod location;
pub mod token;

fn main() {
    let mut lexer = lexer::build_lexer(String::from("<stdin>"), io::stdin());
    while let Some(token) = lexer.peek() {
        println!("{}", token.to_string());
        match token.get_ty() {
            token::TokenTy::Eof => break,
            _ => {}
        }
        lexer.drop();
    }
}
