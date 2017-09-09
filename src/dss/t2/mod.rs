
//! Defines a deterministic threshold secret sharing scheme, implemented
//! as a transform over the simple threshold sharing scheme found
//! in the `dss::thss` module.
//!
/// ### References
/// - *New Directions in Secret Sharing* (TODO: Full reference)

use errors::*;
use dss::thss::MetaData;

mod share;
pub use self::share::*;

mod scheme;
use self::scheme::T2;

/// Performs threshold k-out-of-n deterministic secret sharing.
///
/// # Examples
///
/// ```
/// use rusty_secrets::dss::t2::split_secret;
///
/// let secret = "These programs were never about terrorism: they’re about economic spying, \
///               social control, and diplomatic manipulation. They’re about power.";
///
/// match split_secret(7, 10, &secret.as_bytes(), &None) {
///     Ok(shares) => {
///         // Do something with the shares
///     },
///     Err(_) => {
///         // Deal with error
///     }
/// }
/// ```
pub fn split_secret(
    k: u8,
    n: u8,
    secret: &[u8],
    metadata: &Option<MetaData>,
) -> Result<Vec<Share>> {
    T2::default().split_secret(k, n, secret, metadata)
}

/// Recovers the secret from a k-out-of-n deterministic secret sharing scheme.
///
/// At least `k` distinct shares need to be provided to recover the share.
///
/// # Examples
///
/// TODO
pub fn recover_secret(shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
    T2::default().recover_secret(shares)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, &None).unwrap();

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
