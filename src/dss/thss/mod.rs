
//! Simple threshold secret sharing scheme

use ring::rand::{SecureRandom, SystemRandom};

use errors::*;
use dss::random::{get_random_bytes, random_len};
use interpolation::{lagrange_interpolate, encode_secret};
use share::validation::validate_shares;

mod share;
pub use self::share::*;

/// TODO: Doc
pub fn split_secret(
    k: u8,
    n: u8,
    secret: &[u8],
    metadata: &Option<MetaData>,
) -> Result<Vec<Share>> {
    ThSS::default().split_secret(k, n, secret, metadata)
}

/// TODO: Doc
pub fn recover_secret(shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
    ThSS::default().recover_secret(shares)
}

/// A simple threshold sharing scheme
#[allow(missing_debug_implementations)]
pub(crate) struct ThSS {
    /// The randomness source
    pub random: Box<SecureRandom>,
}

impl Default for ThSS {
    fn default() -> Self {
        Self::new(Box::new(SystemRandom::new()))
    }
}

impl ThSS {
    /// Constructs a new sharing scheme
    pub fn new(random: Box<SecureRandom>) -> Self {
        Self { random }
    }

    /// Split a secret following a given sharing `scheme`,
    /// with `k` being the number of shares necessary to recover the secret,
    /// and `n` the total number of shares to be dealt.
    pub fn split_secret(
        &self,
        k: u8,
        n: u8,
        secret: &[u8],
        metadata: &Option<MetaData>,
    ) -> Result<Vec<Share>> {
        if k < 1 || n < 1 {
            bail!(ErrorKind::InvalidSplitParametersZero(k, n));
        }

        if k > n {
            bail!(ErrorKind::InvalidThreshold(k, n));
        }

        let rands_len = random_len(k as usize, secret.len());
        let rands = get_random_bytes(self.random.as_ref(), rands_len)?;

        let shares = (0..n)
            .map(|id| {
                let data = encode_secret(secret, k, id, &rands);

                Share {
                    id,
                    k,
                    n,
                    data,
                    metadata: metadata.clone(),
                }
            })
            .collect();

        Ok(shares)
    }

    /// Recover the secret from the given set of shares
    pub fn recover_secret(&self, shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
        let (_, shares) = validate_shares(shares.to_vec(), true)?;

        // FIXME: Check that the data length is the same for all shares.
        let m = shares[0].data.len();

        let secret = (0..m)
            .map(|i| {
                let points = shares
                    .iter()
                    .map(|share| (share.id, share.data[i]))
                    .collect::<Vec<_>>();

                lagrange_interpolate(&points)
            })
            .collect();

        let metadata = shares[0].metadata.clone();

        Ok((secret, metadata))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn split_then_recover_yields_original_secret() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, &None).unwrap();
        let (recovered, metadata) = recover_secret(&shares).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
