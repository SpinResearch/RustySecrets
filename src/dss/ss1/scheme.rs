use std::fmt;
use std::collections::HashSet;

use sha3::Shake256;
use digest::{Input, XofReader, ExtendableOutput};
use ring::rand::{SystemRandom, SecureRandom};
use ring::digest::{Context, SHA256};
use rand::{ChaChaRng, Rng, SeedableRng};

use errors::*;
use dss::thss;
use dss::thss::{ThSS, MetaData};
use dss::random::{random_bytes_count, FixedRandom, MAX_MESSAGE_SIZE};
use share::validation::{validate_shares, validate_share_count};
use super::share::*;
use super::utils;

/// We bound the message size at about 16MB to avoid overflow in `random_bytes_count`.
/// Moreover, given the current performances, it is almost unpractical to run
/// the sharing scheme on message larger than that.
const MAX_SECRET_SIZE: usize = MAX_MESSAGE_SIZE;

const DEFAULT_PRESEED: &[u8] = b"rusty_secrets::dss::ss1";

/// There are situations where it's useful to generate shares in a reproducible manner.
/// In particular, this allows a secret that’s in someone’s head, a passphrase,
/// to be shared out in a manner in which different shares can be given to
/// different people at different points in time.
///
/// On the other hand, there is some privacy cost.
/// For example, if you know the secret is one of two possibilities,
/// M0 or M1, in a share-reproducible scheme, acquiring a single share
/// will probably let you decide which of the two possibilities it was.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reproducibility {
    /// Shares will be produced in a deterministic way, using
    /// a default, fixed seed for the internal random number generator
    /// used to generate entropy.
    Reproducible,
    /// Shares will be produced in a deterministic way, using
    /// the given seed for the internal random number generator used to
    /// generate entropy.
    Seeded(Vec<u8>),
    /// Shares will be produced in a deterministic way, using
    /// the given byte vector as the entropy source.
    WithEntropy(Vec<u8>),
    /// Shares will be produced in non-deterministic way, using
    /// the system's random number generator to produce entropy.
    None,
}

impl Reproducibility {
    /// Shares will be produced in a deterministic way, using
    /// a default, fixed seed for the internal random number generator
    /// used to generate entropy.
    pub fn reproducible() -> Self {
        Reproducibility::Reproducible
    }

    /// Shares will be produced in a deterministic way, using
    /// the given seed for the internal random number generator used to
    /// generate entropy.
    pub fn seeded(seed: Vec<u8>) -> Self {
        assert!(!seed.is_empty(), "Reproducibility: seed cannot be empty");
        Reproducibility::Seeded(seed)
    }

    /// Shares will be produced in a deterministic way, using
    /// the given byte vector as the entropy source.
    pub fn with_entropy(entropy: Vec<u8>) -> Self {
        assert!(
            !entropy.is_empty(),
            "Reproducibility: entropy cannot be empty"
        );
        Reproducibility::WithEntropy(entropy)
    }

    /// Shares will be produced in non-deterministic way, using
    /// the system's random number generator to produce entropy.
    pub fn none() -> Self {
        Reproducibility::None
    }
}

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
        Self::new(DEFAULT_RANDOM_PADDING_LEN, DEFAULT_HASH_LEN).unwrap()
    }
}

impl SS1 {
    /// Constructs a new sharing scheme
    pub fn new(random_padding_len: usize, hash_len: usize) -> Result<Self> {
        if random_padding_len < MIN_RANDOM_PADDING_LEN || hash_len < MIN_HASH_LEN {
            bail!(ErrorKind::InvalidSS1Parameters(
                random_padding_len,
                hash_len,
            ));
        }

        Ok(Self {
            random_padding_len,
            hash_len,
        })
    }

    /// Split a secret following a given sharing `scheme`,
    /// with `threshold` being the number of shares necessary to recover the secret,
    /// and `shares_count` the total number of shares to be dealt.
    pub fn split_secret(
        &self,
        threshold: u8,
        shares_count: u8,
        secret: &[u8],
        reproducibility: Reproducibility,
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

        let random_padding = self.generate_random_padding(
            reproducibility,
            secret,
            metadata,
        )?;

        let mut shake = Shake256::default();
        shake.process(&[0]);
        shake.process(&[threshold, shares_count]);
        shake.process(secret);
        shake.process(&random_padding);

        let mut hash = vec![0; self.hash_len];

        let randomness_len = random_bytes_count(threshold, secret.len() + self.random_padding_len);

        let mut randomness = vec![0; randomness_len];

        let mut reader = shake.xof_result();
        reader.read(&mut hash);
        reader.read(&mut randomness);

        let underlying = ThSS::new(Box::new(FixedRandom::new(randomness)));

        let message = [secret, &random_padding].concat();
        let shares = underlying.split_secret(
            threshold,
            shares_count,
            &message,
            metadata,
        )?;

        let res = shares
            .into_iter()
            .map(|share| {
                Share {
                    id: share.id,
                    threshold: share.threshold,
                    shares_count: share.shares_count,
                    data: share.data,
                    hash: hash.clone(),
                    metadata: share.metadata.clone(),
                }
            })
            .collect();

        Ok(res)
    }

    fn generate_random_padding(
        &self,
        reproducibility: Reproducibility,
        secret: &[u8],
        metadata: &Option<MetaData>,
    ) -> Result<Vec<u8>> {
        match reproducibility {
            Reproducibility::None => {
                let rng = SystemRandom::new();
                let mut result = vec![0u8; self.random_padding_len];
                rng.fill(&mut result).chain_err(|| {
                    ErrorKind::CannotGenerateRandomNumbers
                })?;
                Ok(result)
            }
            Reproducibility::Reproducible => {
                let mut ctx = Context::new(&SHA256);
                ctx.update(DEFAULT_PRESEED);
                ctx.update(secret);
                for md in metadata {
                    md.hash_into(&mut ctx);
                }

                // We can safely call `utils::slice_u8_to_slice_u32` because
                // the `digest` produced with `SHA256` is 256 bits long and
                // can thus be represented both a slice of 32 bytes or as
                // a slice of 8 32-bit words.
                let digest = ctx.finish();
                let seed = utils::slice_u8_to_slice_u32(digest.as_ref());

                let mut rng = ChaChaRng::from_seed(seed);
                let mut result = vec![0u8; self.random_padding_len];
                rng.fill_bytes(result.as_mut_slice());
                Ok(result)
            }
            Reproducibility::Seeded(preseed) => {
                let mut ctx = Context::new(&SHA256);
                ctx.update(preseed.as_slice());
                ctx.update(secret);
                for md in metadata {
                    md.hash_into(&mut ctx);
                }

                // We can safely call `utils::slice_u8_to_slice_u32` because
                // the `digest` produced with `SHA256` is 256 bits long and
                // can thus be represented both a slice of 32 bytes or as
                // a slice of 8 32-bit words.
                let digest = ctx.finish();
                let seed = utils::slice_u8_to_slice_u32(digest.as_ref());

                let mut rng = ChaChaRng::from_seed(&seed);
                let mut result = vec![0u8; self.random_padding_len];
                rng.fill_bytes(result.as_mut_slice());
                Ok(result)
            }
            Reproducibility::WithEntropy(entropy) => Ok(entropy),
        }
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
                    shares_count: share.shares_count,
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

        let sub_scheme = Self::new(self.random_padding_len, self.hash_len)?;

        let mut test_shares = sub_scheme.split_secret(
            shares[0].threshold,
            shares[0].shares_count,
            &secret,
            Reproducibility::WithEntropy(
                random_padding.to_vec(),
            ),
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
