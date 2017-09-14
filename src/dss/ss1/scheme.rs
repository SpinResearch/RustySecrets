use std::fmt;
use std::collections::HashSet;

use sha3::Shake256;
use digest::{Input, XofReader, ExtendableOutput};
use ring::rand::{SystemRandom, SecureRandom};

use errors::*;
use dss::thss;
use dss::thss::{ThSS, MetaData};
use dss::random::{random_bytes, random_bytes_count, FixedRandom, MAX_MESSAGE_SIZE};
use share::validation::{validate_shares, validate_share_count};
use super::share::*;

/// We bound the message size at about 16MB to avoid overflow in `random_bytes_count`.
/// Moreover, given the current performances, it is almost unpractical to run
/// the sharing scheme on message larger than that.
const MAX_SECRET_SIZE: usize = MAX_MESSAGE_SIZE;

/// Defines a `SS1` deterministic threshold secret sharing scheme.
///
/// This scheme is implemented as the *T2 transform* over the ThSS threshold sharing scheme.
/// found in the `rusty_secrets::dss::thss` module.
pub(crate) struct SS1 {
    /// How many random bytes to read from `random` to use as
    /// padding to the hash function (param `r` from the paper)
    /// and to the message in the underlying ThSS scheme.
    pub random_padding_len: usize,
    /// The length of the hash used for all shares (param `s` from the paper)
    pub hash_len: usize,
    /// The secure random number generator
    random: Box<SecureRandom>,
}

impl fmt::Debug for SS1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SS1 {{ random_padding_len: {}, hash_len: {} }}",
            self.random_padding_len,
            self.hash_len
        )
    }
}

// TODO: Are those good parameters?
// TODO: Add max length ?
static DEFAULT_RANDOM_PADDING_LEN: usize = 512; // r
static MIN_RANDOM_PADDING_LEN: usize = 128; // r min
static DEFAULT_HASH_LEN: usize = 256; // s
static MIN_HASH_LEN: usize = 128; // s min


impl Default for SS1 {
    fn default() -> Self {
        Self::new(
            DEFAULT_RANDOM_PADDING_LEN,
            DEFAULT_HASH_LEN,
            Box::new(SystemRandom::new()),
        ).unwrap()
    }
}

impl SS1 {
    /// Constructs a new sharing scheme
    pub fn new(
        random_padding_len: usize,
        hash_len: usize,
        random: Box<SecureRandom>,
    ) -> Result<Self> {
        if random_padding_len < MIN_RANDOM_PADDING_LEN || hash_len < MIN_HASH_LEN {
            bail!(ErrorKind::InvalidSS1Parameters(
                random_padding_len,
                hash_len,
            ));
        }

        Ok(Self {
            random_padding_len,
            hash_len,
            random,
        })
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
        let (threshold, total_shares_count) = validate_share_count(threshold, total_shares_count)?;
        let secret_len = secret.len();

        if secret_len == 0 {
            bail!(ErrorKind::EmptySecret);
        }
        if secret_len > MAX_SECRET_SIZE {
            bail!(ErrorKind::SecretTooBig(secret_len, MAX_SECRET_SIZE));
        }

        let random_padding = random_bytes(self.random.as_ref(), self.random_padding_len)?;

        let mut shake = Shake256::default();
        shake.process(&[0]);
        shake.process(&[threshold, total_shares_count]);
        shake.process(secret);
        shake.process(&random_padding);

        let seed_len = random_bytes_count(threshold, secret.len() + self.random_padding_len);

        let mut hash = vec![0; self.hash_len];
        let mut seed = vec![0; seed_len];

        let mut reader = shake.xof_result();
        reader.read(&mut hash);
        reader.read(&mut seed);

        let underlying = ThSS::new(Box::new(FixedRandom::new(seed)));

        let message = [secret, &random_padding].concat();
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
        let (_, mut shares) = validate_shares(shares.to_vec())?;

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
        let (mut secret, metadata) = underlying.recover_secret(&underlying_shares)?;
        let secret_len = secret.len() - self.random_padding_len;
        let random_padding = secret.split_off(secret_len);
        // `secret` nows holds the secret

        let sub_random = FixedRandom::new(random_padding.to_vec());
        let sub_scheme = Self::new(self.random_padding_len, self.hash_len, Box::new(sub_random))?;
        let mut test_shares = sub_scheme.split_secret(
            shares[0].threshold,
            shares[0].total_shares_count,
            &secret,
            &metadata,
        )?;

        test_shares.sort_by_key(|share| share.id);

        shares.sort_by_key(|share| share.id);

        let ids = shares.iter().map(|share| share.id).collect::<HashSet<_>>();
        let relevant_test_shares = test_shares.iter().filter(|share| ids.contains(&share.id));
        let matching_shares = shares.iter().zip(relevant_test_shares);

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
}
