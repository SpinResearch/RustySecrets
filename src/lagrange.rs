use gf256::Gf256;
use poly::Poly;

/// Evaluates an interpolated polynomial at `Gf256::zero()` where
/// the polynomial is determined using barycentric Lagrange
/// interpolation based on the given `points` in
/// the G(2^8) Galois field.
pub(crate) fn interpolate_at(k: u8, points: &[(u8, u8)]) -> u8 {
    barycentric_interpolate_at(k as usize, points)
}

/// Barycentric Lagrange interpolation algorithm from "Polynomial
/// Interpolation: Langrange vs Newton" by Wilhelm Werner. Evaluates
/// the polynomial at `Gf256::zero()`.
#[inline]
fn barycentric_interpolate_at(k: usize, points: &[(u8, u8)]) -> u8 {
    // Compute the barycentric weights `w`.
    let mut w = vec![Gf256::zero(); k];
    w[0] = Gf256::one();

    let mut x = Vec::with_capacity(k);
    x.push(Gf256::from_byte(points[0].0));

    for i in 1..k {
        x.push(Gf256::from_byte(points[i].0));
        for j in 0..i {
            let delta = x[j] - x[i];
            assert_ne!(delta.poly, 0, "Duplicate shares");
            w[j] /= delta;
            w[i] -= w[j];
        }
    }

    // Evaluate the second or "true" form of the barycentric
    // interpolation formula at `Gf256::zero()`.
    let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());
    for i in 0..k {
        assert_ne!(x[i].poly, 0, "Invalid share x = 0");
        let diff = w[i] / x[i];
        num += diff * Gf256::from_byte(points[i].1);
        denom += diff;
    }

    (num / denom).to_byte()
}

/// Computeds the coefficient of the Lagrange polynomial interpolated
/// from the given `points`, in the G(2^8) Galois field.
pub(crate) fn interpolate(points: &[(Gf256, Gf256)]) -> Poly {
    let len = points.len();

    let mut poly = vec![Gf256::zero(); len];

    for &(x, y) in points {
        assert_ne!(x.poly, 0, "Invalid share x = 0");
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

    use super::*;
    use gf256::*;
    use quickcheck::*;
    use std;

    quickcheck! {

        fn interpolate_evaluate_at_works(ys: Vec<Gf256>) -> TestResult {
            if ys.is_empty() || ys.len() > std::u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.into_iter()
                           .zip(1..std::u8::MAX)
                           .map(|(y, x)| (gf256!(x), y))
                           .collect::<Vec<_>>();
            let poly = interpolate(&points);

            for (x, y) in points {
                if poly.evaluate_at(x) != y {
                    return TestResult::failed();
                }
            }

            TestResult::passed()
        }

        fn interpolate_evaluate_at_0_eq_evaluate_at(ys: Vec<u8>) -> TestResult {
            if ys.is_empty() || ys.len() > std::u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.into_iter()
                           .zip(1..std::u8::MAX)
                           .map(|(y, x)| (x, y))
                           .collect::<Vec<_>>();

            let elems = points
                .iter()
                .map(|&(x, y)| (gf256!(x), gf256!(y)))
                .collect::<Vec<_>>();

            let poly = interpolate(&elems);

            let equals = poly.evaluate_at(Gf256::zero()).to_byte()
                == interpolate_at(points.len() as u8, points.as_slice());

            TestResult::from_bool(equals)
        }

    }

}
