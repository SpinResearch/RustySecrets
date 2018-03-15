use gf256::Gf256;
use std::io;
use std::io::prelude::*;

/// evaluates a polynomial at x=1, 2, 3, ... n (inclusive)
pub(crate) fn encode_secret_byte<W: Write>(src: &[u8], n: u8, w: &mut W) -> io::Result<()> {
    for raw_x in 1..(u16::from(n) + 1) {
        let x = Gf256::from_byte(raw_x as u8);
        let sum = src.iter().rev().fold(Gf256::zero(), |acc, &coeff| {
            Gf256::from_byte(coeff) + acc * x
        });
        w.write_all(&[sum.to_byte()])?;
    }
    Ok(())
}
