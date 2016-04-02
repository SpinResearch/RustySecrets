pub use std::convert;
pub use std::io::prelude::*;

use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct Error {
    descr: &'static str,
    detail: Option<String>,
}

impl Error {
    pub fn new(descr: &'static str, detail: Option<String>) -> Error {
        Error { descr: descr, detail: detail }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.detail {
            None => write!(f, "{}", self.descr),
            Some(ref detail) => write!(f, "{} ({})", self.descr, detail)
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str { self.descr }
    fn cause(&self) -> Option<&error::Error> { None }
}

impl convert::From<Error> for io::Error {
    fn from(me: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, me)
    }
}

pub fn other_io_err(descr: &'static str, detail: Option<String>) -> io::Error {
    convert::From::from(
        Error::new(descr, detail)
    )
}
