//! `RustySecrets` implements Shamir's secret sharing in Rust. It provides the possibility to sign shares.

#![deny(
    missing_docs,
    missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code, unstable_features,
    unused_import_braces, unused_qualifications
)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

extern crate protobuf;
extern crate rustc_serialize as serialize;
extern crate rand;
extern crate merkle_sigs;
extern crate ring;
extern crate sha3;
extern crate digest;

use ring::digest::{Algorithm, SHA512};
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;

#[macro_use]
mod gf256;
mod share;
mod poly;
mod lagrange;

pub mod errors;
pub mod sss;
pub mod wrapped_secrets;
pub mod dss;
pub mod proto;

#[cfg(test)]
extern crate itertools;

#[cfg(test)]
extern crate flate2;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
