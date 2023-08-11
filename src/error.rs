use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error<'static>>;

#[derive(Debug)]
pub enum Error<'s> {
    Other(&'s str),
    FromIo(io::Error),
    FromSlice(std::array::TryFromSliceError),
    FromUtf8Error(std::string::FromUtf8Error),
}

impl<'s> fmt::Display for Error<'s> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Other(msg) => write!(f, "Other error: {}", msg),
            Self::FromIo(e) => write!(f, "io error {}:", e),
            Self::FromSlice(e) => write!(f, "slice error: {}", e),
            Self::FromUtf8Error(e) => write!(f, "utf8 error: {}", e),
        }
    }
}

impl<'s> From<std::array::TryFromSliceError> for Error<'s> {
    fn from(e: std::array::TryFromSliceError) -> Self {
        Self::FromSlice(e)
    }
}

impl<'s> From<io::Error> for Error<'s> {
    fn from(e: io::Error) -> Self {
        Self::FromIo(e)
    }
}

impl<'s> From<std::string::FromUtf8Error> for Error<'s> {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8Error(e)
    }
}

impl<'s> std::error::Error for Error<'s> {}
