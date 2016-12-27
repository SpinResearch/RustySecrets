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
    share_groups: Option<Vec<Vec<u8>>>
}

pub enum RustyErrorTypes {
    DuplicateShareNum(u8),
    DuplicateShareData(u8),
    EmptyShares,
    IncompatibleSets(Vec<Vec<u8>>),
    InvalidSignature(u8, String),
    MissingShares(u8, usize),
    MissingSignature(u8),
    SecretDeserializationIssue,
    ShareParsingError(u8, String)
}

impl RustyError {
    /// Initializes a new error with a description and optional detail string.
    fn new(descr: &'static str, detail: Option<String>, share_num: Option<u8>, share_groups: Option<Vec<Vec<u8>>>) -> RustyError {
        RustyError {
            descr: descr,
            detail: detail,
            share_num: share_num,
            share_groups: share_groups
        }
    }

    pub fn with_type(error_type: RustyErrorTypes) -> RustyError {
        RustyError {
            descr: RustyError::descr_for_type(&error_type),
            detail: RustyError::detail_for_type(&error_type),
            share_num: RustyError::share_num_for_type(&error_type),
            share_groups: RustyError::share_groups_for_type(error_type),
        }
    }

    pub fn share_num(&self) -> Option<u8> {
        self.share_num
    }

    pub fn share_groups(&self) -> Option<Vec<Vec<u8>>> {
        self.share_groups.clone()
    }

    fn descr_for_type(error_type: &RustyErrorTypes) -> &'static str {
        match *error_type {
            RustyErrorTypes::EmptyShares => "No shares were provided.",
            RustyErrorTypes::IncompatibleSets(_) => "The shares are incompatible with each other.",
            RustyErrorTypes::InvalidSignature(_, _) => "The signature of this share is not valid.",
            RustyErrorTypes::MissingShares(_, _) => "The number of shares provided is insufficient to recover the secret.",
            RustyErrorTypes::MissingSignature(_) => "Signature is missing while shares are required to be signed.",
            RustyErrorTypes::SecretDeserializationIssue => "An issue was encountered deserializing the secret.
            Updating to the latest version of RustySecrets might help fix this.",
            RustyErrorTypes::ShareParsingError(_, _) => "This share is incorrectly formatted.",
            RustyErrorTypes::DuplicateShareNum(_) => "This share number has already been used by a previous share.",
            RustyErrorTypes::DuplicateShareData(_) => "The data encoded in this share is the same as the one found in a previous share."
        }
    }

    fn detail_for_type(error_type: &RustyErrorTypes) -> Option<String> {
        match *error_type {
            RustyErrorTypes::MissingShares(required, found) => Some(format!("{} shares are required to recover the secret,
                                                                   found only {}.", required, found)),
            RustyErrorTypes::ShareParsingError(_, ref description) => Some(description.clone()),
            _ => None
        }
    }

    fn share_groups_for_type(error_type: RustyErrorTypes) -> Option<Vec<Vec<u8>>>{
        match error_type {
            RustyErrorTypes::IncompatibleSets(groups) => Some(groups),
            _ => None
        }
    }

    fn share_num_for_type(error_type: &RustyErrorTypes) -> Option<u8> {
        match *error_type {
            RustyErrorTypes::InvalidSignature(share_num, _)
            | RustyErrorTypes::MissingSignature(share_num)
            | RustyErrorTypes::ShareParsingError(share_num, _)
            | RustyErrorTypes::DuplicateShareNum(share_num)
            | RustyErrorTypes::DuplicateShareData(share_num) => Some(share_num),
            _ => None
        }
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

        RustyError::new("from io:Error", Some(descr), None, None)
    }
}

impl From<RustyError> for io::Error {
    fn from(me: RustyError) -> io::Error {
        io::Error::new(io::ErrorKind::Other, me)
    }
}

/// Returns an `io::Error` from description string and optional detail string.
/// Particularly useful in `Result` expressions.
pub fn other_io_err(descr: &'static str, detail: Option<String>,
                    share_num: Option<u8>, share_groups: Option<Vec<Vec<u8>>>) -> io::Error {
    convert::From::from(RustyError::new(descr, detail, share_num, share_groups))
}

/// maps a `ParseIntError` to an `Error`
pub fn pie2error(p: num::ParseIntError) -> RustyError {
    RustyError::new("Integer parsing error", Some(p.to_string()), None, None)
}

#[cfg(test)]
mod tests_custom_err {
    use std::error;
    use custom_error::RustyError;

    #[test]
    fn test_custom_error() {
        let desc = "Boring error description";
        let detail = "More of it";
        let ewd = RustyError::new(desc, Some(detail.to_string()), None, None);

        assert_eq!(error::Error::description(&ewd), desc);
        match error::Error::cause(&ewd) {
            Some(_) => assert!(false),
            None => assert!(true),
        }
        let _formated_err = format!("{}", ewd);
        let ewod = RustyError::new(desc, None, None, None);
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
