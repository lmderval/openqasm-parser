use std::io::Error;
use std::io::Read;

use crate::location;
use crate::location::Location;

use crate::token::Token;

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

    pub fn dump_chars(&mut self) {
        while let Some(c) = self.next_char() {
            println!("{}@{}", c.escape_debug(), self.location.to_string());
            self.reset_char();
        }
        println!("eof@{}", self.location.to_string());
    }
}
