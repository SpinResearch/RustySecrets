//! Defines two different deterministic sharing schemes, ThSS and SS1.
//!
//! # Deterministic secret sharing
//!
//! TODO: Doc
//!
//! # Schemes
//!
//! The two schemes differ by the security properties that they satisfy.
//! The following table summarizes which properties are satisfied by each scheme.
//! The definitions of the properties can be found under the 'Security properties' section.
//!
//! **Scheme / Property** | **Basic** | **Priv1** | **Priv2** | **Auth1** | **Auth2** | **ErrDet** | **Repro** |
//! :--------------------:|:---------:|:---------:|:---------:|:---------:|:---------:|:----------:|:---------:|
//!  **ThSS**             |    Yes    |    Yes    |     No    |    No     |    No     |    Yes     |    No     |
//!  **SS1**              |    Yes    |    Yes    |     Yes   |    Yes    |    Yes    |    Yes     |    Yes    |
//!
//! # Security properties
//!
//! **Property** | **Description**
//! :-----------:|----------------|----------------
//! **Basic**    | Basic correctness: If you attempt to recover a secret from an authorized set of shares that were obtained by sharing out a secret **M** using an access structure **A**, you're sure to get back **A** and **M**.<br> <em>Note: in this implementation **A** is not actually returned, but definitely could.</em>
//! **Priv1**    | Standard privacy notation: When the coins are used by the dealer are uniformly random, unauthorized sets of shares have no computationally extractable information about the underlying secret.
//! **Priv2**    | Privacy for deterministic or hedged schemes: extract whatever entropy one can from the underlying secret. If it’s adequate, no additional randomness is needed in order to achieve a meaningful notion of privacy.
//! **Auth1**    | A share obtained from an honest dealer commits it to a single underlying secret: that and only that value can be recovered.
//! **Auth2**    | A share obtained even from a dishonest dealer commits it to a single underlying secret: that and only that value might be recovered. Implies Auth1.
//! **ErrDet**   | An inauthentic set of shares produced by an adversary will be flagged as such when fed to the recovery algorithm.
//! **Repro**    | Share reproducible: The scheme can produce shares in a deterministic way.

pub mod ss1;
pub mod thss;

mod metadata;

mod format;
mod random;
mod utils;

/// Define the access structure used to deal and recover the shares.
///
/// For example, if one wants to deal 10 shares, and require 7 of them to
/// recover the secret, one would express it as:
///
/// ```rust
/// # use rusty_secrets::dss::AccessStructure;
/// AccessStructure {
///     threshold: 7,
///     shares_count: 10,
/// };
/// ```
#[derive(Copy, Clone, Debug)]
pub struct AccessStructure {
    /// The minimum amount of shares required to recover the secret.
    pub threshold: u8,

    /// The total number of shares generated when splitting up the secret.
    /// Always greater than or equal to `threshold`.
    pub shares_count: u8,
}
