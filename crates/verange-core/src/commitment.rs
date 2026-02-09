use crate::curve::{CurvePoint, Scalar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commitment {
    point: CurvePoint,
}

impl Commitment {
    pub fn new(point: CurvePoint) -> Self {
        Self { point }
    }

    pub fn point(&self) -> &CurvePoint {
        &self.point
    }

    pub fn identity() -> Self {
        Self::new(CurvePoint::default())
    }

    pub fn add(&self, other: &Self) -> Self {
        Self::new(self.point + other.point)
    }

    pub fn mul_scalar(&self, scalar: Scalar) -> Self {
        Self::new(self.point * scalar)
    }
}
