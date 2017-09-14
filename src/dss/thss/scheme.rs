
//! Simple threshold secret sharing scheme

use ring::rand::{SecureRandom, SystemRandom};

use errors::*;
use gf256::Gf256;
use dss::random::{random_bytes, random_bytes_count, MAX_MESSAGE_SIZE};
use share::validation::validate_shares;
use lagrange;

use super::share::*;
use super::encode::encode_secret;

/// We bound the message size at about 16MB to avoid overflow in `random_bytes_count`.
/// Moreover, given the current performances, it is almost unpractical to run
/// the sharing scheme on message larger than that.
const MAX_SECRET_SIZE: usize = MAX_MESSAGE_SIZE;

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
        if threshold < MIN_THRESHOLD {
            bail!(ErrorKind::ThresholdTooSmall(threshold));
        }

        if total_shares_count > MAX_SHARES {
            bail!(ErrorKind::InvalidShareCountMax(
                total_shares_count,
                MAX_SHARES,
            ));
        }

        if total_shares_count < MIN_SHARES {
            bail!(ErrorKind::InvalidShareCountMin(
                total_shares_count,
                MIN_SHARES,
            ));
        }

        if threshold > total_shares_count {
            bail!(ErrorKind::ThresholdTooBig(threshold, total_shares_count));
        }

        let secret_len = secret.len();
        if secret_len == 0 {
            bail!(ErrorKind::EmptySecret);
        }
        if secret_len > MAX_SECRET_SIZE {
            bail!(ErrorKind::SecretTooBig(secret_len, MAX_SECRET_SIZE));
        }

        let rands_len = random_bytes_count(threshold, secret_len);
        let rands = random_bytes(self.random.as_ref(), rands_len)?;

        let shares = (1..total_shares_count+1)
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
        let (threshold, shares) = validate_shares(shares.to_vec())?;

        let cypher_len = shares[0].data.len();

        let polys = (0..cypher_len)
            .map(|i| {
                let points = shares
                    .iter()
                    .take(threshold as usize)
                    .map(|share| {
                        (Gf256::from_byte(share.id), Gf256::from_byte(share.data[i]))
                    })
                    .collect::<Vec<_>>();

                lagrange::interpolate(&points)
            })
            .collect::<Vec<_>>();

        for (i, poly) in polys.iter().enumerate() {
            // Check shares for consistency.
            // See Figure 7 of the paper
            for (u, share) in shares.iter()
                                    .enumerate()
                                    .take(shares.len())
                                    .skip(threshold as usize + 1)
            {
                let value = poly.evaluate_at(Gf256::from_byte(u as u8 + 1)).to_byte();
                if value != share.data[i] {
                    bail!(ErrorKind::InconsistentShares);
                }
            }
        }

        let metadata = shares[0].metadata.clone();
        let secret = polys
            .iter()
            .map(|p| p.evaluate_at_zero().to_byte())
            .collect();

        Ok((secret, metadata))
    }
}
