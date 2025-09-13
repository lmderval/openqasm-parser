use std::vec::Vec;

use crate::ast::node;
use crate::ast::visitor::Visitor;

pub struct PrettyPrinter;

impl Visitor for PrettyPrinter {
    fn visit_dec(&mut self, e: &node::Dec) {
        match e {
            &node::Dec::RegDec {
                loc: _,
                ref name,
                ref ty,
                size,
            } => println!("{} {}[{}];", ty.to_string(), name, size),
        }
    }

    fn visit_reg(&mut self, e: &node::Reg) {
        match e {
            &node::Reg::SimpleReg { loc: _, ref name } => print!("{}", name),
            &node::Reg::SubscriptReg {
                loc: _,
                ref name,
                index,
            } => print!("{}[{}]", name, index),
        }
    }

    fn visit_stmt(&mut self, e: &node::Stmt) {
        match e {
            &node::Stmt::DecStmt(ref dec) => self.visit_dec(dec),
            &node::Stmt::GateStmt {
                loc: _,
                ref gate,
                ref pars,
                ref args,
            } => {
                print!("{} ", gate);
                if !pars.is_empty() {
                    print!("(");
                    self.visit_exp(&pars[0]);
                    pars[1..].iter().for_each(|e| {
                        print!(", ");
                        self.visit_exp(e);
                    });
                    print!(") ");
                }
                self.visit_reg(&args[0]);
                args[1..].iter().for_each(|e| {
                    print!(", ");
                    self.visit_reg(e);
                });
                println!(";");
            }
            &node::Stmt::MeasureStmt {
                loc: _,
                ref src,
                ref dst,
            } => {
                print!("measure ");
                self.visit_reg(src);
                print!(" -> ");
                self.visit_reg(dst);
                println!(";");
            }
            &node::Stmt::ResetStmt { loc: _, ref reg } => {
                print!("reset ");
                self.visit_reg(reg);
                println!(";");
            }
        }
    }

    fn visit_exp(&mut self, e: &node::Exp) {
        match e {
            &node::Exp::IntExp { loc: _, value } => print!("{}", value),
            &node::Exp::RealExp { loc: _, value } => print!("{}", value),
            &node::Exp::PiExp { loc: _ } => print!("pi"),
            &node::Exp::BinopExp {
                loc: _,
                ref left,
                ref op,
                ref right,
            } => {
                print!("(");
                self.visit_exp(left.as_ref());
                print!(" {} ", op.to_string());
                self.visit_exp(right.as_ref());
                print!(")");
            }
            &node::Exp::UnaryExp {
                loc: _,
                ref op,
                ref exp,
            } => {
                print!("{}(", op.to_string());
                self.visit_exp(exp.as_ref());
                print!(")");
            }
        }
    }
}

impl PrettyPrinter {
    pub fn pretty_print(&mut self, program: &Vec<node::Stmt>) {
        println!("OPENQASM 2.0;");
        program.iter().for_each(|e| self.visit_stmt(e));
    }
}
