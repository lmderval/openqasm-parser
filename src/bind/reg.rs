use crate::ast::node::RegTy;

pub struct RegDec {
    name: String,
    ty: RegTy,
    size: u32,
}

impl ToString for RegDec {
    fn to_string(&self) -> String {
        format!(
            "{:p} ({},{},{})",
            self,
            self.ty.to_string(),
            self.name,
            self.size
        )
    }
}

impl RegDec {
    pub fn new(name: String, ty: RegTy, size: u32) -> RegDec {
        RegDec {
            name: name,
            ty: ty,
            size: size,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
