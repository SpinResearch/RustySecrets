#![deny(
    //missing_docs,
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

mod gf256;
mod validation;
mod interpolation;
mod share_format;
mod sss;

pub mod custom_error;
pub use sss::generate_shares;
pub use sss::recover_secret;
pub mod share_data;

use ring::digest::{Algorithm, SHA512};
#[allow(non_upper_case_globals)]
static digest: &'static Algorithm = &SHA512;
