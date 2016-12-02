use std::convert;
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
    /// Initializes a new error with a description and optional detail string.
    pub fn new(descr: &'static str, detail: Option<String>) -> Error {
        Error {
            descr: descr,
            detail: detail,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.detail {
            None => write!(f, "{}", self.descr),
            Some(ref detail) => write!(f, "{} ({})", self.descr, detail),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.descr
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<Error> for io::Error {
    fn from(me: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, me)
    }
}

/// Returns an `io::Error` from description string and optional detail string.
/// Particularly useful in `Result` expressions.
pub fn other_io_err(descr: &'static str, detail: Option<String>) -> io::Error {
    convert::From::from(Error::new(descr, detail))
}

/// maps a `ParseIntError` to an `io::Error`
pub fn pie2io(p: num::ParseIntError) -> io::Error {
    convert::From::from(Error::new("Integer parsing error", Some(p.to_string())))
}

#[cfg(test)]
mod tests_custom_err {
    use std::error;
    use custom_error;

    #[test]
    fn test_custom_error() {
        let desc = "Boring error description";
        let detail = "More of it";
        let ewd = custom_error::Error::new(desc, Some(detail.to_string()));

        assert_eq!(error::Error::description(&ewd), desc);
        match error::Error::cause(&ewd) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        let _formated_err = format!("{}", ewd);
        let ewod = custom_error::Error::new(desc, None);
        let _formated_err = format!("{}", ewod);
    }
}

#[cfg(test)]
mod tests_std_err {
    use std::error::Error;
    use custom_error::pie2io;

    #[test]
    fn test_parse_errors() {
        let nan = "2a".to_string();
        match nan.parse::<u8>().map_err(pie2io) {
            Ok(_) => assert!(false),
            Err(x) => assert_eq!("Integer parsing error", x.description()),
        }
    }
}
