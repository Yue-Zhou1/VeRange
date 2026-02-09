use crate::ProofError;
use ark_ff::{Field, PrimeField};
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};

pub(crate) fn commit_with_basis_and_h(
    basis: &[CurvePoint],
    coeffs: &[Scalar],
    h: &CurvePoint,
    r: Scalar,
) -> Result<Commitment, ProofError> {
    if basis.len() != coeffs.len() {
        return Err(ProofError::InvalidStatement(
            "basis and coefficient lengths must match",
        ));
    }

    let mut point = *h * r;
    for (g, c) in basis.iter().zip(coeffs.iter()) {
        point += *g * *c;
    }
    Ok(Commitment::new(point))
}

pub(crate) fn inner_product(a: &[Scalar], b: &[Scalar]) -> Result<Scalar, ProofError> {
    if a.len() != b.len() {
        return Err(ProofError::InvalidProof(
            "inner-product vectors must have same length",
        ));
    }
    Ok(a.iter()
        .zip(b.iter())
        .fold(Scalar::from(0u64), |acc, (x, y)| acc + (*x * *y)))
}

pub(crate) fn invert_or_err(value: Scalar, msg: &'static str) -> Result<Scalar, ProofError> {
    value.inverse().ok_or(ProofError::InvalidProof(msg))
}

pub(crate) fn build_base_powers(nbits: usize, base: Scalar) -> Vec<Scalar> {
    (0..nbits).map(|i| pow_usize(base, i)).collect()
}

pub(crate) fn biguint_to_scalar(value: &BigUint) -> Scalar {
    Scalar::from_be_bytes_mod_order(&value.to_bytes_be())
}

pub(crate) fn pow_usize(base: Scalar, exp: usize) -> Scalar {
    base.pow([exp as u64])
}

pub(crate) fn random_scalar(rng: &mut impl RngCore) -> Scalar {
    let mut bytes = [0u8; 64];
    rng.fill_bytes(&mut bytes);
    Scalar::from_be_bytes_mod_order(&bytes)
}
