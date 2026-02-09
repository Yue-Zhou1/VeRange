use crate::curve::CurvePoint;
use crate::CoreError;

#[derive(Clone, Debug)]
pub struct PedersenParams {
    pub g: CurvePoint,
    pub h: CurvePoint,
    pub gs: Vec<CurvePoint>,
}

impl PedersenParams {
    pub fn new(g: CurvePoint, h: CurvePoint, gs: Vec<CurvePoint>) -> Result<Self, CoreError> {
        if g == CurvePoint::default() {
            return Err(CoreError::InvalidGenerator("g"));
        }
        if h == CurvePoint::default() {
            return Err(CoreError::InvalidGenerator("h"));
        }
        if gs.is_empty() {
            return Err(CoreError::EmptyGeneratorBasis);
        }

        Ok(Self { g, h, gs })
    }

    pub fn require_generators(&self, required: usize) -> Result<(), CoreError> {
        if self.gs.len() < required {
            return Err(CoreError::InsufficientGenerators {
                required,
                available: self.gs.len(),
            });
        }
        Ok(())
    }
}
