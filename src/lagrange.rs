use gf256::Gf256;

/// evaluates an interpolated polynomial at `Gf256::zero()` where
/// the polynomial is determined using Lagrangian interpolation
/// based on the given x/y coordinates `src`.
pub(crate) fn lagrange_interpolate(src: &[(u8, u8)]) -> u8 {
    let mut sum = Gf256::zero();
    for (i, &(raw_xi, raw_yi)) in src.iter().enumerate() {
        let xi = Gf256::from_byte(raw_xi);
        let yi = Gf256::from_byte(raw_yi);
        let mut prod = Gf256::one();
        for (j, &(raw_xj, _)) in src.iter().enumerate() {
            if i != j {
                let xj = Gf256::from_byte(raw_xj);
                let delta = xi - xj;
                assert_ne!(delta.poly, 0, "Duplicate shares");
                prod = prod * xj / delta;
            }
        }
        sum = sum + prod * yi;
    }
    sum.to_byte()
}
