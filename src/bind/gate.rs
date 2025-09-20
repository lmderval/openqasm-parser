use std::rc::Rc;
use std::vec::Vec;

use crate::bind::par::ParDec;
use crate::bind::reg::RegDec;

use crate::typing::ty::Ty;

pub struct GateDec {
    name: String,
    pars: Vec<Rc<ParDec>>,
    args: Vec<Rc<RegDec>>,
    ty: Ty,
}

impl ToString for GateDec {
    fn to_string(&self) -> String {
        format!(
            "{:p} ({},[{}],[{}])",
            self,
            self.name,
            self.pars
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(","),
            self.args
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl GateDec {
    pub fn new(name: String, pars: Vec<Rc<ParDec>>, args: Vec<Rc<RegDec>>) -> GateDec {
        let ty = Ty::GateTy(pars.len() as u32, args.len() as u32);
        GateDec {
            name: name,
            pars: pars,
            args: args,
            ty: ty,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_ty(&self) -> &Ty {
        &self.ty
    }
}
