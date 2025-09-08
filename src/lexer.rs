use std::io::Error;
use std::io::Read;

use crate::chars;

use crate::location;
use crate::location::Location;

use crate::token;
use crate::token::Token;
use crate::token::TokenTy;

pub struct Lexer<Input: Read> {
    file: String,
    input: Input,
    current: Option<char>,
    location: Location,
    token: Option<Token>,
    error: Option<Error>,
}

pub fn build_lexer<Input: Read>(file: String, input: Input) -> Lexer<Input> {
    Lexer {
        file: file.clone(),
        input: input,
        current: None,
        location: location::build_location(file, 0, 0, 0, 0),
        token: None,
        error: None,
    }
}

impl<Input: Read> Lexer<Input> {
    fn reset_char(&mut self) {
        self.current = None;
    }

    fn next_char(&mut self) -> Option<char> {
        if self.current.is_some() {
            return self.current;
        }

        self.location.start_to_end();

        let mut buf = [0];
        match self.input.read(&mut buf) {
            Ok(0) => None,
            Ok(1) => {
                let c = buf[0] as char;
                match c {
                    '\n' => self.location.step_line(),
                    _ => self.location.step_column(),
                }
                self.current = Some(c);
                self.current
            }
            Ok(2..) => {
                self.error = Some(Error::other("Read an unexpected number of characters"));
                None
            }
            Err(err) => {
                self.error = Some(err);
                None
            }
        }
    }

    fn process_identifier(&mut self) {
        let mut location = self.location.clone();
        let mut id = String::from("");
        while let Some(c) = self.next_char()
            && chars::is_id(c)
        {
            id += &c.to_string();
            self.reset_char();
        }
        if self.error.is_some() {
            return;
        }
        let ty = match id.as_str() {
            "creg" => TokenTy::CReg,
            "qreg" => TokenTy::QReg,
            "measure" => TokenTy::Measure,
            "reset" => TokenTy::Reset,
            "pi" => TokenTy::Pi,
            "sin" => TokenTy::Sin,
            "cos" => TokenTy::Cos,
            "tan" => TokenTy::Tan,
            "exp" => TokenTy::Exp,
            "ln" => TokenTy::Ln,
            "sqrt" => TokenTy::Sqrt,
            _ => TokenTy::Id(id),
        };
        location.end_to_next(&self.location);
        self.token = Some(token::build_token(ty, location));
    }

    fn process_reserved_identifier(&mut self) {
        let mut location = self.location.clone();
        let mut id = String::from("");
        while let Some(c) = self.next_char()
            && chars::is_id(c)
        {
            id += &c.to_string();
            self.reset_char();
        }
        if self.error.is_some() {
            return;
        }
        if let Some(ty) = match id.as_str() {
            "OPENQASM" => Some(TokenTy::OpenQASM),
            "U" => Some(TokenTy::UGate),
            "CX" => Some(TokenTy::CXGate),
            _ => {
                self.error = Some(Error::other(format!("Invalid identifier '{}'", id)));
                None
            }
        } {
            location.end_to_next(&self.location);
            self.token = Some(token::build_token(ty, location));
        }
    }

    fn process(&mut self) {
        while let Some(c) = self.next_char()
            && chars::is_space(c)
        {
            self.reset_char();
        }
        if self.error.is_some() {
            return;
        }
        if let Some(c) = self.next_char() {
            if chars::is_lower(c) {
                self.process_identifier();
            } else if chars::is_upper(c) {
                self.process_reserved_identifier();
            } else {
                self.error = Some(Error::other(format!("Invalid character '{}'", c)));
            }
        } else {
            self.token = Some(token::build_token(TokenTy::Eof, self.location.clone()));
        }
    }

    pub fn peek(&mut self) -> &Option<Token> {
        if self.token.is_some() {
            return &self.token;
        }
        self.process();
        &self.token
    }

    pub fn drop(&mut self) {
        self.token = None;
    }

    pub fn dump_chars(&mut self) {
        while let Some(c) = self.next_char() {
            println!("{}@{}", c.escape_debug(), self.location.to_string());
            self.reset_char();
        }
        println!("eof@{}", self.location.to_string());
    }
}
