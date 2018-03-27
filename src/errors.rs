//! Define the various error kinds specific to deterministic secret sharing.

#![allow(unknown_lints, missing_docs)]

use std::collections::HashSet;

#[cfg(feature = "dss")]
use dss::ss1;

/// Minimum allowed number of shares (n)
pub(crate) static MIN_SHARES: u8 = 2;
/// Minimum allowed threshold (k)
pub(crate) static MIN_THRESHOLD: u8 = 2;
/// Maximum allowed number of shares (k,n)
pub(crate) static MAX_SHARES: u8 = 255;
/// SSS Shares should be structured as k-n-data hence 3 parts
pub(crate) static SSS_SHARE_PARTS_COUNT: usize = 3;

/// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        ThresholdTooBig(k: u8, n: u8) {
            description("Threshold k must be smaller than or equal to n")
            display("Threshold k must be smaller than or equal to n, got: k = {}, n = {}.", k, n)
        }

        ThresholdTooSmall(k: u8) {
            description("Threshold k must be bigger than or equal to 2")
            display("Threshold k must be bigger than or equal to 2, got: k = {}", k)
        }

        SecretTooBig(len: usize, max: usize) {
            description("The secret is too long")
            display("The secret is too long, maximum allowed size = {} bytes, got {} bytes", max, len)
        }

        InvalidShareCountMax(nb_shares: u8, max: u8) {
            description("Number of shares is too big")
            display("Number of shares must be smaller than or equal {}, got: {} shares.", max, nb_shares)
        }

        InvalidShareCountMin(nb_shares: u8, min: u8) {
            description("Number of shares is too small")
            display("Number of shares must be larger than or equal {}, got: {} shares.", min, nb_shares)
        }

        EmptySecret {
            description("The secret cannot be empty")
            display("The secret cannot be empty")
        }

        EmptyShares {
            description("No shares provided")
            display("No shares were provided.")
        }

        IncompatibleSets(sets: Vec<HashSet<u8>>) {
            description("The shares are incompatible with each other.")
            display("The shares are incompatible with each other.")
        }

        MissingShares(provided: usize, required: u8) {
            description("The number of shares provided is insufficient to recover the secret.")
            display("{} shares are required to recover the secret, found only {}.", required, provided)
        }

        InvalidSignature(share_id: u8, signature: String) {
            description("The signature of this share is not valid.")
        }

        MissingSignature(share_id: u8) {
            description("Signature is missing while shares are required to be signed.")
        }

        SecretDeserializationError {
            description("An issue was encountered deserializing the secret. \
                         Updating to the latest version of RustySecrets might help fix this.")
        }

        ShareParsingError(reason: String) {
            description("This share is incorrectly formatted.")
            display("This share is incorrectly formatted. Reason: {}", reason)
        }

        ShareParsingErrorEmptyShare(share_id: u8) {
            description("This share is empty.")
            display("Found empty share for share identifier ({})", share_id)
        }

        ShareParsingInvalidShareId(share_id: u8) {
            description("Invalid share identifier.")
            display("Found invalid share identifier ({})", share_id)
        }

        ShareParsingInvalidShareThreshold(k: u8, id: u8) {
            description("Threshold k must be bigger than or equal to 2")
            display("Threshold k must be bigger than or equal to 2. Got k = {} for share identifier {}.", k, id)
        }

        InvalidSS1Parameters(r: usize, s: usize) {
            description("Invalid parameters for the SS1 sharing scheme")
            display("Invalid parameters for the SS1 sharing scheme: r = {}, s = {}.", r, s)
        }

        InvalidSplitParametersZero(k: u8, n: u8) {
            description("Parameters k and n must be greater than zero")
            display("Parameters k and n must be greater than zero.")
        }

        #[cfg(feature = "dss")]
        MismatchingShares(got: ss1::Share, expected: ss1::Share) {
            description("Share mismatch during verification of secret recovery")
            display("Share mismatch during verification of secret recovery.")
        }

        CannotGenerateRandomNumbers {
            description("Cannot generate random numbers")
            display("Cannot generate random numbers.")
        }

        DuplicateShareId(share_id: u8) {
            description("This share number has already been used by a previous share.")
            display("This share number ({}) has already been used by a previous share.", share_id)
        }

        InconsistentShares {
            description("The shares are inconsistent")
            display("The shares are inconsistent")
        }
    }

    foreign_links {
        Io(::std::io::Error);
        IntegerParsingError(::std::num::ParseIntError);
    }
}
