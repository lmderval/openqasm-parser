pub mod location;
pub mod token;

use crate::token::TokenTy;

fn main() {
    {
        let loc = location::build_location(String::from("basic.qasm"), 0, 0, 0, 8);
        let token = token::build_token(TokenTy::OpenQASM, loc);
        println!("{}", token.to_string());
    }
    {
        let loc = location::build_location(String::from("basic.qasm"), 0, 10, 0, 13);
        let token = token::build_token(TokenTy::Real(2.0), loc);
        println!("{}", token.to_string());
    }
    {
        let loc = location::build_location(String::from("basic.qasm"), 0, 13, 0, 14);
        let token = token::build_token(TokenTy::Semi, loc);
        println!("{}", token.to_string());
    }
    {
        let loc = location::build_location(String::from("basic.qasm"), 1, 0, 1, 0);
        let token = token::build_token(TokenTy::Eof, loc);
        println!("{}", token.to_string());
    }
}
