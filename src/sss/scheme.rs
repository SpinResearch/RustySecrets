//! Provides Shamir's secret sharing with raw data.

use std::cmp::min;

use merkle_sigs::sign_data_vec;
use rand::{OsRng, Rng};

use errors::*;
use gf256::Gf256;
use lagrange::{evaluate_at_zero, BarycentricWeights};
use share::IsShare;
use share::validation::*;
use sss::format::format_share_for_signing;
use sss::{Share, HASH_ALGO};

use super::encode::encode_secret_byte;

/// Performs threshold k-out-of-n Shamir's secret sharing.
pub(crate) fn split_secret(
    threshold: u8,
    shares_count: u8,
    secret: &[u8],
    sign_shares: bool,
) -> Result<Vec<Share>> {
    let (threshold, shares_count) = validate_share_count(threshold, shares_count)?;
    let shares = secret_share(secret, threshold, shares_count)?;

    let signatures = if sign_shares {
        let shares_to_sign = shares
            .iter()
            .enumerate()
            .map(|(i, x)| format_share_for_signing(threshold, (i + 1) as u8, x))
            .collect::<Vec<_>>();

        let sign = sign_data_vec(&shares_to_sign, HASH_ALGO)
            .unwrap()
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>();

        Some(sign)
    } else {
        None
    };

    let sig_pairs = signatures
        .unwrap_or_else(|| vec![None; shares_count as usize])
        .into_iter()
        .map(|sig_pair| sig_pair.map(From::from));

    let shares_and_sigs = shares.into_iter().enumerate().zip(sig_pairs);

    let result = shares_and_sigs.map(|((index, data), signature_pair)| {
        // This is actually safe since we alwaays generate less than 256 shares.
        let id = (index + 1) as u8;

        Share {
            id,
            threshold,
            data,
            signature_pair,
        }
    });

    Ok(result.collect())
}

fn secret_share(src: &[u8], threshold: u8, shares_count: u8) -> Result<Vec<Vec<u8>>> {
    let mut result = Vec::with_capacity(shares_count as usize);
    for _ in 0..(shares_count as usize) {
        result.push(vec![0u8; src.len()]);
    }
    let mut col_in = vec![0u8; threshold as usize];
    let mut col_out = Vec::with_capacity(shares_count as usize);
    let mut osrng = OsRng::new()?;
    for (c, &s) in src.iter().enumerate() {
        col_in[0] = s;
        // NOTE: switch to `try_fill_bytes` when it lands in a stable release:
        // https://github.com/rust-lang-nursery/rand/commit/230b2258dbd99ff8bd991008c972d923d4b5d10c
        osrng.fill_bytes(&mut col_in[1..]);
        col_out.clear();
        encode_secret_byte(&*col_in, shares_count, &mut col_out)?;
        for (&y, share) in col_out.iter().zip(result.iter_mut()) {
            share[c] = y;
        }
    }
    Ok(result)
}

/// `Recover` provides an interface for recovering a secret.
pub(crate) struct Recover {
    /// The state of each partially-recovered secret byte.
    barycentric: Vec<BarycentricWeights>,
    /// The ids of the share (varies between 1 and n where n is the total number of generated
    /// shares).
    ids: Vec<Gf256>,
    /// The number of shares necessary to recover the secret.
    threshold: u8,
    /// The length of the secret.
    slen: usize,
    /// If the shares are signed, the root hash of the Merkle tree all shares are signed with.
    root_hash: Option<Vec<u8>>,
    /// The secret. `None` until computation is complete.
    secret: Option<Vec<u8>>,
}

impl Recover {
    /// Recovers the secret from a k-out-of-n Shamir's secret sharing.
    ///
    /// At least `k` distinct shares need to be provided to recover the share.
    pub fn recover_secret(shares: &[Share], verify_signatures: bool) -> Result<Vec<u8>> {
        let recovery = Self::new(shares, verify_signatures)?;
        recovery.get_secret()
    }

    /// Begins a partial secret recovery.
    pub fn new(shares: &[Share], verify_signatures: bool) -> Result<Self> {
        let (threshold, slen, root_hash) =
            validate_initial_signed_shares(shares, verify_signatures)?;

        let mut incremental_recovery = Self {
            barycentric: Vec::with_capacity(slen),
            ids: Vec::with_capacity(threshold as usize),
            threshold,
            slen,
            root_hash,
            secret: None,
        };

        incremental_recovery.process_shares(shares);

        Ok(incremental_recovery)
    }

    /// Contines a partial secret recovery.
    pub fn update(&mut self, shares: &[Share]) -> Result<()> {
        if self.shares_needed() == 0 {
            bail!(ErrorKind::NoMoreSharesNeeded(self.threshold))
        }

        let ids: Vec<u8> = self.ids.iter().map(|id| id.poly).collect();
        validate_additional_signed_shares(
            shares,
            Some(self.threshold),
            Some(self.slen),
            Some(&ids),
            Some(&self.root_hash.clone().unwrap()),
        )?;

        self.process_shares(shares);
        Ok(())
    }

    fn process_shares(&mut self, shares: &[Share]) {
        // Don't bother interpolating more than k shares.
        let ub = min(self.shares_needed() as usize, shares.len());
        let shares = &shares[..ub];
        let is_new_computation = self.shares_interpolated() == 0;
        // NOTE: `shares_interpolated` will be very temporarily be out of sync until the next for
        // loop completes.
        self.ids.extend(shares.iter().map(|s| gf256!(s.id)));

        for byteindex in 0..self.slen {
            let ys: Vec<Gf256> = shares.iter().map(|s| gf256!(s.data[byteindex])).collect();
            if is_new_computation {
                self.barycentric
                    .push(BarycentricWeights::new(&self.ids, &ys));
            } else {
                self.barycentric[byteindex].update(&self.ids, &ys);
            }
        }

        // If we have sufficient information, we can compute the secret.
        if self.shares_needed() == 0 {
            self.compute_secret();
        }
    }

    /// Computes the secret (called automatically when sufficient shares have been processed).
    fn compute_secret(&mut self) {
        self.secret = Some(
            self.barycentric
                .iter()
                .map(|wds| evaluate_at_zero(wds, &self.ids))
                .collect(),
        );
    }

    /// Used to determine how many more shares are needed to finish computing a partial secret.
    pub fn shares_interpolated(&self) -> u8 {
        // Safe cast because validation ensures `self.ids.len() < 255`.
        self.ids.len() as u8
    }

    /// Returns the threshold of shares needed to recover the secret.
    pub fn get_threshold(&self) -> u8 {
        self.threshold
    }

    /// Used to determine how many more shares are needed to finish computing a partial secret.
    pub fn shares_needed(&self) -> u8 {
        // Safe unsigned subraction because `process_shares` ensures we never interpolate more than
        // `threshold`.
        self.threshold - self.shares_interpolated()
    }

    /// Used to obtain the resulting secret when `threshold` shares have been evaluated.
    pub fn get_secret(&self) -> Result<Vec<u8>> {
        if self.secret.is_some() {
            Ok(self.secret.clone().unwrap())
        } else {
            bail!(ErrorKind::MissingShares(
                self.shares_interpolated(),
                self.threshold
            ))
        }
    }
}
