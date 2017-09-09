
//! Simple threshold secret sharing scheme

use errors::*;

mod math;

mod share;
pub use self::share::*;

mod scheme;
pub(crate) use self::scheme::ThSS;

/// TODO: Doc
pub fn split_secret(
    k: u8,
    n: u8,
    secret: &[u8],
    metadata: &Option<MetaData>,
) -> Result<Vec<Share>> {
    ThSS::default().split_secret(k, n, secret, metadata)
}

/// TODO: Doc
pub fn recover_secret(shares: &[Share]) -> Result<(Vec<u8>, Option<MetaData>)> {
    ThSS::default().recover_secret(shares)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn split_then_recover_yields_original_secret() {
        let secret = "Hello, World!".to_string().into_bytes();

        let shares = split_secret(7, 10, &secret, &None).unwrap();
        let (recovered, metadata) = recover_secret(&shares).unwrap();

        assert_eq!(secret, recovered);
        assert_eq!(None, metadata);
    }

}
