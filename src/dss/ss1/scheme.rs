
use std::collections::HashSet;

use sha3::Shake256;
use digest::{Input, XofReader, ExtendableOutput};
use ring::rand::{SystemRandom, SecureRandom};

use errors::*;
use dss::thss;
use dss::thss::{ThSS, MetaData};
use dss::random::{random_bytes, random_bytes_count, FixedRandom};
use share::validation::validate_shares;
use super::share::*;

#[allow(missing_debug_implementations)]
pub(crate) struct SS1 {
    /// How many random bytes to read from the underlying entropy source
    pub r: usize,
    /// TODO
    pub s: usize,
    /// The entropy source to use to generate random bytes
    random: Box<SecureRandom>,
}

// TODO: Are those good parameters?
static DEFAULT_R: usize = 256;
static DEFAULT_S: usize = 256;

impl Default for SS1 {
    fn default() -> Self {
        Self::new(DEFAULT_R, DEFAULT_S, Box::new(SystemRandom::new())).unwrap()
    }
}

impl SS1 {
    /// Constructs a new sharing scheme
    pub fn new(r: usize, s: usize, random: Box<SecureRandom>) -> Result<Self> {
        if r < 128 || s < 128 {
            bail!(ErrorKind::InvalidSS1Parameters(r, s));
        }

        Ok(Self { r, s, random })
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

        let rand = random_bytes(self.random.as_ref(), self.r)?;

        let mut shake = Shake256::default();
        shake.process(&[0]);
        shake.process(&[threshold, total_shares_count]);
        shake.process(secret);
        shake.process(&rand);

        let seed_len = random_bytes_count(threshold as usize, secret.len() + self.r);

        let mut hash = vec![0; self.s];
        let mut seed = vec![0; seed_len];

        let mut reader = shake.xof_result();
        reader.read(&mut hash);
        reader.read(&mut seed);

        let underlying_random = FixedRandom::new(seed);
        let underlying = ThSS::new(Box::new(underlying_random));

        let message = [secret, &rand].concat();
        let shares = underlying.split_secret(
            threshold,
            total_shares_count,
            &message,
            metadata,
        )?;

        let res = shares
            .into_iter()
            .map(|share| {
                Share {
                    id: share.id,
                    threshold: share.threshold,
                    total_shares_count: share.total_shares_count,
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
        let (_, shares) = validate_shares(shares.to_vec())?;

        let underlying_shares = shares
            .iter()
            .map(|share| {
                thss::Share {
                    id: share.id,
                    threshold: share.threshold,
                    total_shares_count: share.total_shares_count,
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
            shares[0].threshold,
            shares[0].total_shares_count,
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
