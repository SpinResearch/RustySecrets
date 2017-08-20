
//! Deterministic secret sharing

#[allow(unknown_lints)]
mod errors {

    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        errors {
            KMustBeSmallerThanN(k: u8, n: u8) {
                description("k must be smaller than or equal to n")
                display("k must be smaller than or equal to n, got: k = {}, n = {}", k, n)
            }

            CannotGenerateRandomNumbers {
                description("cannot generate random numbers")
                display("cannot generate random numbers")
            }
        }

        foreign_links {
            Io(::std::io::Error);
        }
    }

}

use self::errors::*;

use gf256::Gf256;
use interpolation::lagrange_interpolate;

use ring::rand::{SystemRandom, SecureRandom};

/// A share
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Share {
    /// The identifier of the share (varies between 1 and n where n is the total number of generated shares)
    pub id: u8,
    /// The number of shares necessary to recover the secret
    pub k: u8,
    /// The share data itself
    pub data: Vec<u8>,
}

/// Split the `secret` into `n` shares, with `k`-out-of-`n` shares required to recover it
pub fn generate_shares(k: u8, n: u8, secret: &[u8]) -> Result<Vec<Share>> {
    if k > n {
        bail!(ErrorKind::KMustBeSmallerThanN(k, n));
    }

    let m = secret.len();
    let rands = rand(k as usize, m)?;

    let shares = (0..n)
        .map(|id| {
            let data = (0..m)
                .map(|i| {
                    let mut f = Gf256::from_byte(secret[i]);
                    for l in 0..(k - 1) as usize {
                        let r = Gf256::from_byte(rands[i * (k as usize - 1) + l]);
                        let s = Gf256::from_byte(id).pow(l as u8 + 1);
                        f = f + r * s;
                    }
                    f.to_byte()
                })
                .collect();

            Share { id, k, data }
        })
        .collect();

    Ok(shares)
}

/// Recover the secret from the given set of shares
pub fn recover_secret(shares: &[Share]) -> Result<Vec<u8>> {
    unimplemented!()
}

fn rand(k: usize, m: usize) -> Result<Vec<u8>> {
    let mut rl = vec![0; (k - 1) * m];

    SystemRandom::new().fill(&mut rl).chain_err(|| {
        ErrorKind::CannotGenerateRandomNumbers
    })?;

    Ok(rl)
}
