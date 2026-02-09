use crate::PolyCommitError;
use ark_ff::{Field, One, Zero};
use verange_core::curve::Scalar;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Polynomial {
    coeffs: Vec<Scalar>,
}

impl Polynomial {
    pub fn from_coeffs(mut coeffs: Vec<Scalar>) -> Self {
        trim_trailing_zeroes(&mut coeffs);
        if coeffs.is_empty() {
            coeffs.push(Scalar::zero());
        }
        Self { coeffs }
    }

    pub fn zero() -> Self {
        Self {
            coeffs: vec![Scalar::zero()],
        }
    }

    pub fn coeffs(&self) -> &[Scalar] {
        &self.coeffs
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|c| c.is_zero())
    }

    pub fn add(&self, other: &Self) -> Self {
        let size = self.coeffs.len().max(other.coeffs.len());
        let mut out = vec![Scalar::zero(); size];

        for (i, coeff) in self.coeffs.iter().enumerate() {
            out[i] += *coeff;
        }
        for (i, coeff) in other.coeffs.iter().enumerate() {
            out[i] += *coeff;
        }
        Self::from_coeffs(out)
    }

    pub fn sub(&self, other: &Self) -> Self {
        let size = self.coeffs.len().max(other.coeffs.len());
        let mut out = vec![Scalar::zero(); size];

        for (i, coeff) in self.coeffs.iter().enumerate() {
            out[i] += *coeff;
        }
        for (i, coeff) in other.coeffs.iter().enumerate() {
            out[i] -= *coeff;
        }
        Self::from_coeffs(out)
    }

    pub fn mul(&self, other: &Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }

        let mut out = vec![Scalar::zero(); self.degree() + other.degree() + 1];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                out[i + j] += *a * *b;
            }
        }
        Self::from_coeffs(out)
    }

    pub fn evaluate(&self, x: Scalar) -> Scalar {
        let mut acc = Scalar::zero();
        for coeff in self.coeffs.iter().rev() {
            acc *= x;
            acc += *coeff;
        }
        acc
    }

    pub fn long_divide(&self, divisor: &Self) -> Result<(Self, Self), PolyCommitError> {
        if divisor.is_zero() {
            return Err(PolyCommitError::DivisionByZeroPolynomial);
        }

        if self.degree() < divisor.degree() {
            return Ok((Self::zero(), self.clone()));
        }

        let mut remainder = self.clone();
        let quotient_len = self.degree() - divisor.degree() + 1;
        let mut quotient = vec![Scalar::zero(); quotient_len];

        let divisor_lead = *divisor
            .coeffs
            .last()
            .ok_or(PolyCommitError::DivisionByZeroPolynomial)?;
        let divisor_lead_inv = divisor_lead
            .inverse()
            .ok_or(PolyCommitError::MissingInverse("divisor leading coefficient"))?;

        while !remainder.is_zero() && remainder.degree() >= divisor.degree() {
            let degree_diff = remainder.degree() - divisor.degree();
            let lead = *remainder
                .coeffs
                .last()
                .ok_or(PolyCommitError::DivisionByZeroPolynomial)?
                * divisor_lead_inv;

            quotient[degree_diff] += lead;
            let shifted = divisor.mul_monomial(lead, degree_diff);
            remainder = remainder.sub(&shifted);
        }

        Ok((Self::from_coeffs(quotient), remainder))
    }

    pub fn mul_monomial(&self, coeff: Scalar, degree: usize) -> Self {
        if coeff.is_zero() || self.is_zero() {
            return Self::zero();
        }

        let mut out = vec![Scalar::zero(); self.coeffs.len() + degree];
        for (idx, c) in self.coeffs.iter().enumerate() {
            out[idx + degree] = *c * coeff;
        }
        Self::from_coeffs(out)
    }
}

pub fn vanishing_polynomial(domain_size: usize) -> Polynomial {
    if domain_size == 0 {
        return Polynomial::zero();
    }

    let mut coeffs = vec![Scalar::zero(); domain_size + 1];
    coeffs[0] = -Scalar::one();
    coeffs[domain_size] = Scalar::one();
    Polynomial::from_coeffs(coeffs)
}

fn trim_trailing_zeroes(coeffs: &mut Vec<Scalar>) {
    while coeffs.last().is_some_and(|c| c.is_zero()) {
        coeffs.pop();
    }
}
