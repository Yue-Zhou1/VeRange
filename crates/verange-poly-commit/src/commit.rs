use crate::polynomial::Polynomial;
use crate::PolyCommitError;
use ark_bn254::G1Projective;
use ark_ec::Group;
use ark_ff::{Field, Zero};
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};

#[derive(Clone, Debug)]
pub struct PolyCommitParams {
    generators: Vec<CurvePoint>,
}

impl PolyCommitParams {
    pub fn new(num_generators: usize) -> Result<Self, PolyCommitError> {
        if num_generators == 0 {
            return Err(PolyCommitError::InsufficientGenerators {
                required: 1,
                available: 0,
            });
        }

        let base = G1Projective::generator();
        let generators = (0..num_generators)
            .map(|i| base * Scalar::from((i + 71) as u64))
            .collect::<Vec<_>>();

        Ok(Self { generators })
    }

    pub fn generators(&self) -> &[CurvePoint] {
        &self.generators
    }
}

pub fn commit_poly(
    poly: &Polynomial,
    m: usize,
    n: usize,
    degree: usize,
    blinding: &[Scalar],
    params: &PolyCommitParams,
) -> Result<Vec<Commitment>, PolyCommitError> {
    ensure_generator_count(params, n + 1)?;
    let h = setup_hijs(poly, m, n, degree, blinding)?;

    let mut commitments = Vec::with_capacity(m + 1);
    for row in h.iter().take(m + 1) {
        commitments.push(msm(params.generators(), row)?);
    }
    Ok(commitments)
}

pub fn open_poly(
    poly: &Polynomial,
    x: Scalar,
    m: usize,
    n: usize,
    degree: usize,
    blinding: &[Scalar],
) -> Result<Vec<Scalar>, PolyCommitError> {
    let h = setup_hijs(poly, m, n, degree, blinding)?;
    let mut pi = vec![Scalar::zero(); n + 1];

    for (i, pi_i) in pi.iter_mut().enumerate().take(n + 1) {
        let mut acc = Scalar::zero();
        for (j, row) in h.iter().enumerate().take(m + 1) {
            acc += row[i] * pow_usize(x, j);
        }
        *pi_i = acc;
    }

    Ok(pi)
}

pub fn open_poly_batch(
    polys: &[Polynomial],
    x: Scalar,
    rho: Scalar,
    m: usize,
    n: usize,
    degree: usize,
    blindings: &[Vec<Scalar>],
) -> Result<Vec<Scalar>, PolyCommitError> {
    if polys.len() != blindings.len() {
        return Err(PolyCommitError::InvalidDimensions(
            "polys and blindings length mismatch",
        ));
    }

    let mut out = vec![Scalar::zero(); n + 1];
    for (idx, poly) in polys.iter().enumerate() {
        let pi = open_poly(poly, x, m, n, degree, &blindings[idx])?;
        let weight = pow_usize(rho, idx + 1);
        for (slot, value) in out.iter_mut().zip(pi.iter()) {
            *slot += *value * weight;
        }
    }
    Ok(out)
}

pub fn verify_poly(
    commitments: &[Commitment],
    x: Scalar,
    y: Scalar,
    pi: &[Scalar],
    degree: usize,
    params: &PolyCommitParams,
) -> Result<bool, PolyCommitError> {
    if commitments.is_empty() || pi.is_empty() {
        return Err(PolyCommitError::InvalidDimensions(
            "commitments and pi must be non-empty",
        ));
    }

    let m = commitments.len() - 1;
    let n = pi.len() - 1;
    ensure_generator_count(params, n + 1)?;

    let lhs = msm(params.generators(), pi)?;
    let rhs = commitments
        .iter()
        .enumerate()
        .fold(Commitment::identity(), |acc, (i, cm)| {
            acc.add(&cm.mul_scalar(pow_usize(x, i)))
        });
    let b1 = lhs == rhs;

    let eta = degree
        .checked_sub(m * n)
        .ok_or(PolyCommitError::InvalidDimensions(
            "degree must be >= m*n in verify",
        ))?;

    let mut eval = pi[0];
    for (i, value) in pi.iter().enumerate().take(n + 1).skip(1) {
        let exp = (i - 1) * m + eta;
        eval += *value * pow_usize(x, exp);
    }

    Ok(b1 && eval == y)
}

pub fn verify_poly_batch(
    commitments_batch: &[Vec<Commitment>],
    x: Scalar,
    rho: Scalar,
    y: Scalar,
    pi: &[Scalar],
    degree: usize,
    params: &PolyCommitParams,
) -> Result<bool, PolyCommitError> {
    if commitments_batch.is_empty() || pi.is_empty() {
        return Err(PolyCommitError::InvalidDimensions(
            "batch commitments and pi must be non-empty",
        ));
    }

    let m = commitments_batch[0]
        .len()
        .checked_sub(1)
        .ok_or(PolyCommitError::InvalidDimensions(
            "commitment rows must be non-empty",
        ))?;
    if commitments_batch.iter().any(|row| row.len() != m + 1) {
        return Err(PolyCommitError::InvalidDimensions(
            "all batched commitment rows must have same length",
        ));
    }

    let n = pi.len() - 1;
    ensure_generator_count(params, n + 1)?;

    let lhs = msm(params.generators(), pi)?;
    let mut rhs = Commitment::identity();
    for (poly_idx, commitments) in commitments_batch.iter().enumerate() {
        let rho_weight = pow_usize(rho, poly_idx + 1);
        for (j, cm) in commitments.iter().enumerate() {
            rhs = rhs.add(&cm.mul_scalar(rho_weight * pow_usize(x, j)));
        }
    }
    let b1 = lhs == rhs;

    let eta = degree
        .checked_sub(m * n)
        .ok_or(PolyCommitError::InvalidDimensions(
            "degree must be >= m*n in batch verify",
        ))?;

    let mut eval = pi[0];
    for (i, value) in pi.iter().enumerate().take(n + 1).skip(1) {
        let exp = (i - 1) * m + eta;
        eval += *value * pow_usize(x, exp);
    }
    Ok(b1 && eval == y)
}

fn setup_hijs(
    poly: &Polynomial,
    m: usize,
    n: usize,
    degree: usize,
    blinding: &[Scalar],
) -> Result<Vec<Vec<Scalar>>, PolyCommitError> {
    if blinding.len() < n {
        return Err(PolyCommitError::InvalidDimensions(
            "blinding length must be >= n",
        ));
    }
    if degree < poly.degree() {
        return Err(PolyCommitError::InvalidDimensions(
            "degree must be >= polynomial degree",
        ));
    }
    if degree < m * n {
        return Err(PolyCommitError::InvalidDimensions(
            "degree must be >= m*n",
        ));
    }

    let mut coeffs = poly.coeffs().to_vec();
    while coeffs.len() <= degree {
        coeffs.push(Scalar::zero());
    }

    let eta = degree - (m * n);
    let mut h = vec![vec![Scalar::zero(); n + 1]; m + 1];

    h[0][0] = if eta == 0 {
        coeffs[0] - blinding[0]
    } else {
        coeffs[0]
    };

    for i in 1..=n {
        h[0][i] = blinding[i - 1];
    }

    for i in 1..=m {
        h[i][0] = if eta == 0 {
            Scalar::zero()
        } else if i < eta {
            coeffs[i]
        } else if i == eta {
            coeffs[i] - blinding[0]
        } else {
            Scalar::zero()
        };
    }

    let mut flag = eta;
    for i in 1..=n {
        for row in h.iter_mut().take(m + 1).skip(1) {
            flag += 1;
            row[i] = coeffs[flag];
        }
    }

    if n > 0 {
        for i in 1..n {
            h[m][i] -= blinding[i];
        }
    }

    Ok(h)
}

fn msm(generators: &[CurvePoint], scalars: &[Scalar]) -> Result<Commitment, PolyCommitError> {
    if generators.len() < scalars.len() {
        return Err(PolyCommitError::InsufficientGenerators {
            required: scalars.len(),
            available: generators.len(),
        });
    }

    let mut acc = CurvePoint::zero();
    for (g, s) in generators.iter().zip(scalars.iter()) {
        acc += *g * *s;
    }
    Ok(Commitment::new(acc))
}

fn pow_usize(base: Scalar, exp: usize) -> Scalar {
    base.pow([exp as u64])
}

fn ensure_generator_count(params: &PolyCommitParams, required: usize) -> Result<(), PolyCommitError> {
    if params.generators.len() < required {
        return Err(PolyCommitError::InsufficientGenerators {
            required,
            available: params.generators.len(),
        });
    }
    Ok(())
}
