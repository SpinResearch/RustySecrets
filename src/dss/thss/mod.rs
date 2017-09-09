
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
    /// with `threshold` being the number of shares necessary to recover the secret,
    /// and `total_shares_count` the total number of shares to be dealt.
    pub fn split_secret(
        &self,
        threshold: u8,
        total_shares_count: u8,
        secret: &[u8],
        metadata: &Option<MetaData>,
    ) -> Result<Vec<Share>> {
        if threshold < 1 || total_shares_count < 1 {
            bail!(ErrorKind::InvalidSplitParametersZero(
                threshold,
                total_shares_count,
            ));
        }

        if threshold > total_shares_count {
            bail!(ErrorKind::InvalidThreshold(threshold, total_shares_count));
        }

        let rands_len = random_len(threshold as usize, secret.len());
        let rands = get_random_bytes(self.random.as_ref(), rands_len)?;

        let shares = (0..total_shares_count)
            .map(|id| {
                let data = encode_secret(secret, threshold, id, &rands);

                Share {
                    id,
                    threshold,
                    total_shares_count,
                    data,
                    metadata: metadata.clone(),
                }
            })
            .collect();

        Ok(shares)
    }

    /// Recover the secret from the given set of shares
    pub fn recover_secret(&self, shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
        let (_, shares) = validate_shares(shares.to_vec())?;

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
