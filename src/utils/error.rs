use crate::utils::location::Location;

pub enum ErrorTy {
    Ok,
    Internal,
    Lex,
    Parse,
    Bind,
    Type,
    Sanity,
}

impl Clone for ErrorTy {
    fn clone(&self) -> Self {
        match self {
            Self::Ok => Self::Ok,
            Self::Internal => Self::Internal,
            Self::Lex => Self::Lex,
            Self::Parse => Self::Parse,
            Self::Bind => Self::Bind,
            Self::Type => Self::Type,
            Self::Sanity => Self::Sanity,
        }
    }
}

impl ErrorTy {
    fn exit_code(&self) -> i32 {
        match self {
            ErrorTy::Ok => 0,
            ErrorTy::Internal => 1,
            ErrorTy::Lex => 2,
            ErrorTy::Parse => 3,
            ErrorTy::Bind => 4,
            ErrorTy::Type => 5,
            ErrorTy::Sanity => 6,
        }
    }
}

pub trait Error {
    fn get_ty(&self) -> ErrorTy;
    fn get_desc(&self) -> String;

    fn get_exit_code(&self) -> i32 {
        self.get_ty().exit_code()
    }
}

pub struct SimpleError {
    ty: ErrorTy,
    desc: String,
}

impl Error for SimpleError {
    fn get_ty(&self) -> ErrorTy {
        self.ty.clone()
    }

    fn get_desc(&self) -> String {
        self.desc.clone()
    }
}

impl SimpleError {
    pub fn new<T: ToString>(ty: ErrorTy, desc: T) -> SimpleError {
        SimpleError {
            ty: ty,
            desc: desc.to_string(),
        }
    }
}

pub struct LocatedError {
    ty: ErrorTy,
    desc: String,
    loc: Location,
}

impl Error for LocatedError {
    fn get_ty(&self) -> ErrorTy {
        self.ty.clone()
    }

    fn get_desc(&self) -> String {
        format!("{} at {}", self.desc, self.loc.to_string())
    }
}

impl LocatedError {
    pub fn new<T: ToString>(ty: ErrorTy, desc: T, loc: Location) -> LocatedError {
        LocatedError {
            ty: ty,
            desc: desc.to_string(),
            loc: loc,
        }
    }
}

pub struct CompoundError {
    errors: Vec<Box<dyn Error>>,
}

impl Error for CompoundError {
    fn get_ty(&self) -> ErrorTy {
        self.errors
            .iter()
            .min_by_key(|x| x.get_exit_code())
            .map(|x| x.get_ty())
            .unwrap_or(ErrorTy::Ok)
    }

    fn get_desc(&self) -> String {
        self.errors
            .iter()
            .map(|x| x.get_desc())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl CompoundError {
    pub fn new() -> CompoundError {
        CompoundError { errors: Vec::new() }
    }

    pub fn add(&mut self, error: impl Error + 'static) {
        self.errors.push(Box::new(error));
    }

    pub fn consume(&mut self, other: &mut CompoundError) {
        while !other.empty() {
            self.errors.push(other.errors.remove(0));
        }
    }

    pub fn empty(&self) -> bool {
        self.errors.is_empty()
    }
}
