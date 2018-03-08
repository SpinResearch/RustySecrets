//! Implements the `SS1` deterministic threshold secret sharing scheme.
//!
//! This scheme is implemented as the *T2 transform* over the ThSS threshold sharing scheme.
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
//! **Repro**    | Yes | Share reproducible: The scheme can produce shares in a deterministic way.
//!
//! # References
//!
//! - *New Directions in Secret Sharing* (TODO: Full reference)

use errors::*;

mod serialize;

mod share;
pub use self::share::*;

mod scheme;
pub use self::scheme::Reproducibility;
use self::scheme::SS1;

/// Performs threshold k-out-of-n deterministic secret sharing.
///
/// # Examples
///
/// ```
/// use rusty_secrets::dss::ss1::{self, Reproducibility, MetaData};
///
/// let secret = "These programs were never about terrorism: they’re about economic spying, \
///               social control, and diplomatic manipulation. They’re about power.";
///
/// let mut metadata = MetaData::new();
/// metadata.tags.insert("mime_type".to_string(), "text/plain".to_string());
///
/// match ss1::split_secret(7, 10, &secret.as_bytes(), Reproducibility::reproducible(), &Some(metadata)) {
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
    reproducibility: Reproducibility,
    metadata: &Option<MetaData>,
) -> Result<Vec<Share>> {
    SS1::default().split_secret(k, n, secret, reproducibility, metadata)
}

/// Recovers the secret from a k-out-of-n deterministic secret sharing scheme (`SS1`).
///
/// At least `k` distinct shares need to be provided to recover the secret.
///
/// # Examples
///
/// ```rust
/// use rusty_secrets::dss::ss1::{self, Reproducibility, MetaData};
///
/// let secret = "These programs were never about terrorism: they’re about economic spying, \
///               social control, and diplomatic manipulation. They’re about power.";
///
/// let mut metadata = MetaData::new();
/// metadata.tags.insert("mime_type".to_string(), "text/plain".to_string());
///
/// let shares = ss1::split_secret(
///     7,
///     10,
///     &secret.as_bytes(),
///     Reproducibility::reproducible(),
///     &Some(metadata)
/// ).unwrap();
///
/// match ss1::recover_secret(&shares) {
///     Ok((secret, metadata)) => {
///         // Do something with the secret and the metadata
///     },
///     Err(e) => {
///         // Deal with the error
///     }
/// }
/// ```
pub fn recover_secret(shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
    SS1::default().recover_secret(shares)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn nonreproducible_split_then_recover_yields_original_secret() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, Reproducibility::none(), &None).unwrap();

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

    #[test]
    fn reproducible_split_then_recover_yields_original_secret() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, Reproducibility::reproducible(), &None).unwrap();

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

    #[test]
    fn seeded_reproducible_split_then_recover_yields_original_secret() {
        let secret = "Hello, World!".to_string().into_bytes();

        let seed = vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16u8];
        let shares = split_secret(7, 10, &secret, Reproducibility::seeded(seed), &None).unwrap();

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

    #[test]
    fn reproducible_split() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares_1 =
            split_secret(7, 10, &secret, Reproducibility::reproducible(), &None).unwrap();
        let shares_2 =
            split_secret(7, 10, &secret, Reproducibility::reproducible(), &None).unwrap();

        assert_eq!(shares_1, shares_2);
    }

    #[test]
    fn nonreproducible_split() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares_1 = split_secret(7, 10, &secret, Reproducibility::none(), &None).unwrap();
        let shares_2 = split_secret(7, 10, &secret, Reproducibility::none(), &None).unwrap();

        assert!(shares_1 != shares_2);
    }

    #[test]
    fn seeded_split() {
        let secret = "Hello, World!".to_string().into_bytes();

        let seed = vec![1, 2, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16u8];
        let shares_1 =
            split_secret(7, 10, &secret, Reproducibility::seeded(seed.clone()), &None).unwrap();
        let shares_2 =
            split_secret(7, 10, &secret, Reproducibility::seeded(seed.clone()), &None).unwrap();

        assert_eq!(shares_1, shares_2);
    }

}
