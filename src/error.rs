use std::{fmt, io, num, result};

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    NoneError(String),
    ParseInt(num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;

        match self {
            IO(err) => write!(fmt, "IO error ({})", err),
            NoneError(err) => write!(fmt, "NoneError ({})", err),
            ParseInt(err) => write!(fmt, "ParseInt error ({})", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseInt(err)
    }
}
