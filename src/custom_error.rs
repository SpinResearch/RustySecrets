use std::convert;
use std::error;
use std::error::Error;
use std::fmt;
use std::io;
use std::num;

/// Error struct used for generating an `io::Error` from a generic description.
#[derive(Debug)]
pub struct RustyError {
    descr: &'static str,
    detail: Option<String>,
    share_num: Option<u8>,
}

impl RustyError {
    /// Initializes a new error with a description and optional detail string.
    pub fn new(descr: &'static str, detail: Option<String>, share_num: Option<u8>) -> RustyError {
        RustyError {
            descr: descr,
            detail: detail,
            share_num: share_num,
        }
    }

    pub fn share_num(&self) -> Option<u8> {
        self.share_num
    }
}

impl fmt::Display for RustyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.detail {
            None => write!(f, "{}", self.descr),
            Some(ref detail) => write!(f, "{} ({})", self.descr, detail),
        }
    }
}

impl error::Error for RustyError {
    fn description(&self) -> &str {
        self.descr
    }
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<io::Error> for RustyError {
    fn from(err: io::Error) -> RustyError {
        let descr = err.description().to_owned();

        RustyError::new("from io:Error", Some(descr), None)
    }
}

impl From<RustyError> for io::Error {
    fn from(me: RustyError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, me)
    }
}

/// Returns an `io::Error` from description string and optional detail string.
/// Particularly useful in `Result` expressions.
pub fn other_io_err(descr: &'static str, detail: Option<String>, share_num: Option<u8>) -> io::Error {
    convert::From::from(RustyError::new(descr, detail, share_num))
}

/// maps a `ParseIntError` to an `Error`
pub fn pie2error(p: num::ParseIntError) -> RustyError {
    RustyError::new("Integer parsing error", Some(p.to_string()), None)
}

#[cfg(test)]
mod tests_custom_err {
    use std::error;
    use custom_error::RustyError;

    #[test]
    fn test_custom_error() {
        let desc = "Boring error description";
        let detail = "More of it";
        let ewd = RustyError::new(desc, Some(detail.to_string()), None);

        assert_eq!(error::Error::description(&ewd), desc);
        match error::Error::cause(&ewd) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        let _formated_err = format!("{}", ewd);
        let ewod = RustyError::new(desc, None, None);
        let _formated_err = format!("{}", ewod);
    }
}

#[cfg(test)]
mod tests_std_err {
    use std::error::Error;
    use custom_error::pie2error;

    #[test]
    fn test_parse_errors() {
        let nan = "2a".to_string();
        match nan.parse::<u8>().map_err(pie2error) {
            Ok(_) => assert!(false),
            Err(x) => assert_eq!("Integer parsing error", x.description()),
        }
    }
}
