
use gf256::Gf256;

/// Encode the given `secret`.
///
/// TODO: Doc.
pub(crate) fn encode_secret(secret: &[u8], k: u8, share_id: u8, rands: &[u8]) -> Vec<u8> {
    secret
        .into_iter()
        .enumerate()
        .map(|(i, m)| {
            let mut poly = Vec::new();
            for l in 0..(k - 1) as usize {
                poly.push(rands[i * (k as usize - 1) + l]);
            }
            encode_secret_byte(*m, share_id, &poly)
        })
        .collect()
}

/// Encode the given secret byte `m`, by evaluating the given
/// polynomial at x = `j`, and adding the result to `m`.
///
/// TODO: Doc.
pub(crate) fn encode_secret_byte(m: u8, j: u8, poly: &[u8]) -> u8 {
    let mut acc = Gf256::from_byte(m);
    for (l, p) in poly.iter().enumerate() {
        let r = Gf256::from_byte(*p);
        let s = Gf256::from_byte(j).pow(l as u8 + 1);
        acc = acc + r * s;
    }
    acc.to_byte()
}
