
use std;

use errors::*;

use ring::error::Unspecified;
use ring::rand::SecureRandom;

/// TODO: Doc.
pub(crate) fn random_len(k: usize, m: usize) -> usize {
    assert!(k >= 1);
    assert!(m >= 1);

    (k - 1) * m
}

/// TODO: Doc.
pub(crate) fn get_random_bytes(random: &SecureRandom, len: usize) -> Result<Vec<u8>> {
    let mut rl = vec![0; len];

    random.fill(&mut rl).chain_err(|| {
        ErrorKind::CannotGenerateRandomNumbers
    })?;

    Ok(rl)
}

/// TODO: Doc.
pub(crate) struct FixedRandom {
    src: Vec<u8>,
}

impl FixedRandom {
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
