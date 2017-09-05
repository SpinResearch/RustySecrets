
//! Deterministic threshold secret sharing scheme

use std::collections::HashSet;

use sha3::Shake256;
use digest::{Input, XofReader, ExtendableOutput};
use ring::rand::{SystemRandom, SecureRandom};

use errors::*;
use dss::thss;
use dss::thss::{ThSS, MetaData};
use dss::random::{get_random_bytes, random_len, FixedRandom};
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
    T2::default().split_secret(k, n, secret, metadata)
}

/// TODO: Doc
pub fn recover_secret(shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
    T2::default().recover_secret(shares)
}

/// An implementation of the T2 transform over a threshold secret sharing scheme,
/// as described in the 'New Directions in Secret Sharing' paper.
#[allow(missing_debug_implementations)]
pub(crate) struct T2 {
    /// How many random bytes to read from the underlying entropy source
    pub r: usize,
    ///
    pub s: usize,
    /// The entropy source to use to generate random bytes
    random: Box<SecureRandom>,
}

impl Default for T2 {
    fn default() -> Self {
        Self::new(256, 256, Box::new(SystemRandom::new())).unwrap()
    }
}

impl T2 {
    /// Constructs a new sharing scheme
    pub fn new(r: usize, s: usize, random: Box<SecureRandom>) -> Result<Self> {
        if r < 128 || s < 128 {
            bail!(ErrorKind::InvalidT2Parameters(r, s));
        }

        Ok(Self { r, s, random })
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

        let rand = get_random_bytes(self.random.as_ref(), self.r)?;

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

        let underlying_random = FixedRandom::new(seed);
        let underlying = ThSS::new(Box::new(underlying_random));

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
        let (_, shares) = validate_shares(shares.to_vec(), true)?;

        let underlying_shares = shares
            .iter()
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

        let underlying = ThSS::default();
        let recovered = underlying.recover_secret(&underlying_shares)?;
        let (mut secret, metadata) = recovered;
        let secret_len = secret.len() - self.r;
        let rand = secret.split_off(secret_len);

        let sub_random = FixedRandom::new(rand.to_vec());
        let sub_scheme = Self::new(self.r, self.s, Box::new(sub_random))?;
        let mut test_shares = sub_scheme.split_secret(
            shares[0].k,
            shares[0].n,
            &secret,
            &metadata,
        )?;

        test_shares.sort_by_key(|share| share.id);

        let mut sorted_shares = shares.to_vec();
        sorted_shares.sort_by_key(|share| share.id);

        let ids = shares.iter().map(|share| share.id).collect::<HashSet<_>>();
        let relevant_test_shares = test_shares.iter().filter(|share| ids.contains(&share.id));
        let matching_shares = sorted_shares.iter().zip(relevant_test_shares);

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

    // FIXME: Is CorruptedShare needed?
    // fn check_shares(&self, shares: &[Share]) -> Result<()> {
    //     let k = shares[0].k;
    //     let n = shares[0].n;
    //     let m = shares[0].data.len();
    //     let h = &shares[0].hash;

    //     if &share.hash != h {
    //         bail!(ErrorKind::CorruptedShare(share.id));
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, &None).unwrap();

        assert_eq!(shares.len(), 10);

        let (recovered, metadata) = recover_secret(&shares[2..9]).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
