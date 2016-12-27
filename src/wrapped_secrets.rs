use custom_error::{RustyError, RustyErrorTypes};
use protobuf;
use protobuf::Message;
use secret::{RustySecret, RustySecretsVersions};
use sss;
use std::io;

pub fn generate_shares(k: u8, n: u8, secret: &[u8], mime_type: &str, sign_shares: bool) -> io::Result<Vec<String>> {
    let mut rusty_secret = RustySecret::new();
    rusty_secret.set_version(RustySecretsVersions::INITIAL_RELEASE);
    rusty_secret.set_mime_type(mime_type.to_owned());
    rusty_secret.set_secret(secret.to_owned());

    sss::generate_shares(k, n, rusty_secret.write_to_bytes().unwrap().as_slice(), sign_shares)
}

pub fn recover_secret(shares: Vec<String>, verify_signatures: bool) -> Result<RustySecret, RustyError> {
    let secret = try!(sss::recover_secret(shares, verify_signatures));

    protobuf::parse_from_bytes::<RustySecret>(secret.as_slice())
    .map_err(|_| RustyError::with_type(RustyErrorTypes::SecretDeserializationIssue))
}
