
//! Define the various error kinds specific to deterministic secret sharing.

#![allow(unknown_lints, missing_docs)]

use dss::t2;

/// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        /// k must be smaller than or equal to n
        InvalidSplitParametersSmaller(k: u8, n: u8) {
            description("The parameter k must be smaller than or equal to n")
            display("The parameter k must be smaller than or equal to n, got: k = {}, n = {}", k, n)
        }

        EmptyShares {
            description("No shares were provided")
            display("No shares were provided")
        }

        IncompatibleSets {
            description("The shares are incompatible with each other.")
            display("The shares are incompatible with each other.")
        }

        ShareIdentifierTooBig(id: u8, n: u8) {
            description("Share identifier too big")
            display("Found share identifier ({}) bigger than the maximum number of shares ({})", id, n)
        }

        MissingShares(provided: usize, required: usize) {
            description("The number of shares provided is insufficient to recover the secret.")
            display("The number of shares provided is insufficient to recover the secret. Provided: {}, required: {}.", provided, required)
        }

        InvalidT2Parameters(r: usize, s: usize) {
            description("invalid parameters for the T2 sharing scheme")
            display("invalid parameters for the T2 sharing scheme: r = {}, s = {}", r, s)
        }

        InvalidSplitParametersZero(k: u8, n: u8) {
            description("k and n must be greater than zero")
            display("k and n must be greater than zero")
        }

        MismatchingShares(got: t2::Share, expected: t2::Share) {
            description("Share mismatch during verification of secret recovery")
            display("Share mismatch during verification of secret recovery")
        }

        CannotGenerateRandomNumbers {
            description("Cannot generate random numbers")
            display("Cannot generate random numbers")
        }

        DuplicateShareId(share_id: u8) {
            description("This share number has already been used by a previous share.")
            display("This share number ({}) has already been used by a previous share.", share_id)
        }

        DuplicateShareData(share_id: u8) {
            description("the data encoded in this share is the same as the one found in a previous share")
            display("the data encoded in share #{} is the same as the one found in a previous share", share_id)
        }

        CorruptedShare(share_id: u8) {
            description("A share is corrupted")
            display("Share #{} is corrupted.", share_id)
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
