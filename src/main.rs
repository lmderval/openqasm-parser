use crate::ast::{factory, node};

pub mod ast;
pub mod chars;
pub mod lexer;
pub mod location;
pub mod token;

fn main() {
    let loc = location::build_location(String::from("<stdin>"), 0, 0, 0, 0);
    let program = vec![
        factory::make_reg_dec(loc.clone(), String::from("qr0"), node::RegTy::QReg, 2),
        factory::make_reg_dec(loc.clone(), String::from("qr1"), node::RegTy::QReg, 2),
        factory::make_reg_dec(loc.clone(), String::from("cr0"), node::RegTy::CReg, 2),
        factory::make_reg_dec(loc.clone(), String::from("cr1"), node::RegTy::CReg, 2),
        factory::make_gate_stmt(
            loc.clone(),
            String::from("U"),
            vec![
                factory::make_binop_exp(
                    loc.clone(),
                    factory::make_pi_exp(loc.clone()),
                    node::BinopTy::Div,
                    factory::make_int_exp(loc.clone(), 2),
                ),
                factory::make_int_exp(loc.clone(), 0),
                factory::make_pi_exp(loc.clone()),
            ],
            vec![factory::make_simple_reg(loc.clone(), String::from("qr0"))],
        ),
        factory::make_gate_stmt(
            loc.clone(),
            String::from("CX"),
            vec![],
            vec![
                factory::make_subscript_reg(loc.clone(), String::from("qr0"), 0),
                factory::make_subscript_reg(loc.clone(), String::from("qr1"), 0),
            ],
        ),
        factory::make_measure_stmt(
            loc.clone(),
            factory::make_simple_reg(loc.clone(), String::from("qr0")),
            factory::make_simple_reg(loc.clone(), String::from("cr0")),
        ),
        factory::make_measure_stmt(
            loc.clone(),
            factory::make_simple_reg(loc.clone(), String::from("qr1")),
            factory::make_simple_reg(loc.clone(), String::from("cr1")),
        ),
    ];
}
