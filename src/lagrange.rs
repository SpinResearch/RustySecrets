use gf256::Gf256;

/// Evaluates an interpolated polynomial at `Gf256::zero()` where
/// the polynomial is determined using Lagrangian interpolation
/// based on the given `points` in the G(2^8) Galois field.
pub(crate) fn interpolate_at(points: &[(u8, u8)]) -> u8 {
    let mut sum = Gf256::zero();
    for (i, &(raw_xi, raw_yi)) in points.iter().enumerate() {
        let xi = Gf256::from_byte(raw_xi);
        let yi = Gf256::from_byte(raw_yi);
        let mut prod = Gf256::one();
        for (j, &(raw_xj, _)) in points.iter().enumerate() {
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

/// Computeds the coefficient of the Lagrange polynomial interpolated
/// from the given `points`, in the G(2^8) Galois field.
pub(crate) fn interpolate(points: &[(Gf256, Gf256)]) -> Vec<Gf256> {
    let len = points.len();

    let mut poly = vec![Gf256::zero(); len];

    for &(x, y) in points {
        let mut coeffs = vec![Gf256::zero(); len];
        coeffs[0] = y;

        let mut prod = Gf256::one();
        for &(x1, _) in points {
            if x != x1 {
                prod = prod * (x - x1);

                let mut prec = Gf256::zero();
                for mut coeff in coeffs.iter_mut() {
                    let new_coeff = *coeff * (-x1) + prec;
                    prec = *coeff;
                    *coeff = new_coeff;
                }
            }
        }

        poly = poly.iter()
            .zip(coeffs.iter())
            .map(|(&old_coeff, &add)| old_coeff + add / prod)
            .collect();
    }

    poly
}

pub(crate) fn evaluate_at_zero(poly: &[Gf256]) -> Gf256 {
    poly[0]
}

pub(crate) fn evaluate_at(poly: &[Gf256], x: Gf256) -> Gf256 {
    assert!(poly.len() < 256);

    let mut result = Gf256::zero();

    for (i, c) in poly.iter().enumerate() {
        result = result + *c * x.pow(i as u8);
    }

    result
}
