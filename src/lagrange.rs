use gf256::Gf256;
use poly::Poly;

/// Evaluates an interpolated polynomial at `Gf256::zero()` where the polynomial is determined
/// using barycentric Lagrange interpolation based on the given `points` in the G(2^8) Galois
/// field.
pub(crate) fn interpolate_at(points: &[(u8, u8)]) -> u8 {
    // Algorithm from "Polynomial Interpolation: Langrange vs Newton" by Wilhelm Werner.
    let x = points.iter().map(|x| Gf256::from_byte(x.0)).collect();
    let y = points.iter().map(|x| Gf256::from_byte(x.1)).collect();
    let w = compute_barycentric_weights(&x);
    let (num, denom) = compute_barycentric_num_denom_at(&x, &y, &w);
    (num / denom).to_byte()
}

/// Compute the barycentric weights `w` corresponding to a set of `x` values.
#[inline]
fn compute_barycentric_weights(x: &Vec<Gf256>) -> Vec<Gf256> {
    let k = x.len();
    let mut w = vec![Gf256::zero(); k];
    w[0] = Gf256::one();

    for i in 1..k {
        for j in 0..i {
            let delta = x[j] - x[i];
            assert_ne!(delta.poly, 0, "Duplicate shares");
            w[j] /= delta;
            w[i] -= w[j];
        }
    }

    w
}

// Compute the numerator and denominator of the second or "true" form of the barycentric
// interpolation formula at `Gf256::zero()`.
#[inline]
fn compute_barycentric_num_denom_at(
    x: &Vec<Gf256>,
    y: &Vec<Gf256>,
    w: &Vec<Gf256>,
) -> (Gf256, Gf256) {
    let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());

    for (i, &xi) in x.iter().enumerate() {
        assert_ne!(xi.poly, 0, "Invalid share x = 0");
        let diff = w[i] / xi;
        num += diff * y[i];
        denom += diff;
    }

    (num, denom)
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
                == interpolate_at(points.as_slice());

            TestResult::from_bool(equals)
        }

    }

}
