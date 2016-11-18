pub use std::convert;
pub use std::io::prelude::*;

use std::error;
use std::fmt;
use std::io;
use std::num;

/// Error struct used for generating an `io::Error` from a generic description.
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

/// Returns an `io::Error` from description string and optional detail string.
/// Particularly useful in `Result` expressions.
pub fn other_io_err(descr: &'static str, detail: Option<String>) -> io::Error {
    convert::From::from(
        Error::new(descr, detail)
    )
}

/// maps a `ParseIntError` to an `io::Error`
pub fn pie2io(p: num::ParseIntError) -> io::Error {
    convert::From::from(
        Error::new("Integer parsing error", Some(p.to_string()))
    )
}

#[test]
fn test_custom_error() {
    let desc = "Boring error description";
    let detail = "More of it";
    let ewd = Error::new(desc, Some(detail.to_string()));

    assert_eq!(error::Error::description(&ewd), desc);
    match error::Error::cause(&ewd) {
        Some(_)  => assert!(false),
        None   => assert!(true),
    }
    let _formated_err = format!("{}", ewd);
    let ewod = Error::new(desc, None);
    let _formated_err = format!("{}", ewod);
}
