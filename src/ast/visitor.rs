use crate::ast::node;

pub trait Visitor {
    fn visit_dec(&mut self, e: &node::Dec);
    fn visit_reg(&mut self, e: &node::Reg);
    fn visit_stmt(&mut self, e: &node::Stmt);
    fn visit_exp(&mut self, e: &node::Exp);
}

pub trait MutVisitor {
    fn visit_dec(&mut self, e: &mut node::Dec);
    fn visit_reg(&mut self, e: &mut node::Reg);
    fn visit_stmt(&mut self, e: &mut node::Stmt);
    fn visit_exp(&mut self, e: &mut node::Exp);
}
