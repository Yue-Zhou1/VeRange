use crate::curve::Scalar;

pub fn scalar_from_u64(value: u64) -> Scalar {
    Scalar::from(value)
}
