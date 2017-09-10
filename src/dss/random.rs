
use std;

use errors::*;

use ring::error::Unspecified;
use ring::rand::SecureRandom;

/// Returns the number of random bytes to read from the secure random number generator.
/// As defined in section 3.1 of the 'New Directions in Secret Sharing' paper.
pub(crate) fn random_bytes_count(threshold: usize, message_size: usize) -> usize {
    assert!(threshold >= 1);
    assert!(message_size >= 1);

    // TODO: How about overflow?
    (threshold - 1) * message_size
}

/// Attempts to read a prefix of length `len` from the given secure random generator.
pub(crate) fn random_bytes(random: &SecureRandom, len: usize) -> Result<Vec<u8>> {
    assert!(len > 0);

    let mut rl = vec![0; len];

    random.fill(&mut rl).chain_err(|| {
        ErrorKind::CannotGenerateRandomNumbers
    })?;

    Ok(rl)
}

/// An implementation of SecureRandom that fills the output slice with the slice in `src`.
/// The length of `src` must be larger than any slice that we attempt to fill.
pub(crate) struct FixedRandom {
    src: Vec<u8>,
}

impl FixedRandom {
    /// Create a new fixed random generator.
    /// The length of `src` must be larger than any slice that we attempt to fill.
    pub(crate) fn new(src: Vec<u8>) -> Self {
        if src.is_empty() {
            panic!("The source slice of FixedRandom cannot be empty!");
        }
        FixedRandom { src }
    }
}

impl SecureRandom for FixedRandom {
    fn fill(&self, dst: &mut [u8]) -> std::result::Result<(), Unspecified> {
        if dst.len() > self.src.len() {
            return Err(Unspecified);
        }

        let len = dst.len();
        dst.copy_from_slice(&self.src[0..len]);
        Ok(())
    }
}
