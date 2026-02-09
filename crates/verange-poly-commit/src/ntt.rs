use crate::PolyCommitError;
use ark_ff::{Field, FftField, One, Zero};
use verange_core::curve::Scalar;

pub fn domain(size: usize) -> Result<Vec<Scalar>, PolyCommitError> {
    if size == 0 || !size.is_power_of_two() {
        return Err(PolyCommitError::InvalidDomainSize(size));
    }

    let omega =
        Scalar::get_root_of_unity(size as u64).ok_or(PolyCommitError::InvalidDomainSize(size))?;
    let mut values = Vec::with_capacity(size);
    let mut cur = Scalar::one();
    for _ in 0..size {
        values.push(cur);
        cur *= omega;
    }
    Ok(values)
}

pub fn ntt(values: &mut [Scalar], invert: bool) -> Result<(), PolyCommitError> {
    let n = values.len();
    if n == 0 {
        return Ok(());
    }
    if !n.is_power_of_two() {
        return Err(PolyCommitError::InvalidDomainSize(n));
    }

    let omega = Scalar::get_root_of_unity(n as u64).ok_or(PolyCommitError::InvalidDomainSize(n))?;
    let omega = if invert {
        omega
            .inverse()
            .ok_or(PolyCommitError::MissingInverse("ntt omega inverse"))?
    } else {
        omega
    };

    let input = values.to_vec();
    let mut out = vec![Scalar::zero(); n];

    for (k, out_k) in out.iter_mut().enumerate() {
        let mut acc = Scalar::zero();
        for (j, v) in input.iter().enumerate() {
            let exp = (j * k) as u64;
            let twiddle = omega.pow([exp]);
            acc += *v * twiddle;
        }
        *out_k = acc;
    }

    if invert {
        let n_inv = Scalar::from(n as u64)
            .inverse()
            .ok_or(PolyCommitError::MissingInverse("ntt size inverse"))?;
        for value in &mut out {
            *value *= n_inv;
        }
    }

    values.copy_from_slice(&out);
    Ok(())
}
