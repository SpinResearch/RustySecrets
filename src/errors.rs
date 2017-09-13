
//! Define the various error kinds specific to deterministic secret sharing.

#![allow(unknown_lints, missing_docs)]

use std::collections::HashSet;

use dss::ss1;

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

        ShareIdentifierTooBig(id: u8, n: u8) {
            description("Share identifier too big")
            display("Found share identifier ({}) bigger than the maximum number of shares ({}).", id, n)
        }

        MissingShares(provided: usize, required: usize) {
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

        ShareParsingError(share_id: u8, data: String) {
            description("This share is incorrectly formatted.")
        }

        InvalidSS1Parameters(r: usize, s: usize) {
            description("Invalid parameters for the SS1 sharing scheme")
            display("Invalid parameters for the SS1 sharing scheme: r = {}, s = {}.", r, s)
        }

        InvalidSplitParametersZero(k: u8, n: u8) {
            description("Parameters k and n must be greater than zero")
            display("Parameters k and n must be greater than zero.")
        }

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

        DuplicateShareData(share_id: u8) {
            description("The data encoded in this share is the same as the one found in a previous share")
            display("The data encoded in share #{} is the same as the one found in a previous share.", share_id)
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
