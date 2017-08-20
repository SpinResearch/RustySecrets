
use std;

use dss::errors::*;

use ring::error::Unspecified;
use ring::rand::SecureRandom;

pub(crate) fn random_len(k: usize, m: usize) -> usize {
    assert!(k >= 1);
    assert!(m >= 1);

    (k - 1) * m
}

pub(crate) fn get_random_bytes<R: SecureRandom>(random: &R, len: usize) -> Result<Vec<u8>> {
    let mut rl = vec![0; len];

    random.fill(&mut rl).chain_err(|| {
        ErrorKind::CannotGenerateRandomNumbers
    })?;

    Ok(rl)
}

pub(crate) struct FixedRandom<'a> {
    src: &'a [u8],
}

impl<'a> FixedRandom<'a> {
    /// TODO
    pub(crate) fn new(src: &'a [u8]) -> Self {
        FixedRandom { src }
    }
}

impl<'a> SecureRandom for FixedRandom<'a> {
    fn fill(&self, dst: &mut [u8]) -> std::result::Result<(), Unspecified> {
        if dst.len() > self.src.len() {
            return Err(Unspecified);
        }

        let len = dst.len();
        dst.copy_from_slice(&self.src[0..len]);
        Ok(())
    }
}
