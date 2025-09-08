use crate::location::Location;

pub enum RegTy {
    QReg,
    CReg,
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

pub enum UnaryOp {
    Minus,
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,
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
