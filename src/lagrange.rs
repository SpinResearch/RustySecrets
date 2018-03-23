use std::u8;

use errors::*;
use gf256::Gf256;
use poly::Poly;

/// Evaluates an interpolated polynomial at `Gf256::zero()` where the polynomial is determined
/// using barycentric Lagrange interpolation based on the given `points` in the G(2^8) Galois
/// field.
pub(crate) fn interpolate_at(threshold: u8, points: &[(u8, u8)]) -> Result<u8> {
    if points.len() < threshold as usize {
        bail!(ErrorKind::MissingShares(points.len(), threshold as usize));
    }
    let partial_comp = PartialSecret::new(threshold, points)?;
    Ok(partial_comp.secret.unwrap())
}

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
    /// shares)
    ids: Vec<Gf256>,
    /// The differences of share values divided by their ids.
    diffs: Vec<Gf256>,
    /// The barycentric weights.
    weights: Vec<Gf256>,
}

impl PartialSecret {
    /// Create a new partial computation given a `threshold` (to know when the computation is
    /// finished), and an initial set of `points`.
    #[inline]
    pub fn new(threshold: u8, points: &[(u8, u8)]) -> Result<Self> {
        if threshold < 2 {
            bail!(ErrorKind::ThresholdTooSmall(threshold));
        } else if points.len() == 0 {
            bail!(ErrorKind::EmptyShares);
        } else if points.len() > MAX_SHARES as usize {
            bail!(ErrorKind::InvalidShareCountMax(
                points.len() as u8,
                MAX_SHARES
            ));
        }

        let mut ids = Vec::with_capacity(points.len());
        let mut diffs = Vec::with_capacity(points.len());
        // If provided with more than `threshold` points, only the first threshold are considered.
        for pi in points.iter().take(threshold as usize) {
            if pi.0 == 0 {
                bail!(ErrorKind::ShareParsingInvalidShareId(0));
            }
            let xi = Gf256::from_byte(pi.0);
            if ids.iter().find(|&&xj| xi == xj).is_some() {
                bail!(ErrorKind::DuplicateShareId(xi.poly));
            }
            let yi = Gf256::from_byte(pi.1);
            ids.push(xi);
            // Storing these `diffs` instead of the `y` values allows us to do a little more
            // precomputation, since we really only need `y / x` and not `y` to evaluate the second
            // form of the barycentric interpolation formula.
            diffs.push(yi / xi);
        }

        let mut partial_comp = Self {
            secret: None,
            threshold,
            ids,
            diffs,
            weights: vec![],
        };

        if partial_comp.ids.len() == 1 {
            return Ok(partial_comp);
        }

        partial_comp.update_barycentric_weights();
        Ok(partial_comp)
    }

    /// Update the partial computation given an additional set of `points`.
    #[inline]
    pub fn update(&mut self, points: &[(u8, u8)]) -> Result<()> {
        if points.len() == 0 {
            bail!(ErrorKind::EmptyShares);
        } else if points.len() + self.ids.len() > MAX_SHARES as usize
            || self.ids.len() == self.threshold as usize
        {
            bail!(ErrorKind::InvalidShareCountMax(
                (points.len() + self.ids.len()) as u8,
                MAX_SHARES
            ));
        }

        // If provided with more than than `threshold - self.ids.len()` points, only enough to
        // satisfy the threshold are considered.
        for pi in points.iter().take(self.threshold as usize - self.ids.len()) {
            if pi.0 == 0 {
                bail!(ErrorKind::ShareParsingInvalidShareId(0));
            }
            let xi = Gf256::from_byte(pi.0);
            if self.ids.iter().find(|&&xj| xi == xj).is_some() {
                bail!(ErrorKind::DuplicateShareId(xi.poly));
            }
            let yi = Gf256::from_byte(pi.1);
            self.ids.push(xi);
            self.diffs.push(yi / xi);
        }

        self.update_barycentric_weights();
        Ok(())
    }

    /// Update the barycentric weights `w` corresponding to a set of `x` values.
    #[inline]
    fn update_barycentric_weights(&mut self) {
        let x = if self.weights.len() == 0 {
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
                self.weights[j] /= self.ids[j] - self.ids[i];
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

    /// Returns the number of shares needed to complete the computation.
    #[inline]
    pub fn shares_needed(&self) -> u8 {
        // Safe to cast and subtract because `ids.len()` will be less than `MAX_SHARES` and <=
        // `threshold`.
        self.threshold - self.ids.len() as u8
    }
}

/// Computeds the coefficient of the Lagrange polynomial interpolated
/// from the given `points`, in the G(2^8) Galois field.
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

            let equals = poly.evaluate_at(Gf256::zero()).to_byte()
                == interpolate_at(points.len() as u8, points.as_slice()).unwrap();

            TestResult::from_bool(equals)
        }

    }

}
