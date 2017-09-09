
use std::collections::BTreeMap;

/// A share's public metadata.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MetaData {
    /// The tags associated with the share
    pub tags: BTreeMap<String, String>,
}

impl MetaData {
    /// Construct a new MetaData struct.
    pub fn new() -> Self {
        MetaData { tags: BTreeMap::new() }
    }

    /// Construct a new MetaData struct, holding the given tags
    pub fn with_tags(tags: BTreeMap<String, String>) -> Self {
        Self { tags }
    }
}
