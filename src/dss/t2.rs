
//! Deterministic threshold secret sharing scheme

use std::collections::HashSet;

use dss::errors::*;
use dss::thss;
use dss::thss::MetaData;
use dss::random::{get_random_bytes, random_len, FixedRandom};

use sha3::Shake256;
use digest::{Input, XofReader, ExtendableOutput};
use ring::rand::{SystemRandom, SecureRandom};

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
    /// The hash value common to the whole deal
    pub hash: Vec<u8>,
    /// TODO
    pub metadata: Option<MetaData>,
}

/// Deterministic threshold sharing scheme
/// TODO: Figure out a way to get rid of the type parameter (with impl trait maybe?)
#[allow(missing_debug_implementations)]
#[derive(Debug, Clone, Copy)]
pub struct SharingScheme<R: SecureRandom> {
    /// How many random bytes to read from the underlying entropy source
    pub r: usize,
    /// TODO
    pub s: usize,
    /// The entropy source to use to generate random bytes
    random: R,
}

impl Default for SharingScheme<SystemRandom> {
    fn default() -> Self {
        SharingScheme::new(256, 256, SystemRandom::new()).unwrap()
    }
}

impl<R: SecureRandom> SharingScheme<R> {
    /// Constructs a new sharing scheme
    pub fn new(r: usize, s: usize, random: R) -> Result<Self> {
        if r < 128 || s < 128 {
            bail!(ErrorKind::InvalidT2Parameters(r, s));
        }

        Ok(SharingScheme { r, s, random })
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

        let rand = get_random_bytes(&self.random, self.r)?;

        let mut shake = Shake256::default();
        shake.process(&[0]);
        shake.process(&[k, n]);
        shake.process(secret);
        shake.process(&rand);

        let seed_len = random_len(k as usize, secret.len() + self.r);

        let mut hash = vec![0; self.s];
        let mut seed = vec![0; seed_len];

        let mut reader = shake.xof_result();
        reader.read(&mut hash);
        reader.read(&mut seed);

        let underlying = thss::SharingScheme::new(FixedRandom::new(&seed));

        let message = [secret, &rand].concat();
        let shares = underlying.split_secret(k, n, &message, metadata)?;

        let res = shares
            .into_iter()
            .map(|share| {
                Share {
                    id: share.id,
                    k: share.k,
                    n: share.n,
                    data: share.data,
                    hash: hash.clone(),
                    metadata: share.metadata.clone(),
                }
            })
            .collect();

        Ok(res)
    }

    /// Recover the secret from the given set of shares
    pub fn recover_secret(&self, shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
        self.check_shares(shares)?;

        let underlying_shares = shares
            .into_iter()
            .map(|share| {
                thss::Share {
                    id: share.id,
                    k: share.k,
                    n: share.n,
                    data: share.data.clone(),
                    metadata: share.metadata.clone(),
                }
            })
            .collect::<Vec<_>>();

        let underlying = thss::SharingScheme::default();
        let recovered = underlying.recover_secret(&underlying_shares)?;
        let (mut secret, metadata) = recovered;
        let secret_len = secret.len() - self.r;
        let rand = secret.split_off(secret_len);

        let sub_scheme = SharingScheme::new(self.r, self.s, FixedRandom::new(&rand))?;
        let test_shares = sub_scheme.split_secret(
            shares[0].k,
            shares[0].n,
            &secret,
            &metadata,
        )?;

        let ids = shares.iter().map(|share| share.id).collect::<HashSet<_>>();

        // TODO: Sort shares by id before zipping
        // .sort_by_key(|share| share.id)
        let matching_shares = shares.iter().zip(test_shares.iter().filter(|share| {
            ids.contains(&share.id)
        }));

        for (share, test_share) in matching_shares {
            if share != test_share {
                bail!(ErrorKind::MismatchingShares(
                    share.clone(),
                    test_share.clone(),
                ));
            }
        }

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
        let h = &shares[0].hash;

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

            if &share.hash != h {
                bail!(ErrorKind::CorruptedShare(share.id));
            }
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

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = scheme.recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
