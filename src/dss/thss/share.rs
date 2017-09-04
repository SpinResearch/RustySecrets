
use std::collections::BTreeMap;

use errors::*;
use share::IsShare;
use void::Void;

/// A share's public metadata.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MetaData {
    /// The tags associated with the share
    pub tags: BTreeMap<String, Vec<u8>>,
}

impl MetaData {
    /// Construct a new MetaData struct.
    pub fn new(tags: BTreeMap<String, Vec<u8>>) -> Self {
        MetaData { tags }
    }
}

/// A share identified by an `id`, a threshold `k`, a number of total shares `n`,
/// the `data` held in the share, and the share's `metadata`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Share {
    /// The identifier of the share (varies between 1 and n where n is the total number of generated shares)
    pub id: u8,
    /// The number of shares necessary to recover the secret, aka a threshold
    pub k: u8,
    /// The total number of shares that have been dealt
    pub n: u8,
    /// The share data itself
    pub data: Vec<u8>,
    /// The metadata associated with this share
    pub metadata: Option<MetaData>,
}

impl IsShare for Share {
    type Signature = Void;

    fn verify_signatures(_shares: &[Self]) -> Result<()> {
        Ok(())
    }

    fn id(&self) -> u8 {
        self.id
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn k(&self) -> u8 {
        self.k
    }

    fn n(&self) -> u8 {
        self.n
    }

    fn is_signed(&self) -> bool {
        false
    }

    fn signature(&self) -> &Self::Signature {
        unimplemented!()
    }
}
