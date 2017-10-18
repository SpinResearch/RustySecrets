//! `RustySecrets` implements Shamir's secret sharing in Rust. It provides the possibility to sign shares.

#![deny(
    missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications
)]

extern crate protobuf;
extern crate rustc_serialize as serialize;
extern crate rand;
extern crate merkle_sigs;
extern crate ring;
#[macro_use]
extern crate error_chain;

use ring::digest::{Algorithm, SHA512};
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

mod error;
mod gf256;
mod interpolation;
#[allow(unused_qualifications)]
mod secret;
#[allow(unused_qualifications)]
mod share_data;
mod share_format;
mod validation;

pub mod sss;
pub mod wrapped_secrets;

#[cfg(test)]
mod tests;
