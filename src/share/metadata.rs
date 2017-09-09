
use std::collections::BTreeMap;

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
