
//! Implements the `T2` deterministic threshold secret sharing scheme.
//!
//! This scheme is implemted as the *T2 transform* over the simple *threshold sharing scheme (ThSS)*
//! found in the `rusty_secrets::dss::thss` module.
//!
//! # Security properties
//!
//! This scheme satisfies the following security properties:
//!
//! **Property** | **Satisifed?** | **Description**
//! -------------|----------------|----------------
//! **Basic**    | Yes | Basic correctness: If you attempt to recover a secret from an authorized set of shares that were obtained by sharing out a secret **M** using an access structure **A**, you're sure to get back **A** and **M**.<br> <em>Note: in this implementation **A** is not actually returned, but definitely could.</em>
//! **Priv1**    | Yes | Standard privacy notation: When the coins are used by the dealer are uniformly random, unauthorized sets of shares have no computationally extractable information about the underlying secret.
//! **Priv2**    | Yes | Privacy for deterministic or hedged schemes: extract whatever entropy one can from the underlying secret. If it’s adequate, no additional randomness is needed in order to achieve a meaningful notion of privacy.
//! **Auth1**    | Yes | A share obtained from an honest dealer commits it to a single underlying secret: that and only that value can be recovered.
//! **Auth2**    | Yes | A share obtained even from a dishonest dealer commits it to a single underlying secret: that and only that value might be recovered. Implies Auth1.
//! **ErrDet**   | Yes | An inauthentic set of shares produced by an adversary will be flagged as such when fed to the recovery algorithm.
//! **Repro**    | Yes | Share reproducible: TODO
//!
//! # References
//!
//! - *New Directions in Secret Sharing* (TODO: Full reference)

use errors::*;
pub use dss::thss::MetaData;

mod share;
pub use self::share::*;

mod scheme;
use self::scheme::T2;

/// Performs threshold k-out-of-n deterministic secret sharing.
///
/// # Examples
///
/// ```
/// use rusty_secrets::dss::t2;
///
/// let secret = "These programs were never about terrorism: they’re about economic spying, \
///               social control, and diplomatic manipulation. They’re about power.";
///
/// let mut metadata = t2::MetaData::new();
/// metadata.tags.insert("mime_type".to_string(), "text/plain".to_string());
///
/// match t2::split_secret(7, 10, &secret.as_bytes(), &Some(metadata)) {
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

/// Recovers the secret from a k-out-of-n deterministic secret sharing scheme (`T2`).
///
/// At least `k` distinct shares need to be provided to recover the secret.
///
/// # Examples
///
/// ```rust
/// use rusty_secrets::dss::t2;
///
/// let secret = "These programs were never about terrorism: they’re about economic spying, \
///               social control, and diplomatic manipulation. They’re about power.";
///
/// let mut metadata = t2::MetaData::new();
/// metadata.tags.insert("mime_type".to_string(), "text/plain".to_string());
///
/// let shares = t2::split_secret(
///     7,
///     10,
///     &secret.as_bytes(),
///     &Some(metadata)
/// ).unwrap();
///
/// match t2::recover_secret(&shares) {
///     Ok((secret, metadata)) => {
///         // Do something with the secret and the metadata
///     },
///     Err(e) => {
///         // Deal with the error
///     }
/// }
/// ```
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
