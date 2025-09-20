use std::collections::HashMap;
use std::rc::Rc;
use std::vec::Vec;

use crate::ast::node;
use crate::ast::node::RegTy;
use crate::ast::visitor::MutVisitor;

use crate::bind::gate::GateDec;
use crate::bind::par::ParDec;
use crate::bind::reg::RegDec;

use crate::utils::error::{CompoundError, ErrorTy, LocatedError};

pub struct Binder {
    gates: HashMap<String, Rc<GateDec>>,
    regs: HashMap<String, Rc<RegDec>>,
    error: CompoundError,
}

impl MutVisitor for Binder {
    fn visit_dec(&mut self, e: &mut node::Dec) {
        match e {
            node::Dec::RegDec {
                loc,
                name,
                ty,
                size,
                dec,
            } => {
                if self.regs.contains_key(name) {
                    self.error.add(LocatedError::new(
                        ErrorTy::Bind,
                        format!("redefined register '{}'", name),
                        loc.clone(),
                    ));
                    return;
                }
                let it = Rc::new(RegDec::new(name.clone(), ty.clone(), *size));
                dec.replace(Rc::clone(&it));
                self.regs.insert(name.clone(), it);
            }
        }
    }

    fn visit_reg(&mut self, e: &mut node::Reg) {
        match e {
            node::Reg::SimpleReg { loc, name, dec } => match self.regs.get(name) {
                Some(it) => drop(dec.replace(Rc::clone(it))),
                _ => self.error.add(LocatedError::new(
                    ErrorTy::Bind,
                    format!("undeclared register '{}'", name),
                    loc.clone(),
                )),
            },
            node::Reg::SubscriptReg {
                loc,
                name,
                index: _,
                dec,
            } => match self.regs.get(name) {
                Some(it) => drop(dec.replace(Rc::clone(it))),
                _ => self.error.add(LocatedError::new(
                    ErrorTy::Bind,
                    format!("undeclared register '{}'", name),
                    loc.clone(),
                )),
            },
        }
    }

    fn visit_stmt(&mut self, e: &mut node::Stmt) {
        match e {
            node::Stmt::DecStmt(dec) => self.visit_dec(dec),
            node::Stmt::GateStmt {
                loc,
                gate,
                pars: _,
                args,
                dec,
            } => {
                match self.gates.get(gate) {
                    Some(it) => drop(dec.replace(Rc::clone(it))),
                    _ => self.error.add(LocatedError::new(
                        ErrorTy::Bind,
                        format!("undeclared gate '{}'", gate),
                        loc.clone(),
                    )),
                };
                args.iter_mut().for_each(|arg| self.visit_reg(arg));
            }
            node::Stmt::MeasureStmt { loc: _, src, dst } => {
                self.visit_reg(src);
                self.visit_reg(dst);
            }
            node::Stmt::ResetStmt { loc: _, reg } => self.visit_reg(reg),
        }
    }

    fn visit_exp(&mut self, _: &mut node::Exp) {}
}

impl Binder {
    pub fn new() -> Binder {
        let ugate = Rc::new(GateDec::new(
            String::from("U"),
            vec![
                Rc::new(ParDec::new(String::from("theta"))),
                Rc::new(ParDec::new(String::from("phi"))),
                Rc::new(ParDec::new(String::from("lambda"))),
            ],
            vec![Rc::new(RegDec::new_bit(String::from("qubit"), RegTy::QReg))],
        ));
        let cxgate = Rc::new(GateDec::new(
            String::from("CX"),
            vec![],
            vec![
                Rc::new(RegDec::new_bit(String::from("control"), RegTy::QReg)),
                Rc::new(RegDec::new_bit(String::from("target"), RegTy::QReg)),
            ],
        ));

        let mut gates = HashMap::new();
        gates.insert(ugate.get_name().clone(), ugate);
        gates.insert(cxgate.get_name().clone(), cxgate);

        Binder {
            gates: gates,
            regs: HashMap::new(),
            error: CompoundError::new(),
        }
    }

    pub fn bind(&mut self, program: &mut Vec<node::Stmt>) {
        program.iter_mut().for_each(|x| self.visit_stmt(x));
    }

    pub fn get_error(&self) -> &CompoundError {
        &self.error
    }

    pub fn get_error_mut(&mut self) -> &mut CompoundError {
        &mut self.error
    }
}
