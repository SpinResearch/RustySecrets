//! Define the traits used to distinguish the various
//! kind of shares internally used by the library.
//! These traits are currently not exposed, but this might
//! change in the future.

use errors::*;

pub(crate) mod validation;

/// All types of share should implement this trait.
pub(crate) trait IsShare: Sized {
    /// Returns the identifier of the share.
    /// Varies between 1 and n where n is the total number of generated shares.
    fn get_id(&self) -> u8;

    /// Returns the share data itself
    fn get_data(&self) -> &[u8];

    /// Returns the number of shares necessary to recover the secret, aka the threshold
    fn get_threshold(&self) -> u8;

    /// Returns the total number of shares that have been dealt
    fn get_shares_count(&self) -> Option<u8>;
}

/// This trait must be implemented by shares' types wich can be signed.
pub(crate) trait IsSignedShare: IsShare {
    /// The type of shares' sigature.
    type Signature;

    /// Returns whether this share is signed or not.
    fn is_signed(&self) -> bool;

    /// Return the signature itself.
    fn get_signature(&self) -> &Self::Signature;

    /// Verify a given batch of shares are all signed by the same root hash. Returns the root hash
    /// if verification succeeds, and an `Err` otherwise.
    fn verify_signatures(shares: &[Self]) -> Result<Vec<u8>>;

    /// Verify the `shares` all have valid signatures from the `root_hash`. Pass a list of shares
    /// identifiers already verified against this `root_hash`, if any, for better error messaging
    /// if verification fails.
    fn continue_verify_signatures(
        shares: &[Self],
        root_hash: &Vec<u8>,
        already_verified_ids: &Vec<u8>,
    ) -> Result<()>;
}
