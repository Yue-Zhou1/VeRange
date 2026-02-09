use crate::error::SdkError;
use crate::params::Parameters;
use rand_core::RngCore;
use verange_core::transcript::TranscriptMode;
use verange_proof::type2p::{Type2PProof, Type2PProver, Type2PStatement, Type2PWitness};

#[derive(Clone, Debug)]
pub struct Prover {
    params: Parameters,
    mode: TranscriptMode,
}

impl Prover {
    pub fn new(params: Parameters, mode: TranscriptMode) -> Self {
        Self { params, mode }
    }

    pub fn parameters(&self) -> &Parameters {
        &self.params
    }

    pub fn prove_type2p(
        &self,
        statement: &Type2PStatement,
        witness: &Type2PWitness,
        rng: &mut impl RngCore,
    ) -> Result<Type2PProof, SdkError> {
        Ok(Type2PProver::prove(
            statement,
            witness,
            &self.params.pedersen,
            self.mode,
            rng,
        )?)
    }
}
