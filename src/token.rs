use crate::location::Location;

pub enum TokenTy {
    OpenQASM,

    Integer(u32),
    Real(f32),
    Id(String),

    QReg,
    CReg,

    UGate,
    CXGate,

    Measure,
    Reset,

    Pi,
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
    Sqrt,

    LBrack,
    RBrack,
    LPar,
    RPar,

    Semi,
    Comma,

    Arrow,

    Plus,
    Minus,
    Mul,
    Div,
    Pow,

    Eof,
}

impl ToString for TokenTy {
    fn to_string(&self) -> String {
        match self {
            TokenTy::OpenQASM => String::from("OPENQASM"),
            TokenTy::Integer(n) => format!("INTEGER({})", n),
            TokenTy::Real(x) => format!("REAL({})", x),
            TokenTy::Id(s) => format!("ID({})", s),
            TokenTy::QReg => String::from("QREG"),
            TokenTy::CReg => String::from("CREG"),
            TokenTy::UGate => String::from("UGATE"),
            TokenTy::CXGate => String::from("CXGATE"),
            TokenTy::Measure => String::from("MEASURE"),
            TokenTy::Reset => String::from("RESET"),
            TokenTy::Pi => String::from("PI"),
            TokenTy::Sin => String::from("SIN"),
            TokenTy::Cos => String::from("COS"),
            TokenTy::Tan => String::from("TAN"),
            TokenTy::Exp => String::from("EXP"),
            TokenTy::Ln => String::from("LN"),
            TokenTy::Sqrt => String::from("SQRT"),
            TokenTy::LBrack => String::from("LBRACK"),
            TokenTy::RBrack => String::from("RBRACK"),
            TokenTy::LPar => String::from("LPAR"),
            TokenTy::RPar => String::from("RPAR"),
            TokenTy::Semi => String::from("SEMI"),
            TokenTy::Comma => String::from("COMMA"),
            TokenTy::Arrow => String::from("ARROW"),
            TokenTy::Plus => String::from("PLUS"),
            TokenTy::Minus => String::from("MINUS"),
            TokenTy::Mul => String::from("MUL"),
            TokenTy::Div => String::from("DIV"),
            TokenTy::Pow => String::from("POW"),
            TokenTy::Eof => String::from("EOF"),
        }
    }
}

pub struct Token {
    ty: TokenTy,
    loc: Location,
}

pub fn build_token(ty: TokenTy, loc: Location) -> Token {
    Token { ty: ty, loc: loc }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        self.ty.to_string() + "@" + &self.loc.to_string()
    }
}

impl Token {
    pub fn get_ty(&self) -> &TokenTy {
        &self.ty
    }
}
