use crate::location::Location;

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
    },
}

pub enum Reg {
    SimpleReg {
        loc: Location,
        name: String,
    },
    SubscriptReg {
        loc: Location,
        name: String,
        index: u32,
    },
}

pub enum Stmt {
    DecStmt(Dec),
    GateStmt {
        loc: Location,
        gate: String,
        pars: Vec<Exp>,
        args: Vec<Reg>,
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
