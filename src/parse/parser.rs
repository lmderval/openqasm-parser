use std::io::Error;
use std::io::Read;

use std::vec::Vec;

use crate::ast::{factory, node};

use crate::parse::lexer::Lexer;
use crate::parse::token::TokenTy;

pub struct Parser<Input: Read> {
    lexer: Lexer<Input>,
    error: Option<Error>,
}

macro_rules! is_token {
    ($token:expr; in [$($tys:pat),+]) => {
        match $token.get_ty() {
            $($tys)|+ => true,
            _ => false,
        }
    };
}

macro_rules! peek_token {
    ($parser:expr; in [$($tys:pat),+]) => {{
        let token = $parser.lexer.peek().as_ref()?;
        match token.get_ty() {
            $($tys)|+ => Some(token),
            _ => {
                $parser.error = Some(Error::other(format!(
                    "Unexpected token {}",
                    token.to_string()
                )));
                None
            }
        }
    }};
}

impl<Input: Read> Parser<Input> {
    pub fn new(lexer: Lexer<Input>) -> Parser<Input> {
        Parser {
            lexer: lexer,
            error: None,
        }
    }

    pub fn parse_input(&mut self) -> Option<Vec<node::Stmt>> {
        peek_token!(self; in [TokenTy::OpenQASM])?;
        self.lexer.drop();

        peek_token!(self; in [TokenTy::Real(2.0)])?;
        self.lexer.drop();

        peek_token!(self; in [TokenTy::Semi])?;
        self.lexer.drop();

        self.parse_program()
    }

    fn parse_program(&mut self) -> Option<Vec<node::Stmt>> {
        let mut program: Vec<node::Stmt> = Vec::new();
        loop {
            program.push(self.parse_statement()?);
            if is_token!(self.lexer.peek().as_ref()?; in [TokenTy::Eof]) {
                return Some(program);
            }
        }
    }

    fn parse_statement(&mut self) -> Option<node::Stmt> {
        let token = peek_token!(self; in [
            TokenTy::QReg,
            TokenTy::CReg,
            TokenTy::UGate,
            TokenTy::CXGate,
            TokenTy::Measure,
            TokenTy::Reset
        ])?;
        if is_token!(token; in [TokenTy::QReg, TokenTy::CReg]) {
            self.parse_decl()
        } else {
            self.parse_qop()
        }
    }

    fn parse_decl(&mut self) -> Option<node::Stmt> {
        let begin = peek_token!(self; in [TokenTy::QReg, TokenTy::CReg])?;
        let mut loc = begin.get_loc().clone();
        let ty = match begin.get_ty() {
            TokenTy::QReg => node::RegTy::QReg,
            TokenTy::CReg => node::RegTy::CReg,
            _ => return None,
        };
        self.lexer.drop();

        let name = match peek_token!(self; in [TokenTy::Id(_)])?.get_ty() {
            TokenTy::Id(it) => it.clone(),
            _ => return None,
        };
        self.lexer.drop();

        peek_token!(self; in [TokenTy::LBrack])?;
        self.lexer.drop();

        let size = match peek_token!(self; in [TokenTy::Integer(_)])?.get_ty() {
            TokenTy::Integer(it) => *it,
            _ => return None,
        };
        self.lexer.drop();

        peek_token!(self; in [TokenTy::RBrack])?;
        self.lexer.drop();

        let end = peek_token!(self; in [TokenTy::Semi])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_reg_dec(loc, name, ty, size))
    }

    fn parse_qop(&mut self) -> Option<node::Stmt> {
        let token = peek_token!(self; in [
            TokenTy::UGate,
            TokenTy::CXGate,
            TokenTy::Measure,
            TokenTy::Reset
        ])?;
        match token.get_ty() {
            TokenTy::UGate | TokenTy::CXGate => self.parse_uop(),
            TokenTy::Measure => self.parse_measure(),
            TokenTy::Reset => self.parse_reset(),
            _ => return None,
        }
    }

    fn parse_uop(&mut self) -> Option<node::Stmt> {
        match peek_token!(self; in [TokenTy::UGate, TokenTy::CXGate])?.get_ty() {
            TokenTy::UGate => self.parse_ugate(),
            TokenTy::CXGate => self.parse_cxgate(),
            _ => return None,
        }
    }

    fn parse_ugate(&mut self) -> Option<node::Stmt> {
        let begin = peek_token!(self; in [TokenTy::UGate])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        peek_token!(self; in [TokenTy::LPar])?;
        self.lexer.drop();

        let pars = self.parse_explist()?;

        peek_token!(self; in [TokenTy::RPar])?;
        self.lexer.drop();

        let arg = self.parse_argument()?;

        let end = peek_token!(self; in [TokenTy::Semi])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_gate_stmt(
            loc,
            String::from("U"),
            pars,
            vec![arg],
        ))
    }

    fn parse_cxgate(&mut self) -> Option<node::Stmt> {
        let begin = peek_token!(self; in [TokenTy::CXGate])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        let control = self.parse_argument()?;

        peek_token!(self; in [TokenTy::Comma])?;
        self.lexer.drop();

        let target = self.parse_argument()?;

        let end = peek_token!(self; in [TokenTy::Semi])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_gate_stmt(
            loc,
            String::from("CX"),
            vec![],
            vec![control, target],
        ))
    }

    fn parse_measure(&mut self) -> Option<node::Stmt> {
        let begin = peek_token!(self; in [TokenTy::Measure])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        let src = self.parse_argument()?;

        peek_token!(self; in [TokenTy::Arrow])?;
        self.lexer.drop();

        let dst = self.parse_argument()?;

        let end = peek_token!(self; in [TokenTy::Semi])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_measure_stmt(loc, src, dst))
    }

    fn parse_reset(&mut self) -> Option<node::Stmt> {
        let begin = peek_token!(self; in [TokenTy::Reset])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        let reg = self.parse_argument()?;

        let end = peek_token!(self; in [TokenTy::Semi])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_reset_stmt(loc, reg))
    }

    fn parse_argument(&mut self) -> Option<node::Reg> {
        let begin = peek_token!(self; in [TokenTy::Id(_)])?;
        let mut loc = begin.get_loc().clone();
        let name = match begin.get_ty() {
            TokenTy::Id(it) => it.clone(),
            _ => return None,
        };
        self.lexer.drop();

        if !is_token!(self.lexer.peek().as_ref()?; in [TokenTy::LBrack]) {
            return Some(factory::make_simple_reg(loc, name));
        }
        self.lexer.drop();

        let index = match peek_token!(self; in [TokenTy::Integer(_)])?.get_ty() {
            TokenTy::Integer(it) => *it,
            _ => return None,
        };
        self.lexer.drop();

        let end = peek_token!(self; in [TokenTy::RBrack])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_subscript_reg(loc, name, index))
    }

    fn parse_explist(&mut self) -> Option<Vec<node::Exp>> {
        let mut exps: Vec<node::Exp> = Vec::new();
        loop {
            exps.push(self.parse_exp()?);
            if !is_token!(self.lexer.peek().as_ref()?; in [TokenTy::Comma]) {
                return Some(exps);
            }
            self.lexer.drop();
        }
    }

    fn parse_simple_term_exp(&mut self) -> Option<node::Exp> {
        let token = peek_token!(self; in [
            TokenTy::Real(_),
            TokenTy::Integer(_),
            TokenTy::Pi
        ])?;
        let loc = token.get_loc().clone();
        let exp = match token.get_ty() {
            TokenTy::Real(value) => factory::make_real_exp(loc, *value),
            TokenTy::Integer(value) => factory::make_int_exp(loc, *value),
            TokenTy::Pi => factory::make_pi_exp(loc),
            _ => return None,
        };
        self.lexer.drop();

        Some(exp)
    }

    fn parse_delimited_term_exp(&mut self) -> Option<node::Exp> {
        let begin = peek_token!(self; in [TokenTy::LPar])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        let exp = self.parse_exp()?;

        let end = peek_token!(self; in [TokenTy::RPar])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        let res = match exp {
            node::Exp::IntExp { loc: _, value } => factory::make_int_exp(loc, value),
            node::Exp::RealExp { loc: _, value } => factory::make_real_exp(loc, value),
            node::Exp::PiExp { loc: _ } => factory::make_pi_exp(loc),
            node::Exp::BinopExp {
                loc: _,
                left,
                op,
                right,
            } => factory::make_binop_exp(loc, *left, op, *right),
            node::Exp::UnaryExp { loc: _, op, exp } => factory::make_unary_exp(loc, op, *exp),
        };

        Some(res)
    }

    fn parse_negative_term_exp(&mut self) -> Option<node::Exp> {
        let begin = peek_token!(self; in [TokenTy::Minus])?;
        let mut loc = begin.get_loc().clone();
        self.lexer.drop();

        let exp = self.parse_term_exp()?;
        loc.end_to_end(exp.get_loc());

        Some(factory::make_unary_exp(loc, node::UnaryOp::Minus, exp))
    }

    fn parse_unary_term_exp(&mut self) -> Option<node::Exp> {
        let begin = peek_token!(self; in [
            TokenTy::Sin,
            TokenTy::Cos,
            TokenTy::Tan,
            TokenTy::Exp,
            TokenTy::Ln,
            TokenTy::Sqrt
        ])?;
        let mut loc = begin.get_loc().clone();
        let op = match begin.get_ty() {
            TokenTy::Sin => node::UnaryOp::Sin,
            TokenTy::Cos => node::UnaryOp::Cos,
            TokenTy::Tan => node::UnaryOp::Tan,
            TokenTy::Exp => node::UnaryOp::Exp,
            TokenTy::Ln => node::UnaryOp::Ln,
            TokenTy::Sqrt => node::UnaryOp::Sqrt,
            _ => return None,
        };
        self.lexer.drop();

        peek_token!(self; in [TokenTy::LPar])?;
        self.lexer.drop();

        let exp = self.parse_exp()?;

        let end = peek_token!(self; in [TokenTy::RPar])?;
        loc.end_to_end(end.get_loc());
        self.lexer.drop();

        Some(factory::make_unary_exp(loc, op, exp))
    }

    fn parse_term_exp(&mut self) -> Option<node::Exp> {
        let token = peek_token!(self; in [
            TokenTy::Real(_),
            TokenTy::Integer(_),
            TokenTy::Pi,
            TokenTy::LPar,
            TokenTy::Minus,
            TokenTy::Sin,
            TokenTy::Cos,
            TokenTy::Tan,
            TokenTy::Exp,
            TokenTy::Ln,
            TokenTy::Sqrt
        ])?;
        match token.get_ty() {
            TokenTy::Real(_) | TokenTy::Integer(_) | TokenTy::Pi => self.parse_simple_term_exp(),
            TokenTy::LPar => self.parse_delimited_term_exp(),
            TokenTy::Minus => self.parse_negative_term_exp(),
            TokenTy::Sin
            | TokenTy::Cos
            | TokenTy::Tan
            | TokenTy::Exp
            | TokenTy::Ln
            | TokenTy::Sqrt => self.parse_unary_term_exp(),
            _ => return None,
        }
    }

    fn parse_exponential_exp(&mut self) -> Option<node::Exp> {
        let left = self.parse_term_exp()?;
        let mut loc = left.get_loc().clone();

        if !is_token!(self.lexer.peek().as_ref()?; in [TokenTy::Pow]) {
            return Some(left);
        }
        self.lexer.drop();

        let right = self.parse_exponential_exp()?;
        loc.end_to_end(right.get_loc());

        Some(factory::make_binop_exp(
            loc,
            left,
            node::BinopTy::Pow,
            right,
        ))
    }

    fn parse_multiplicative_exp(&mut self) -> Option<node::Exp> {
        let mut left = self.parse_exponential_exp()?;
        let mut loc = left.get_loc().clone();
        loop {
            let op = match self.lexer.peek().as_ref()?.get_ty() {
                TokenTy::Mul => node::BinopTy::Mul,
                TokenTy::Div => node::BinopTy::Div,
                _ => break,
            };
            self.lexer.drop();

            let right = self.parse_exponential_exp()?;
            loc.end_to_end(right.get_loc());

            left = factory::make_binop_exp(loc.clone(), left, op, right)
        }
        Some(left)
    }

    fn parse_additive_exp(&mut self) -> Option<node::Exp> {
        let mut left = self.parse_multiplicative_exp()?;
        let mut loc = left.get_loc().clone();
        loop {
            let op = match self.lexer.peek().as_ref()?.get_ty() {
                TokenTy::Plus => node::BinopTy::Add,
                TokenTy::Minus => node::BinopTy::Sub,
                _ => break,
            };
            self.lexer.drop();

            let right = self.parse_multiplicative_exp()?;
            loc.end_to_end(right.get_loc());

            left = factory::make_binop_exp(loc.clone(), left, op, right)
        }
        Some(left)
    }

    fn parse_exp(&mut self) -> Option<node::Exp> {
        self.parse_additive_exp()
    }

    pub fn get_error(&self) -> &Option<Error> {
        let err = self.lexer.get_error();
        if err.is_some() {
            err
        } else {
            &self.error
        }
    }
}
