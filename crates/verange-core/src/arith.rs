use crate::CoreError;
use num_bigint::BigUint;

pub fn decompose_to_nary(value: &BigUint, base: &BigUint) -> Result<Vec<BigUint>, CoreError> {
    if base <= &BigUint::from(1u8) {
        return Err(CoreError::InvalidRadix);
    }

    let mut remaining = value.clone();
    let mut digits = Vec::new();

    while remaining > BigUint::default() {
        let digit = &remaining % base;
        digits.push(digit);
        remaining /= base;
    }

    Ok(digits)
}

pub fn decompose_to_nary_padded(
    value: &BigUint,
    base: &BigUint,
    size: usize,
) -> Result<Vec<BigUint>, CoreError> {
    let mut digits = decompose_to_nary(value, base)?;
    while digits.len() < size {
        digits.push(BigUint::default());
    }
    Ok(digits)
}
