use gf256::Gf256;
use std::io;
use std::io::prelude::*;

/// evaluates a polynomial at x=1, 2, 3, ... n (inclusive)
pub fn encode<W: Write>(src: &[u8], n: u8, w: &mut W) -> io::Result<()> {
    for raw_x in 1..((n as u16) + 1) {
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

/// evaluates an interpolated polynomial at `Gf256::zero()` where
/// the polynomial is determined using Lagrangian interpolation
/// based on the given x/y coordinates `src`.
pub fn lagrange_interpolate(src: &[(u8, u8)]) -> u8 {
    let mut sum = Gf256::zero();
    for (i, &(raw_xi, raw_yi)) in src.iter().enumerate() {
        let xi = Gf256::from_byte(raw_xi);
        let yi = Gf256::from_byte(raw_yi);
        let mut prod = Gf256::one();
        for (j, &(raw_xj, _)) in src.iter().enumerate() {
            if i != j {
                let xj = Gf256::from_byte(raw_xj);
                let delta = xi - xj;
                assert!(delta.poly != 0, "Duplicate shares");
                prod = prod * xj / delta;
            }
        }
        sum = sum + prod * yi;
    }
    sum.to_byte()
}
