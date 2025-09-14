use std::rc::Rc;

use crate::bind::gate;
use crate::bind::reg;

use crate::utils::location::Location;

pub enum RegTy {
    QReg,
    CReg,
}

impl ToString for RegTy {
    fn to_string(&self) -> String {
        match self {
            RegTy::QReg => String::from("qreg"),
            RegTy::CReg => String::from("creg"),
        }
    }
}

pub enum Dec {
    RegDec {
        loc: Location,
        name: String,
        ty: RegTy,
        size: u32,
        dec: Option<Rc<reg::RegDec>>,
    },
}

impl Dec {
    pub fn get_loc(&self) -> &Location {
        match self {
            Dec::RegDec {
                loc,
                name: _,
                ty: _,
                size: _,
                dec: _,
            } => &loc,
        }
    }
}

pub enum Reg {
    SimpleReg {
        loc: Location,
        name: String,
        dec: Option<Rc<reg::RegDec>>,
    },
    SubscriptReg {
        loc: Location,
        name: String,
        index: u32,
        dec: Option<Rc<reg::RegDec>>,
    },
}

impl Reg {
    pub fn get_loc(&self) -> &Location {
        match self {
            Reg::SimpleReg {
                loc,
                name: _,
                dec: _,
            } => &loc,
            Reg::SubscriptReg {
                loc,
                name: _,
                index: _,
                dec: _,
            } => &loc,
        }
    }
}

pub enum Stmt {
    DecStmt(Dec),
    GateStmt {
        loc: Location,
        gate: String,
        pars: Vec<Exp>,
        args: Vec<Reg>,
        dec: Option<Rc<gate::GateDec>>,
    },
    MeasureStmt {
        loc: Location,
        src: Reg,
        dst: Reg,
    },
    ResetStmt {
        loc: Location,
        reg: Reg,
    },
}

impl Stmt {
    pub fn get_loc(&self) -> &Location {
        match self {
            Stmt::DecStmt(dec) => dec.get_loc(),
            Stmt::GateStmt {
                loc,
                gate: _,
                pars: _,
                args: _,
                dec: _,
            } => &loc,
            Stmt::MeasureStmt {
                loc,
                src: _,
                dst: _,
            } => &loc,
            Stmt::ResetStmt { loc, reg: _ } => &loc,
        }
    }
}

pub enum BinopTy {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl ToString for BinopTy {
    fn to_string(&self) -> String {
        match self {
            BinopTy::Add => String::from("+"),
            BinopTy::Sub => String::from("-"),
            BinopTy::Mul => String::from("*"),
            BinopTy::Div => String::from("/"),
            BinopTy::Pow => String::from("^"),
        }
    }
}

pub enum UnaryOp {
    Minus,
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
}

impl ToString for UnaryOp {
    fn to_string(&self) -> String {
        match self {
            UnaryOp::Minus => String::from("-"),
            UnaryOp::Sin => String::from("sin"),
            UnaryOp::Cos => String::from("cos"),
            UnaryOp::Tan => String::from("tan"),
            UnaryOp::Exp => String::from("exp"),
            UnaryOp::Ln => String::from("ln"),
            UnaryOp::Sqrt => String::from("sqrt"),
        }
    }
}

pub enum Exp {
    IntExp {
        loc: Location,
        value: u32,
    },
    RealExp {
        loc: Location,
        value: f32,
    },
    PiExp {
        loc: Location,
    },
    BinopExp {
        loc: Location,
        left: Box<Exp>,
        op: BinopTy,
        right: Box<Exp>,
    },
    UnaryExp {
        loc: Location,
        op: UnaryOp,
        exp: Box<Exp>,
    },
}

impl Exp {
    pub fn get_loc(&self) -> &Location {
        match self {
            Exp::IntExp { loc, value: _ } => &loc,
            Exp::RealExp { loc, value: _ } => &loc,
            Exp::PiExp { loc } => &loc,
            Exp::BinopExp {
                loc,
                left: _,
                op: _,
                right: _,
            } => &loc,
            Exp::UnaryExp { loc, op: _, exp: _ } => &loc,
        }
    }
}
