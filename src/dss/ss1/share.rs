
use share::{IsShare, HasMetaData};

pub use share::MetaData;

/// A share identified by an `id`, a threshold `k`, a number of total shares `n`,
/// the `data` held in the share, and the share's `metadata`.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Share {
    /// The identifier of the share (varies between 1 and n where n is the total number of generated shares)
    pub id: u8,
    /// The number of shares necessary to recover the secret, aka a threshold
    pub threshold: u8,
    /// The total number of shares that have been dealt
    pub total_shares_count: u8,
    /// The share data itself
    pub data: Vec<u8>,
    /// The hash value common to the whole deal
    pub hash: Vec<u8>,
    /// The metadata associated with this share
    pub metadata: Option<MetaData>,
}

impl IsShare for Share {
    fn get_id(&self) -> u8 {
        self.id
    }

    fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_threshold(&self) -> u8 {
        self.threshold
    }

    fn get_total_shares_count(&self) -> Option<u8> {
        Some(self.total_shares_count)
    }

    // type Signature = Void;

    // fn is_signed(&self) -> bool {
    //     false
    // }

    // fn signature(&self) -> &Self::Signature {
    //     unimplemented!()
    // }

    // fn verify_signatures(_shares: &[Self]) -> Result<()> {
    //     Ok(())
    // }
}

impl HasMetaData for Share {
    fn get_metadata(&self) -> &Option<MetaData> {
        &self.metadata
    }
}
