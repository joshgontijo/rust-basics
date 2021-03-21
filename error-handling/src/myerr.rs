use std::{io, error, fmt, result};
use std::fmt::{Display, Formatter};

pub(crate) type AppResult<T> = result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {
    IO(io::Error),
    Serialization(bincode::Error),
    MyAppError,
    Custom(String),
}

impl error::Error for AppError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            AppError::IO(ref io) => Some(io),
            AppError::Serialization(ref ser) => Some(ser),
            AppError::MyAppError => None,
            AppError::Custom(_) => None,
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AppError::IO(ref io) => write!(f, "IO Error"),
            AppError::Serialization(ref ser) => write!(f, "Serialization error"),
            AppError::Custom(ref msg) => write!(f, "Custom Error [{}]", msg),
            AppError::MyAppError => write!(f, "App error 2"),
        }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IO(err)
    }
}

impl From<bincode::Error> for AppError {
    fn from(err: bincode::Error) -> AppError {
        AppError::Serialization(err)
    }
}