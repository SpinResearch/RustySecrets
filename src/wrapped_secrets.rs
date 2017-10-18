//! (Beta) `wrapped_secrets` provides Shamir's secret sharing with a wrapped secret. It currently offers versioning and MIME information about the data.

use error::Error;
use protobuf;
use protobuf::Message;
use secret::{RustySecret, RustySecretsVersions};
use sss;

/// Performs threshold k-out-of-n Shamir's secret sharing.
///
/// # Examples
///
/// ```
/// use rusty_secrets::wrapped_secrets::generate_shares;
/// let secret = "These programs were never about terrorism: they’re about economic spying,
///               social control, and diplomatic manipulation. They’re about power.".to_string();
///
/// match generate_shares(7, 10, &secret.into_bytes(), Some("text/html".to_string()), true){
/// 	Ok(shares) => {
/// 		// Do something with the shares
/// 	},
/// 	Err(_) => {}// Deal with error}
/// }
/// ```
pub fn generate_shares(k: u8, n: u8, secret: &[u8], mime_type: Option<String>, sign_shares: bool) -> Result<Vec<String>, Error> {
    let mut rusty_secret = RustySecret::new();
    rusty_secret.set_version(RustySecretsVersions::INITIAL_RELEASE);
    rusty_secret.set_secret(secret.to_owned());

    for mt in mime_type {
        rusty_secret.set_mime_type(mt);
    }

    sss::generate_shares(k, n, rusty_secret.write_to_bytes().unwrap().as_slice(), sign_shares)
        .map_err(|e| e.into())
}

/// Recovers the secret from a k-out-of-n Shamir's secret sharing.
///
/// At least `k` distinct shares need to be provided to recover the share.
///
/// # Examples
///
/// ```
/// use rusty_secrets::wrapped_secrets::recover_secret;
/// let share1 = "2-1-Cha7s14Q/mSwWko0ittr+/Uf79RHQMIP".to_string();
/// let share2 = "2-4-ChaydsUJDypD9ZWxwvIICh/cmZvzusOF".to_string();
/// let shares = vec![share1, share2];
///
/// match recover_secret(shares, false) {
/// 	Ok(secret) => {
/// 		// Do something with the secret
/// 	},
/// 	Err(e) => {
/// 		// Deal with the error
/// 	}
/// }
/// ```
pub fn recover_secret(shares: Vec<String>, verify_signatures: bool) -> Result<RustySecret, Error> {
    let secret = sss::recover_secret(shares, verify_signatures)?;

    protobuf::parse_from_bytes::<RustySecret>(secret.as_slice())
        .map_err(|e| e.into())
}
