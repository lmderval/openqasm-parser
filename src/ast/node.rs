use crate::utils::location::Location;

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

impl Dec {
    pub fn get_loc(&self) -> &Location {
        match self {
            Dec::RegDec {
                loc,
                name: _,
                ty: _,
                size: _,
            } => &loc,
        }
    }
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

impl Reg {
    pub fn get_loc(&self) -> &Location {
        match self {
            Reg::SimpleReg { loc, name: _ } => &loc,
            Reg::SubscriptReg {
                loc,
                name: _,
                index: _,
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
