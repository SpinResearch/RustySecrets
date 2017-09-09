
//! SSS provides Shamir's secret sharing with raw data.

use digest;
use rand::{OsRng, Rng};
use merkle_sigs::sign_data_vec;

use errors::*;
use interpolation::{encode, lagrange_interpolate};
use sss::format::format_share_for_signing;
use sss::Share;
use share::validation::validate_shares;

/// SSS provides Shamir's secret sharing with raw data.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SSS;

impl SSS {
    /// Performs threshold k-out-of-n Shamir's secret sharing.
    pub fn generate_shares(
        &self,
        k: u8,
        n: u8,
        secret: &[u8],
        sign_shares: bool,
    ) -> Result<Vec<Share>> {
        if k > n {
            bail!(ErrorKind::InvalidThreshold(k, n));
        }

        let shares = self.secret_share(secret, k, n)?;

        let signatures = if sign_shares {
            let shares_to_sign = shares
                .iter()
                .enumerate()
                .map(|(i, x)| format_share_for_signing(k, (i + 1) as u8, x))
                .collect::<Vec<_>>();

            let sign = sign_data_vec(&shares_to_sign, digest)
                .unwrap()
                .into_iter()
                .map(Some)
                .collect::<Vec<_>>();

            Some(sign)
        } else {
            None
        };

        let sig_pairs = signatures
            .unwrap_or_else(|| vec![None; n as usize])
            .into_iter()
            .map(|sig_pair| sig_pair.map(From::from));

        let shares_and_sigs = shares.into_iter().enumerate().zip(sig_pairs);

        let result = shares_and_sigs.map(|((index, data), signature_pair)| {
            // This is actually safe since we alwaays generate less than 256 shares.
            let id = (index + 1) as u8;

            Share {
                id,
                k,
                n,
                data,
                signature_pair,
            }
        });

        Ok(result.collect())
    }

    fn secret_share(&self, src: &[u8], k: u8, n: u8) -> Result<Vec<Vec<u8>>> {
        let mut result = Vec::with_capacity(n as usize);
        for _ in 0..(n as usize) {
            result.push(vec![0u8; src.len()]);
        }
        let mut col_in = vec![0u8, k];
        let mut col_out = Vec::with_capacity(n as usize);
        let mut osrng = OsRng::new()?;
        for (c, &s) in src.iter().enumerate() {
            col_in[0] = s;
            osrng.fill_bytes(&mut col_in[1..]);
            col_out.clear();
            encode(&*col_in, n, &mut col_out)?;
            for (&y, share) in col_out.iter().zip(result.iter_mut()) {
                share[c] = y;
            }
        }
        Ok(result)
    }


    /// Recovers the secret from a k-out-of-n Shamir's secret sharing.
    ///
    /// At least `k` distinct shares need to be provided to recover the share.
    pub fn recover_secret(shares: Vec<Share>, verify_signatures: bool) -> Result<Vec<u8>> {
        let (k, shares) = validate_shares(shares, verify_signatures)?;

        let slen = shares[0].data.len();
        let mut col_in = Vec::with_capacity(k as usize);
        let mut secret = Vec::with_capacity(slen);
        for byteindex in 0..slen {
            col_in.clear();
            for s in shares.iter().take(k as usize) {
                col_in.push((s.n, s.data[byteindex]));
            }
            secret.push(lagrange_interpolate(&*col_in));
        }

        Ok(secret)
    }
}
