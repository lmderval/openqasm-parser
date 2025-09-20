pub enum Ty {
    QubitTy,
    QRegTy(u32),
    BitTy,
    CRegTy(u32),
    GateTy(u32, u32),
}

impl Clone for Ty {
    fn clone(&self) -> Ty {
        match self {
            Ty::QubitTy => Ty::QubitTy,
            Ty::QRegTy(size) => Ty::QRegTy(*size),
            Ty::BitTy => Ty::BitTy,
            Ty::CRegTy(size) => Ty::CRegTy(*size),
            Ty::GateTy(pars, args) => Ty::GateTy(*pars, *args),
        }
    }
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        match self {
            Ty::QubitTy => String::from("qubit_ty"),
            Ty::QRegTy(size) => format!("qreg_ty[{}]", size),
            Ty::BitTy => String::from("bit_ty"),
            Ty::CRegTy(size) => format!("creg_ty[{}]", size),
            Ty::GateTy(pars, args) => format!("gate_ty[{},{}]", pars, args),
        }
    }
}
