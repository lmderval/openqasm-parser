use std::rc::Rc;
use std::vec::Vec;

use crate::bind::par::ParDec;
use crate::bind::reg::RegDec;

pub struct GateDec {
    name: String,
    pars: Vec<Rc<ParDec>>,
    args: Vec<Rc<RegDec>>,
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
        GateDec {
            name: name,
            pars: pars,
            args: args,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
