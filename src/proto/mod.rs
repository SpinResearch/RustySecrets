//! Protocol buffer definitions.

#[allow(unused_qualifications, deprecated, missing_docs)]
pub mod wrapped;

#[cfg(feature = "dss")]
#[allow(unused_qualifications, deprecated, missing_docs)]
pub mod dss;

#[allow(unused_qualifications, deprecated, missing_docs)]
mod version;

pub use self::version::VersionProto;
