use gf256::Gf256;

static MAX_COEFFS: usize = 256;

pub(crate) struct Poly {
    pub coeffs: Vec<Gf256>,
}

impl Poly {
    pub fn new(coeffs: Vec<Gf256>) -> Self {
        Self { coeffs }
    }

    pub fn evaluate_at_zero(&self) -> Gf256 {
        self.coeffs[0]
    }

    pub fn evaluate_at(&self, x: Gf256) -> Gf256 {
        assert!(self.coeffs.len() < MAX_COEFFS);

        let mut result = Gf256::zero();

        for (i, c) in self.coeffs.iter().enumerate() {
            result = result + *c * x.pow(i as u8);
        }

        result
    }
}
