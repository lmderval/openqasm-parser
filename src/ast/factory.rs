use crate::ast::node;

use crate::utils::location::Location;

pub fn make_reg_dec(loc: Location, name: String, ty: node::RegTy, size: u32) -> node::Stmt {
    node::Stmt::DecStmt(node::Dec::RegDec {
        loc: loc,
        name: name,
        ty: ty,
        size: size,
    })
}

pub fn make_simple_reg(loc: Location, name: String) -> node::Reg {
    node::Reg::SimpleReg {
        loc: loc,
        name: name,
    }
}

pub fn make_subscript_reg(loc: Location, name: String, index: u32) -> node::Reg {
    node::Reg::SubscriptReg {
        loc: loc,
        name: name,
        index: index,
    }
}

pub fn make_gate_stmt(
    loc: Location,
    gate: String,
    pars: Vec<node::Exp>,
    args: Vec<node::Reg>,
) -> node::Stmt {
    node::Stmt::GateStmt {
        loc: loc,
        gate: gate,
        pars: pars,
        args: args,
    }
}

pub fn make_measure_stmt(loc: Location, src: node::Reg, dst: node::Reg) -> node::Stmt {
    node::Stmt::MeasureStmt {
        loc: loc,
        src: src,
        dst: dst,
    }
}

pub fn make_reset_stmt(loc: Location, reg: node::Reg) -> node::Stmt {
    node::Stmt::ResetStmt { loc: loc, reg: reg }
}

pub fn make_int_exp(loc: Location, value: u32) -> node::Exp {
    node::Exp::IntExp {
        loc: loc,
        value: value,
    }
}

pub fn make_real_exp(loc: Location, value: f32) -> node::Exp {
    node::Exp::RealExp {
        loc: loc,
        value: value,
    }
}

pub fn make_pi_exp(loc: Location) -> node::Exp {
    node::Exp::PiExp { loc: loc }
}

pub fn make_binop_exp(
    loc: Location,
    left: node::Exp,
    op: node::BinopTy,
    right: node::Exp,
) -> node::Exp {
    node::Exp::BinopExp {
        loc: loc,
        left: Box::new(left),
        op: op,
        right: Box::new(right),
    }
}

pub fn make_unary_exp(loc: Location, op: node::UnaryOp, exp: node::Exp) -> node::Exp {
    node::Exp::UnaryExp {
        loc: loc,
        op: op,
        exp: Box::new(exp),
    }
}
