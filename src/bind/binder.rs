use std::collections::HashMap;
use std::io::Error;
use std::rc::Rc;
use std::vec::Vec;

use crate::ast::node;
use crate::ast::node::RegTy;
use crate::ast::visitor::MutVisitor;

use crate::bind::gate::GateDec;
use crate::bind::par::ParDec;
use crate::bind::reg::RegDec;

pub struct Binder {
    gates: HashMap<String, Rc<GateDec>>,
    regs: HashMap<String, Rc<RegDec>>,
    error: Option<Error>,
}

impl MutVisitor for Binder {
    fn visit_dec(&mut self, e: &mut node::Dec) {
        match e {
            node::Dec::RegDec {
                loc: _,
                name,
                ty,
                size,
                dec,
            } => {
                if self.regs.contains_key(name) {
                    self.error = Some(Error::other(format!("Redefined register '{}'", name)));
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
            node::Reg::SimpleReg { loc: _, name, dec } => match self.regs.get(name) {
                Some(it) => drop(dec.replace(Rc::clone(it))),
                _ => self.error = Some(Error::other(format!("Undeclared register '{}'", name))),
            },
            node::Reg::SubscriptReg {
                loc: _,
                name,
                index: _,
                dec,
            } => match self.regs.get(name) {
                Some(it) => drop(dec.replace(Rc::clone(it))),
                _ => self.error = Some(Error::other(format!("Undeclared register '{}'", name))),
            },
        }
    }

    fn visit_stmt(&mut self, e: &mut node::Stmt) {
        match e {
            node::Stmt::DecStmt(dec) => self.visit_dec(dec),
            node::Stmt::GateStmt {
                loc: _,
                gate,
                pars: _,
                args,
                dec,
            } => {
                match self.gates.get(gate) {
                    Some(it) => drop(dec.replace(Rc::clone(it))),
                    _ => self.error = Some(Error::other(format!("Undeclared gate '{}'", gate))),
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
            vec![Rc::new(RegDec::new(String::from("qubit"), RegTy::QReg, 1))],
        ));
        let cxgate = Rc::new(GateDec::new(
            String::from("CX"),
            vec![],
            vec![
                Rc::new(RegDec::new(String::from("control"), RegTy::QReg, 1)),
                Rc::new(RegDec::new(String::from("target"), RegTy::QReg, 1)),
            ],
        ));

        let mut gates = HashMap::new();
        gates.insert(ugate.get_name().clone(), ugate);
        gates.insert(cxgate.get_name().clone(), cxgate);

        Binder {
            gates: gates,
            regs: HashMap::new(),
            error: None,
        }
    }

    pub fn bind(&mut self, program: &mut Vec<node::Stmt>) {
        program.iter_mut().for_each(|x| self.visit_stmt(x));
    }

    pub fn get_error(&self) -> &Option<Error> {
        &self.error
    }
}
