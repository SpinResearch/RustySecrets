use gf256::Gf256;
use poly::Poly;

/// Encode the given `secret` using the `ThSS[N].Share` algorithm described
/// in the *New directions in Secret Sharing* paper.
///
/// Reference: Figure 7 from the *New Directions in Secret Sharing* paper.
pub(crate) fn encode_secret(secret: &[u8], k: u8, share_id: u8, rands: &[u8]) -> Vec<u8> {
    secret
        .into_iter()
        .enumerate()
        .map(|(i, m)| {
            let k_pred = (k - 1) as usize;
            let coeffs = (0..k_pred)
                .map(|l| {
                    let n = rands[i * k_pred + l];
                    Gf256::from_byte(n)
                })
                .collect();
            let poly = Poly::new(coeffs);
            encode_secret_byte(*m, share_id, &poly)
        })
        .collect()
}

/// Encode the given secret byte `m`, by evaluating the given
/// polynomial at x = `j`, and adding the result to `m`.
///
/// Reference: Figure 7 from the *New Directions in Secret Sharing* paper.
pub(crate) fn encode_secret_byte(m: u8, j: u8, poly: &Poly) -> u8 {
    let mut acc = Gf256::from_byte(m);
    for (l, &r) in poly.coeffs.iter().enumerate() {
        let s = Gf256::from_byte(j).pow(l as u8 + 1);
        acc = acc + r * s;
    }
    acc.to_byte()
}
