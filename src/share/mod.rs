
//! TODO: Doc

use errors::*;

pub(crate) mod format;
pub(crate) mod validation;

/// TODO: Doc
pub(crate) trait IsShare: Sized {
    /// TODO: Doc
    type Signature;

    /// TODO: Doc
    fn verify_signatures(shares: &[Self]) -> Result<()>;

    /// TODO: Doc
    fn id(&self) -> u8;

    /// TODO: Doc
    fn data(&self) -> &[u8];

    /// TODO: Doc
    fn k(&self) -> u8;

    /// TODO: Doc
    fn n(&self) -> u8;

    /// Returns whether this share is signed or not.
    fn is_signed(&self) -> bool;

    /// TODO: Doc
    fn signature(&self) -> &Self::Signature;
}
