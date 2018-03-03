use gf256::Gf256;
use poly::Poly;

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
                prod *= xj / delta;
            }
        }
        sum += prod * yi;
    }
    sum.to_byte()
}

/// Computeds the coefficient of the Lagrange polynomial interpolated
/// from the given `points`, in the G(2^8) Galois field.
pub(crate) fn interpolate(points: &[(Gf256, Gf256)]) -> Poly {
    let len = points.len();

    let mut poly = vec![Gf256::zero(); len];

    for &(x, y) in points {
        let mut coeffs = vec![Gf256::zero(); len];
        coeffs[0] = y;

        let mut prod = Gf256::one();
        for &(x1, _) in points {
            if x != x1 {
                prod *= x - x1;

                let mut prec = Gf256::zero();
                coeffs = coeffs
                    .into_iter()
                    .map(|coeff| {
                        let new_coeff = coeff * (-x1) + prec;
                        prec = coeff;
                        new_coeff
                    })
                    .collect();
            }
        }

        poly = poly.iter()
            .zip(coeffs.iter())
            .map(|(&old_coeff, &add)| old_coeff + add / prod)
            .collect();
    }

    Poly::new(poly)
}

#[cfg(test)]
#[allow(trivial_casts)]
mod tests {

    use std;
    use super::*;
    use gf256::*;
    use quickcheck::*;

    quickcheck! {

        fn evaluate_at_works(ys: Vec<u8>) -> TestResult {
            if ys.is_empty() || ys.len() > std::u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.iter().enumerate().map(|(x, y)| (x as u8, *y)).collect::<Vec<_>>();
            let equals = interpolate_at(points.as_slice()) == ys[0];

            TestResult::from_bool(equals)
        }


        fn interpolate_evaluate_at_works(ys: Vec<Gf256>) -> TestResult {
            if ys.is_empty() || ys.len() > std::u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.into_iter().enumerate().map(|(x, y)| (gf256!(x as u8), y)).collect::<Vec<_>>();
            let poly = interpolate(&points);

            for (x, y) in points {
                if poly.evaluate_at(x) != y {
                    return TestResult::failed();
                }
            }

            TestResult::passed()
        }

        fn interpolate_evaluate_at_0_eq_evaluate_at(ys: Vec<u8>) -> TestResult {
            if ys.len() > std::u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.into_iter().enumerate().map(|(x, y)| (x as u8, y)).collect::<Vec<_>>();

            let elems = points
                .iter()
                .map(|&(x, y)| (gf256!(x), gf256!(y)))
                .collect::<Vec<_>>();

            let poly = interpolate(&elems);

            let equals = poly.evaluate_at(Gf256::zero()).to_byte() == interpolate_at(points.as_slice());

            TestResult::from_bool(equals)
        }

    }

}
