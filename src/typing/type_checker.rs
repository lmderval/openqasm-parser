use crate::ast::node;
use crate::ast::visitor::Visitor;

use crate::bind::gate::GateDec;
use crate::bind::reg::RegDec;

use crate::typing::ty::Ty;

use crate::utils::error::{CompoundError, ErrorTy, LocatedError, SimpleError};
use crate::utils::location::Location;

pub struct TypeChecker {
    ty: Ty,
    error: CompoundError,
}

impl Visitor for TypeChecker {
    fn visit_dec(&mut self, _: &node::Dec) {}

    fn visit_reg(&mut self, e: &node::Reg) {
        match e {
            node::Reg::SimpleReg {
                loc: _,
                name: _,
                dec,
            } => {
                self.ty = match dec {
                    Some(dec) => dec.get_ty().clone(),
                    None => Ty::QubitTy,
                }
            }
            node::Reg::SubscriptReg {
                loc,
                name: _,
                index,
                dec,
            } => {
                self.ty = match dec {
                    Some(dec) => self.access_register(dec, *index, loc),
                    None => Ty::QubitTy,
                }
            }
        }
    }

    fn visit_stmt(&mut self, e: &node::Stmt) {
        match e {
            node::Stmt::DecStmt(_) => {}
            node::Stmt::GateStmt {
                loc,
                gate: _,
                pars,
                args,
                dec,
            } => {
                if let Some(dec) = dec {
                    self.check_gate(dec, pars, args, loc);
                }
                self.check_args(args);
            }
            node::Stmt::MeasureStmt { loc: _, src, dst } => self.check_measure(src, dst),
            node::Stmt::ResetStmt { loc: _, reg } => self.check_reset(reg),
        }
    }

    fn visit_exp(&mut self, _: &node::Exp) {}
}

impl TypeChecker {
    pub fn new() -> TypeChecker {
        TypeChecker {
            ty: Ty::QubitTy,
            error: CompoundError::new(),
        }
    }

    fn access_register(&mut self, dec: &RegDec, index: u32, loc: &Location) -> Ty {
        match dec.get_ty() {
            Ty::QubitTy => {
                self.error.add(LocatedError::new(
                    ErrorTy::Type,
                    "qubit type is not subscriptable",
                    loc.clone(),
                ));
                Ty::QubitTy
            }
            Ty::QRegTy(size) => {
                if index >= *size {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        "register index out of bounds",
                        loc.clone(),
                    ));
                }
                Ty::QubitTy
            }
            Ty::BitTy => {
                self.error.add(LocatedError::new(
                    ErrorTy::Type,
                    "bit type is not subscriptable",
                    loc.clone(),
                ));
                Ty::BitTy
            }
            Ty::CRegTy(size) => {
                if index >= *size {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        "register index out of bounds",
                        loc.clone(),
                    ));
                }
                Ty::BitTy
            }
            Ty::GateTy(_, _) => {
                self.error
                    .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
                Ty::QubitTy
            }
        }
    }

    fn check_gate(
        &mut self,
        dec: &GateDec,
        pars: &Vec<node::Exp>,
        args: &Vec<node::Reg>,
        loc: &Location,
    ) {
        match dec.get_ty() {
            Ty::QubitTy | Ty::QRegTy(_) | Ty::BitTy | Ty::CRegTy(_) => {
                self.error
                    .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
            }
            Ty::GateTy(npars, nargs) => {
                if *npars != pars.len() as u32 {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        format!(
                            "invalid number of parameters, expected {} got {}",
                            npars,
                            pars.len()
                        ),
                        loc.clone(),
                    ));
                }
                if *nargs != args.len() as u32 {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        format!(
                            "invalid number of arguments, expected {} got {}",
                            nargs,
                            args.len()
                        ),
                        loc.clone(),
                    ));
                }
            }
        }
    }

    fn check_args(&mut self, args: &Vec<node::Reg>) {
        let mut reg_size: Option<u32> = None;
        for arg in args {
            let ty = self.type_reg(arg);
            match ty {
                Ty::QubitTy => {}
                Ty::QRegTy(size) => match reg_size {
                    Some(reg_size) if size != reg_size => {
                        self.error.add(LocatedError::new(
                            ErrorTy::Type,
                            format!("expected a register of size {} got {}", size, reg_size),
                            arg.get_loc().clone(),
                        ));
                    }
                    Some(_) => {}
                    None => reg_size = Some(size),
                },
                Ty::BitTy | Ty::CRegTy(_) => {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        "expected a qubit or a qreg",
                        arg.get_loc().clone(),
                    ));
                }
                Ty::GateTy(_, _) => {
                    self.error
                        .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
                }
            }
        }
    }

    fn check_measure(&mut self, src: &node::Reg, dst: &node::Reg) {
        let mut reg_size: Option<u32> = None;
        match self.type_reg(src) {
            Ty::QubitTy => {}
            Ty::QRegTy(size) => reg_size = Some(size),
            Ty::BitTy | Ty::CRegTy(_) => {
                self.error.add(LocatedError::new(
                    ErrorTy::Type,
                    "expected a qubit or a qreg",
                    src.get_loc().clone(),
                ));
            }
            Ty::GateTy(_, _) => {
                self.error
                    .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
            }
        }
        match self.type_reg(dst) {
            Ty::QubitTy | Ty::QRegTy(_) => {
                self.error.add(LocatedError::new(
                    ErrorTy::Type,
                    "expected a bit or a creg",
                    dst.get_loc().clone(),
                ));
            }
            Ty::BitTy => {
                if let Some(reg_size) = reg_size {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        format!("expected a register of size {}", reg_size),
                        dst.get_loc().clone(),
                    ));
                }
            }
            Ty::CRegTy(size) => match reg_size {
                Some(reg_size) if size != reg_size => {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        format!("expected a register of size {}", reg_size),
                        dst.get_loc().clone(),
                    ));
                }
                Some(_) => {}
                None => {
                    self.error.add(LocatedError::new(
                        ErrorTy::Type,
                        "expected a bit",
                        dst.get_loc().clone(),
                    ));
                }
            },
            Ty::GateTy(_, _) => {
                self.error
                    .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
            }
        }
    }

    fn check_reset(&mut self, reg: &node::Reg) {
        match self.type_reg(reg) {
            Ty::QubitTy | Ty::QRegTy(_) => {}
            Ty::BitTy | Ty::CRegTy(_) => {
                self.error.add(LocatedError::new(
                    ErrorTy::Type,
                    "expected a qubit or a qreg",
                    reg.get_loc().clone(),
                ));
            }
            Ty::GateTy(_, _) => {
                self.error
                    .add(SimpleError::new(ErrorTy::Internal, "incoherent type"));
            }
        }
    }

    fn type_reg(&mut self, e: &node::Reg) -> Ty {
        self.visit_reg(e);
        self.ty.clone()
    }

    pub fn check_types(&mut self, program: &Vec<node::Stmt>) {
        program.iter().for_each(|x| self.visit_stmt(x));
    }

    pub fn get_error(&self) -> &CompoundError {
        &self.error
    }

    pub fn get_error_mut(&mut self) -> &mut CompoundError {
        &mut self.error
    }
}
