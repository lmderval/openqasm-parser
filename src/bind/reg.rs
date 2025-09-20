use crate::ast::node::RegTy;

use crate::typing::ty::Ty;

pub struct RegDec {
    name: String,
    ty: Ty,
}

impl ToString for RegDec {
    fn to_string(&self) -> String {
        format!("{:p} ({},{})", self, self.ty.to_string(), self.name)
    }
}

impl RegDec {
    pub fn new(name: String, ty: RegTy, size: u32) -> RegDec {
        RegDec {
            name: name,
            ty: match ty {
                RegTy::QReg => Ty::QRegTy(size),
                RegTy::CReg => Ty::CRegTy(size),
            },
        }
    }

    pub fn new_bit(name: String, ty: RegTy) -> RegDec {
        RegDec {
            name: name,
            ty: match ty {
                RegTy::QReg => Ty::QubitTy,
                RegTy::CReg => Ty::BitTy,
            },
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_ty(&self) -> &Ty {
        &self.ty
    }
}
