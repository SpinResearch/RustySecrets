
use std::collections::BTreeMap;
use ring::digest;

/// A share's public metadata.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Default)]
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

    pub(crate) fn hash_into(&self, ctx: &mut digest::Context) {
        for (tag, value) in &self.tags {
            ctx.update(tag.as_bytes());
            ctx.update(b":");
            ctx.update(value.as_bytes());
        }
    }
}
