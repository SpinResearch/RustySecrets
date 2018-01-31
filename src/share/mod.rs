//! Define the traits used to distinguish the various
//! kind of shares internally used by the library.
//! These traits are currently not exposed, but this might
//! change in the future.

use errors::*;

pub(crate) mod metadata;
pub(crate) mod validation;

pub use self::metadata::MetaData;

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

    /// Verify the signatures of the given batch of shares.
    /// Returns `Ok(())` if validation succeeds, and an `Err` otherwise.
    fn verify_signatures(shares: &[Self]) -> Result<()>;
}

/// This trait must be implemented by types of share which can hold additional metadata.
pub(crate) trait HasMetaData: IsShare {
    /// Return the metadata associated with the share, if any.
    fn get_metadata(&self) -> &Option<MetaData>;
}
