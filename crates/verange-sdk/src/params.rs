use crate::error::SdkError;
use ark_bn254::G1Projective;
use ark_ec::Group;
use verange_core::curve::Scalar;
use verange_core::PedersenParams;

#[derive(Clone, Debug)]
pub struct Parameters {
    pub pedersen: PedersenParams,
}

impl Parameters {
    pub fn bn254_java_compat(basis_len: usize) -> Result<Self, SdkError> {
        if basis_len == 0 {
            return Err(SdkError::InvalidParameter("basis_len must be > 0"));
        }

        let g = G1Projective::generator();
        let h = g * Scalar::from(7u64);
        let gs = (0..basis_len)
            .map(|i| g * Scalar::from((i + 2) as u64))
            .collect::<Vec<_>>();

        Ok(Self {
            pedersen: PedersenParams::new(g, h, gs)?,
        })
    }
}
