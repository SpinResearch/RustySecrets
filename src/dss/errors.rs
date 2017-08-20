
//! Define the various error kinds specific to deterministic secret sharing.

#![allow(unknown_lints, missing_docs)]

/// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    errors {
        /// k must be smaller than or equal to n
        KMustBeSmallerThanN(k: u8, n: u8) {
            description("k must be smaller than or equal to n")
            display("k must be smaller than or equal to n, got: k = {}, n = {}", k, n)
        }

        MustContainAtLeastOneShare {
            // TODO
        }

        SharesDontConformTo(k: u8, n: u8, m: usize) {
            // TODO
        }

        ShareIdentifierGreaterThanN(id: u8, n: u8) {
            // TODO
        }

        NotEnoughSharesProvided(provided: usize, required: usize) {
            // TODO
        }

        CannotGenerateRandomNumbers {
            description("cannot generate random numbers")
            display("cannot generate random numbers")
        }
    }

    foreign_links {
        Io(::std::io::Error);
    }
}
