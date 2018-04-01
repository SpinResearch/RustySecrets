use std::u8;

use errors::*;
use gf256::Gf256;
use poly::Poly;

/// Stores the intermediate state of interpolation and evaluation at `Gf256::zero()` of a
/// polynomial. A secret may be computed incrementally using barycentric Lagrange interpolation.
/// The state is updated with new points until threshold points have been evaluated, at which point
/// the `secret` field will be updated from `None` to `Some(u8)`.
pub struct PartialSecret {
    /// The secret byte. `None` until computation is complete.
    secret: Option<u8>,
    /// The number of shares necessary to recover the secret, a.k.a. the threshold.
    threshold: u8,
    /// The ids of the share (varies between 1 and n where n is the total number of generated
    /// shares).
    ids: Vec<Gf256>,
    /// The differences of share values divided by their ids.
    diffs: Vec<Gf256>,
    /// The barycentric weights.
    weights: Vec<Gf256>,
}

// `PartialSecret` is not a public-facing struct. We expect the functions that interact with it to
// do validation of the `points` and other arguments it operates on. As a defensive programming
// practice, we have included `assert!` statements, which should also clarify the validation
// expectations of each method.
impl PartialSecret {
    /// Create a new partial computation given a `threshold` (to know when the computation is
    /// finished), and an initial set of `points`.
    #[inline]
    pub fn new(threshold: u8, points: &[(u8, u8)]) -> Self {
        assert!(threshold >= 2, "Given k less than 2!");
        assert!(!points.is_empty(), "Given an empty set of points!");
        assert!(
            points.len() <= threshold as usize,
            "Given more than threshold shares!"
        );

        let mut partial_comp = Self {
            secret: None,
            threshold,
            ids: Vec::with_capacity(threshold as usize),
            diffs: Vec::with_capacity(threshold as usize),
            weights: vec![],
        };

        partial_comp.update_diffs(points);
        partial_comp.update_barycentric_weights();
        partial_comp
    }

    /// Update the partial computation given an additional set of `points`.
    #[inline]
    pub fn update(&mut self, points: &[(u8, u8)]) {
        assert!(!points.is_empty(), "Given an empty set of points!");
        assert!(
            self.shares_interpolated() as usize + points.len() < self.threshold as usize,
            "Given more than threshold shares!"
        );

        self.update_diffs(points);
        self.update_barycentric_weights();
    }

    /// Parse just the `points` we need to compute the secret into `x` values and `diffs`.
    fn update_diffs(&mut self, points: &[(u8, u8)]) {
        for pi in points.iter() {
            let xi = Gf256::from_byte(pi.0);
            assert!(xi.poly != 0, "Given invalid share identifier 0!");
            let yi = Gf256::from_byte(pi.1);
            self.ids.push(xi);
            // Storing these `diffs` instead of the `y` values allows us to do a little more
            // precomputation, since we really only need `y / x` and not `y` to evaluate the second
            // form of the barycentric interpolation formula.
            self.diffs.push(yi / xi);
        }
    }

    /// Update the barycentric weights `w` corresponding to a set of `x` values.
    #[inline]
    fn update_barycentric_weights(&mut self) {
        // Need at least two points to start computing the barycentric weights.
        if self.ids.len() == 1 {
            return;
        }

        let x = if self.weights.is_empty() {
            // Initialize initial weights.
            self.weights = vec![Gf256::zero(); self.ids.len()];
            self.weights[0] = Gf256::one();
            1
        } else {
            // Initialize additional weights.
            let initial_len = self.weights.len();
            self.weights
                .append(&mut vec![Gf256::zero(); self.ids.len() - initial_len]);
            initial_len
        };

        // Update weights using algorithm (3.1) from "Polynomial Interpolation: Langrange vs
        // Newton" by Wilhelm Werner.
        for i in x..self.ids.len() {
            for j in 0..i {
                let diff = self.ids[j] - self.ids[i];
                assert!(diff.poly != 0, "Duplicate share identifiers encountered!");
                self.weights[j] /= diff;
                self.weights[i] -= self.weights[j];
            }
        }

        // If we have sufficient information, we can compute the secret.
        if self.shares_needed() == 0 {
            self.compute_secret();
        }
    }

    /// Compute the secret using the second or "true" form of the barycentric interpolation formula
    /// at `Gf256::zero()`.
    #[inline]
    fn compute_secret(&mut self) {
        let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());
        for ((&xi, &di), &wi) in self.ids
            .iter()
            .zip(self.diffs.iter())
            .zip(self.weights.iter())
        {
            num += wi * di;
            denom += wi / xi;
        }
        self.secret = Some((num / denom).to_byte());
    }

    /// If the partial computation is complete, return the secret, else an error.
    #[inline]
    pub fn get_secret(&self) -> Result<u8> {
        if self.secret.is_none() {
            bail!(ErrorKind::PartialInterpolationNotComplete(
                self.threshold,
                self.shares_interpolated()
            ))
        }
        // Safe to unwrap because we just confirmed it's not `None`.
        Ok(self.secret.unwrap())
    }

    /// Returns the threshold for the partial computation.
    #[inline]
    fn get_threshold(&self) -> u8 {
        self.threshold
    }

    /// Returns the number of shares needed to complete the computation.
    #[inline]
    pub fn shares_needed(&self) -> u8 {
        // Casting is safe because `assert!` statements in `new` and `update` ensure
        // `self.ids.len()` will be less than 255.
        self.threshold - self.ids.len() as u8
    }

    /// Returns the number of shares that have been interpolated so far.
    #[inline]
    pub fn shares_interpolated(&self) -> u8 {
        // Casting is safe because `assert!` statements in `new` and `update` ensure
        // `self.ids.len()` will be less than 255.
        self.ids.len() as u8
    }

    /// Evaluate the interpolated polynomial at the point `Gf256::from_byte(x)` in the G(2^8)
    /// Galois field.
    #[inline]
    fn evaluate_at_x(&self, x: u8) -> Result<u8> {
        if self.shares_needed() != 0 {
            bail!(ErrorKind::PartialInterpolationNotComplete(
                self.threshold,
                self.shares_interpolated()
            ))
        }

        let x = Gf256::from_byte(x);
        let (mut num, mut denom) = (Gf256::zero(), Gf256::zero());
        for ((&xi, &di), &wi) in self.ids
            .iter()
            .zip(self.diffs.iter())
            .zip(self.weights.iter())
        {
            let delta = x - xi;
            // Slightly slower to re-multiply the `diffs` by `xi` here, but otherwise we have to
            // additionally store the `y` values in `PartialSecret`, or store `y` values instead of
            // the `diffs` and precompute less in the standard case of evaluating at 0.
            num += wi * di * xi / delta;
            denom += wi / delta;
        }
        Ok((num / denom).to_byte())
    }
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
    use std;

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

            let points = ys.into_iter()
                           .zip(1..u8::MAX)
                           .map(|(y, x)| (x, y))
                           .collect::<Vec<_>>();

            let elems = points
                .iter()
                .map(|&(x, y)| (gf256!(x), gf256!(y)))
                .collect::<Vec<_>>();

            let poly = interpolate(&elems);
            let result_poly = poly.evaluate_at(Gf256::zero()).to_byte();
            // Safe to cast because if `ys.len() > 255` it is discarded.
            let interpolation = PartialSecret::new(points.len() as u8, &points);
            let result_interpolate = interpolation.get_secret().unwrap();

            TestResult::from_bool(result_poly == result_interpolate)
        }

    }

}
