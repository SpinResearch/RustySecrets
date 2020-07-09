//! Simple threshold secret sharing scheme

use std::fmt;

use ring::rand::{SecureRandom, SystemRandom};

use crate::dss::random::{random_bytes, random_bytes_count, MAX_MESSAGE_SIZE};
use crate::errors::*;
use crate::gf256::Gf256;
use crate::lagrange;
use crate::share::validation::{validate_share_count, validate_shares};

use super::AccessStructure;
use super::encode::encode_secret;
use super::share::*;

/// We bound the message size at about 16MB to avoid overflow in `random_bytes_count`.
/// Moreover, given the current performances, it is almost unpractical to run
/// the sharing scheme on message larger than that.
const MAX_SECRET_SIZE: usize = MAX_MESSAGE_SIZE;

/// A simple threshold sharing scheme
#[allow(missing_debug_implementations)]
pub(crate) struct ThSS {
    /// The randomness source
    random: Box<dyn SecureRandom>,
}

impl fmt::Debug for ThSS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThSS")
    }
}

impl Default for ThSS {
    fn default() -> Self {
        Self::new(Box::new(SystemRandom::new()))
    }
}

impl ThSS {
    /// Constructs a new sharing scheme
    pub fn new(random: Box<dyn SecureRandom>) -> Self {
        Self { random }
    }

    /// Split a secret following a given sharing `scheme`,
    /// with `threshold` being the number of shares necessary to recover the secret,
    /// and `shares_count` the total number of shares to be dealt.
    pub fn split_secret(
        &self,
        threshold: u8,
        shares_count: u8,
        secret: &[u8],
        metadata: &Option<MetaData>,
    ) -> Result<Vec<Share>> {
        let (threshold, shares_count) = validate_share_count(threshold, shares_count)?;
        let secret_len = secret.len();

        if secret_len == 0 {
            bail!(ErrorKind::EmptySecret);
        }
        if secret_len > MAX_SECRET_SIZE {
            bail!(ErrorKind::SecretTooBig(secret_len, MAX_SECRET_SIZE));
        }

        let rands_len = random_bytes_count(threshold, secret_len);
        let rands = random_bytes(self.random.as_ref(), rands_len)?;

        let shares = (1..shares_count + 1)
            .map(|id| {
                let data = encode_secret(secret, threshold, id, &rands);

                Share {
                    id,
                    threshold,
                    shares_count,
                    data,
                    metadata: metadata.clone(),
                }
            })
            .collect();

        Ok(shares)
    }

    /// Recover the secret from the given set of shares
    pub fn recover_secret(
        &self,
        shares: &[Share],
    ) -> Result<(Vec<u8>, AccessStructure, Option<MetaData>)> {
        let shares = shares.to_vec();
        let (threshold, cypher_len) = validate_shares(&shares)?;

        let polys = (0..cypher_len)
            .map(|i| {
                let points = shares
                    .iter()
                    .take(threshold as usize)
                    .map(|share| (Gf256::from_byte(share.id), Gf256::from_byte(share.data[i])))
                    .collect::<Vec<_>>();

                lagrange::interpolate(&points)
            })
            .collect::<Vec<_>>();

        for (i, poly) in polys.iter().enumerate() {
            // Check remaining shares for consistency.
            // See Figure 7 of the paper
            let remaining_shares = shares.iter().enumerate().skip(threshold as usize);

            for (u, share) in remaining_shares {
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

        let access_structure = AccessStructure {
            threshold: threshold,
            shares_count: shares.first().unwrap().shares_count,
        };

        Ok((secret, access_structure, metadata))
    }
}
