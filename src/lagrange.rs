/// Implements barycentric Lagrange interpolation.

use errors::*;
use gf256::Gf256;
use poly::Poly;

/// Stores the intermediate state of interpolation and evaluation at `Gf256::zero()` of a
/// polynomial. A secret may be computed incrementally using barycentric Lagrange interpolation.
pub struct BarycentricWeights {
    /// The number of shares necessary to recover the secret, a.k.a. the threshold.
    pub diffs: Vec<Gf256>,
    /// The barycentric weights.
    pub weights: Vec<Gf256>,
}

// `BarycentricWeights` is not a public-facing struct. We expect the functions that interact with
// it to do validation of its operands (namely, the methods of `sss::Recover`.) Still, we have
// included many assertions to guard against clearly wrong inputs.
impl BarycentricWeights {
    /// Create a new partial computation given a `threshold` (to know when the computation is
    /// finished), and an initial set of `points`.
    #[inline]
    pub fn new(ids: &[Gf256], new_ys: &[Gf256]) -> Self {
        let (new_points, total_points) = (new_ys.len(), ids.len());
        assert_ne!(new_points, 0, "Given an empty set of points!");
        assert_eq!(
            new_points, total_points,
            "Given an unequal number of x and y coordinates!"
        );

        let mut partial_comp = Self {
            diffs: Vec::with_capacity(total_points),
            weights: vec![],
        };

        partial_comp.update_diffs(ids, new_ys);
        partial_comp.update_barycentric_weights(ids);
        partial_comp
    }

    /// Update the partial computation given an additional set of `points`.
    #[inline]
    pub fn update(&mut self, ids: &[Gf256], new_ys: &[Gf256]) {
        let (new_points, total_points) = (new_ys.len(), ids.len());
        assert_ne!(new_points, 0, "Given an empty set of points!");
        assert!(total_points > 1, "In order to call update you must have already processed at least
        one point to make a `BarycentricWeights`, and you must provide at least a second in your
        call to update.");
        assert!(
            total_points > new_points,
            "During an update IDs of the shares processed should at least number the new y values."
        );
        // We use diffs here and not weights because no weights are generated until at least 2
        // points have been interpolated.
        assert_eq!(new_points + self.diffs.len(), total_points, "The new points given plus the
        existing diffs should be equal in length to the total points given.");

        self.update_diffs(ids, new_ys);
        self.update_barycentric_weights(ids);
    }

    /// Parse the `new_ys` into `diffs`.
    #[inline]
    fn update_diffs(&mut self, ids: &[Gf256], new_ys: &[Gf256]) {
        let (new_points, total_points) = (new_ys.len(), ids.len());
        let ids = &ids[(total_points - new_points)..];
        self.diffs.reserve_exact(new_points);

        for (&xi, &yi) in ids.iter().zip(new_ys.iter()) {
            assert!(xi.poly != 0, "Given invalid share identifier 0!");
            // Storing these `diffs` instead of the `y` values allows us to do a little more
            // precomputation, since we really only need `y / x` and not `y` to evaluate the second
            // form of the barycentric interpolation formula.
            self.diffs.push(yi / xi);
        }
    }

    /// Update the barycentric weights `w` corresponding to a set of `x` values.
    #[inline]
    fn update_barycentric_weights(&mut self, ids: &[Gf256]) {
        let total_points = ids.len();
        // Need at least two points to start computing the barycentric weights.
        if total_points == 1 {
            return;
        }
        let new_points = total_points - self.weights.len();

        let start_weight = if self.weights.is_empty() {
            // Initialize initial weights.
            self.weights = vec![Gf256::zero(); total_points];
            self.weights[0] = Gf256::one();
            1
        } else {
            // Initialize additional weights.
            self.weights.append(&mut vec![Gf256::zero(); new_points]);
            total_points - new_points
        };

        // Update weights using algorithm (3.1) from "Polynomial Interpolation: Langrange vs
        // Newton" by Wilhelm Werner.
        for i in start_weight..total_points {
            for j in 0..i {
                let diff = ids[j] - ids[i];
                assert!(diff.poly != 0, "Duplicate share identifiers encountered!");
                self.weights[j] /= diff;
                self.weights[i] -= self.weights[j];
            }
        }
    }
}

/// Compute the secret using the second or "true" form of the barycentric interpolation formula
/// at `Gf256::zero()`.
#[inline]
pub fn evaluate_at_zero(wds: &BarycentricWeights, ids: &[Gf256]) -> u8 {
    validate_evaluation_parameters(wds, ids);

    let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());
    for ((&xi, &di), &wi) in ids.iter().zip(wds.diffs.iter()).zip(wds.weights.iter()) {
        num += wi * di;
        denom += wi / xi;
    }

    (num / denom).to_byte()
}

/// Evaluate the interpolated polynomial at the point `gf256!(x)` in the G(2^8)
/// Galois field.
#[inline]
fn evaluate_at_x(wds: &BarycentricWeights, ids: &[Gf256], x: Gf256) -> Result<u8> {
    validate_evaluation_parameters(wds, ids);

    let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());
    for ((&xi, &di), &wi) in ids.iter().zip(wds.diffs.iter()).zip(wds.weights.iter()) {
        let delta = x - xi;
        // Slightly slower to re-multiply the `diffs` by `xi` here, but otherwise we have to
        // additionally store the `y` values in `BarycentricWeights`, or store `y` values instead
        // of the `diffs` and precompute less in the standard case of evaluating at 0.
        num += wi * di * xi / delta;
        denom += wi / delta;
    }
    Ok((num / denom).to_byte())
}

/// Validates the `BarycentricWeights` and `ids` can successfully be used to compute a secret byte.
#[inline]
fn validate_evaluation_parameters(wds: &BarycentricWeights, ids: &[Gf256]) {
    let num_weights = wds.weights.len();
    let num_diffs = wds.diffs.len();
    assert_eq!(
        num_weights, num_diffs,
        "`BarycentricWeights` should contain the same number of weights and diffs!"
    );
    let num_points = ids.len();
    assert_eq!(
        num_weights, num_points,
        "The number of barycentric weights is not equal to the number of IDs!"
    );
    assert!(
        num_points >= 2,
        "Can't evaluate a polynomial at 0 without at least having processed two points."
    );
}

/// Computes the coefficient of the Lagrange polynomial interpolated from the given `points`, in
//the G(2^8) Galois field.
#[inline]
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
    use std::u8;

    quickcheck! {

        fn interpolate_evaluate_at_works(ys: Vec<Gf256>) -> TestResult {
            if ys.len() < 2 || ys.len() > u8::MAX as usize {
                return TestResult::discard();
            }

            let points = ys.into_iter()
                           .zip(1..u8::MAX)
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
            if ys.len() < 2 || ys.len() > u8::MAX as usize {
                return TestResult::discard();
            }

            // Safe to cast because if `ys.len() > 255` it is discarded.
            let num_points = ys.len() as u8;
            let ids: Vec<Gf256> = (1..(num_points + 1)).map(|x| gf256!(x)).collect();
            let ys: Vec<Gf256> = ys.iter().map(|&y| gf256!(y)).collect();

            let elems: Vec<(Gf256, Gf256)> = ids.iter()
                                                .zip(ys.iter())
                                                .map(|(&x, &y)| (x, y))
                                                .collect();

            let poly = interpolate(&elems);
            let result_poly = poly.evaluate_at(Gf256::zero()).to_byte();

            let wds = BarycentricWeights::new(&ids, &ys);
            let result_interpolate = evaluate_at_zero(&wds, &ids);

            TestResult::from_bool(result_poly == result_interpolate)
        }

    }

}
