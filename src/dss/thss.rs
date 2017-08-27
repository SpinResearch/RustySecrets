
//! Simple threshold secret sharing scheme

use std::collections::{HashSet, BTreeMap};

use dss::errors::*;
use dss::random::{get_random_bytes, random_len};
use interpolation::{lagrange_interpolate, evaluate};

use ring::rand::{SecureRandom, SystemRandom};

/// TODO
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct MetaData {
    /// TODO
    pub tags: BTreeMap<String, Vec<u8>>,
}

impl MetaData {
    /// TODO
    pub fn new(tags: BTreeMap<String, Vec<u8>>) -> Self {
        MetaData { tags }
    }
}

/// A share
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Share {
    /// The identifier of the share (varies between 1 and n where n is the total number of generated shares)
    pub id: u8,
    /// The number of shares necessary to recover the secret
    pub k: u8,
    /// The total number of shares that have been dealt
    pub n: u8,
    /// The share data itself
    pub data: Vec<u8>,
    /// Metadata associated with this share
    pub metadata: Option<MetaData>,
}

/// A simple threshold sharing scheme
/// TODO: Figure out a way to get rid of the type parameter (with impl trait maybe?)
#[allow(missing_debug_implementations)]
pub struct SharingScheme<R: SecureRandom> {
    /// The randomness source
    pub random: R,
}

impl Default for SharingScheme<SystemRandom> {
    fn default() -> Self {
        SharingScheme::new(SystemRandom::new())
    }
}

impl<R: SecureRandom> SharingScheme<R> {
    /// Constructs a new sharing scheme
    pub fn new(random: R) -> Self {
        SharingScheme { random }
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
            bail!(ErrorKind::InvalidSplitParametersSmaller(k, n));
        }

        let m = secret.len();

        let rands = get_random_bytes(&self.random, random_len(k as usize, m))?;

        let shares = (0..n)
            .map(|j| {
                let data = (0..m)
                    .map(|i| {
                        // TODO: Document and extract
                        let mut poly = Vec::new();
                        for l in 0..(k - 1) as usize {
                            poly.push(rands[i * (k as usize - 1) + l]);
                        }
                        evaluate(secret[i], k, j, &poly)
                    })
                    .collect();

                Share {
                    id: j,
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
        self.check_shares(shares)?;

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

    // TODO: Deduplicate this function
    fn check_shares(&self, shares: &[Share]) -> Result<()> {
        if shares.is_empty() {
            bail!(ErrorKind::EmptyShares);
        }

        let k = shares[0].k;
        let n = shares[0].n;
        let m = shares[0].data.len();

        if k > n {
            bail!(ErrorKind::InvalidSplitParametersSmaller(k, n));
        }

        let mut id_seen: HashSet<u8> = HashSet::new();
        let mut data_seen: HashSet<&Vec<u8>> = HashSet::new();

        for share in shares {
            if k != share.k || n != share.n || m != share.data.len() {
                bail!(ErrorKind::IncompatibleSets);
            }

            if share.id >= n {
                bail!(ErrorKind::ShareIdentifierTooBig(share.id, n));
            }

            if id_seen.contains(&share.id) {
                bail!(ErrorKind::DuplicateShareId(share.id));
            }

            id_seen.insert(share.id);

            if data_seen.contains(&share.data) {
                bail!(ErrorKind::DuplicateShareData(share.id));
            }

            data_seen.insert(&share.data);
        }

        if shares.len() < k as usize {
            bail!(ErrorKind::MissingShares(shares.len(), shares[0].k as usize));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let secret = "Hello, World!".to_string().into_bytes();

        let scheme = SharingScheme::default();
        let shares = scheme.split_secret(7, 10, &secret, &None).unwrap();
        let (recovered, metadata) = scheme.recover_secret(&shares).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
