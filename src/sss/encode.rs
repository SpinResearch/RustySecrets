
use gf256::Gf256;
use std::io;
use std::io::prelude::*;

/// evaluates a polynomial at x=1, 2, 3, ... n (inclusive)
pub(crate) fn encode_secret_byte<W: Write>(src: &[u8], n: u8, w: &mut W) -> io::Result<()> {
    for raw_x in 1..(u16::from(n) + 1) {
        let x = Gf256::from_byte(raw_x as u8);
        let mut fac = Gf256::one();
        let mut acc = Gf256::zero();
        for &coeff in src.iter() {
            acc = acc + fac * Gf256::from_byte(coeff);
            fac = fac * x;
        }
        w.write_all(&[acc.to_byte()])?;
    }
    Ok(())
}
