
//! Simple thresold secret sharing scheme

use dss::errors::*;

use gf256::Gf256;
use interpolation::lagrange_interpolate;

use ring::rand::SecureRandom;

/// A simple thresold sharing scheme
/// TODO: Figure out a way to get rid of the type parameter (with impl trait maybe?)
#[allow(missing_debug_implementations)]
pub struct SharingScheme<R: SecureRandom> {
    /// The number of shares necessary to recover the secret
    pub k: u8,
    /// The total number of shares to be dealt
    pub n: u8,
    /// The randomness source
    pub random: R,
}

impl<R: SecureRandom> SharingScheme<R> {
    /// Constructs a new sharing scheme
    pub fn new(k: u8, n: u8, random: R) -> Self {
        SharingScheme { k, n, random }
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
}

/// Split a secret following a given sharing scheme
pub fn generate_shares<R>(scheme: &SharingScheme<R>, secret: &[u8]) -> Result<Vec<Share>>
where
    R: SecureRandom,
{
    let k = scheme.k;
    let n = scheme.n;

    if k > n {
        bail!(ErrorKind::KMustBeSmallerThanN(k, n));
    }

    let m = secret.len();

    let rands = get_random_bytes(&scheme.random, k as usize, m)?;

    let shares = (0..n)
        .map(|id| {
            let data = (0..m)
                .map(|i| {
                    // FIXME: Extract into its own function
                    let mut f = Gf256::from_byte(secret[i]);
                    for l in 0..(k - 1) as usize {
                        let r = Gf256::from_byte(rands[i * (k as usize - 1) + l]);
                        let s = Gf256::from_byte(id).pow(l as u8 + 1);
                        f = f + r * s;
                    }
                    f.to_byte()
                })
                .collect();

            Share { id, k, n, data }
        })
        .collect();

    Ok(shares)
}

/// Recover the secret from the given set of shares
pub fn recover_secret(shares: &[Share]) -> Result<Vec<u8>> {
    check_shares(shares)?;

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

    Ok(secret)
}

// FIXME: assert -> bail
fn check_shares(shares: &[Share]) -> Result<()> {
    if shares.is_empty() {
        bail!(ErrorKind::MustContainAtLeastOneShare);
    }

    let k = shares[0].k;
    let n = shares[0].n;
    let m = shares[0].data.len();

    if k > n {
        bail!(ErrorKind::KMustBeSmallerThanN(k, n));
    }

    for share in shares {
        if k != share.k || n != share.n || m != share.data.len() {
            bail!(ErrorKind::SharesDontConformTo(k, n, m));
        }

        if share.id >= n {
            bail!(ErrorKind::ShareIdentifierGreaterThanN(share.id, n));
        }
    }

    if shares.len() < k as usize {
        bail!(ErrorKind::NotEnoughSharesProvided(
            shares.len(),
            shares[0].k as usize,
        ));
    }

    Ok(())
}

fn get_random_bytes<R: SecureRandom>(random: &R, k: usize, m: usize) -> Result<Vec<u8>> {
    let mut rl = vec![0; (k - 1) * m];

    random.fill(&mut rl).chain_err(|| {
        ErrorKind::CannotGenerateRandomNumbers
    })?;

    Ok(rl)
}

#[cfg(test)]
mod tests {

    use super::*;

    use ring::rand::SystemRandom;

    #[test]
    fn it_works() {
        let secret = "Hello, World!".to_string().into_bytes();

        let scheme = SharingScheme::new(7, 10, SystemRandom::new());
        let shares = generate_shares(&scheme, &secret).unwrap();
        let recovered = recover_secret(&shares).unwrap();

        assert_eq!(secret, recovered);
    }

}
