// #![deny(
//     missing_docs,
//     missing_debug_implementations, missing_copy_implementations,
//     trivial_casts, trivial_numeric_casts,
//     unsafe_code, unstable_features,
//     unused_import_braces, unused_qualifications
// )]

//! `RustySecrets` implements Shamir Secret Sharing in Rust. It provides the possibility to sign shares.

extern crate protobuf;
extern crate rustc_serialize as serialize;
extern crate rand;
extern crate merkle_sigs;
extern crate ring;

use ring::digest::{Algorithm, SHA512};
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

mod custom_error;
mod gf256;
mod interpolation;
mod share_data;
mod share_format;
mod sss;
mod validation;

pub use sss::generate_shares;
pub use sss::recover_secret;
pub use custom_error::RustyError;

#[cfg(test)]
mod tests;
