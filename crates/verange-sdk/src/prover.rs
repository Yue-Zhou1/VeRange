use crate::error::SdkError;
use crate::params::Parameters;
use rand_core::RngCore;
use verange_core::transcript::TranscriptMode;
use verange_proof::type1::{Type1Proof, Type1Prover, Type1Statement, Type1Witness};
use verange_proof::type2::{Type2Proof, Type2Prover, Type2Statement, Type2Witness};
use verange_proof::type2p::{Type2PProof, Type2PProver, Type2PStatement, Type2PWitness};
use verange_proof::type3::{Type3Proof, Type3Prover, Type3Statement, Type3Witness};
use verange_proof::type4_batch::{
    Type4BatchProof, Type4BatchProver, Type4BatchStatement, Type4BatchWitness,
};

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

    pub fn prove_type1(
        &self,
        statement: &Type1Statement,
        witness: &Type1Witness,
        rng: &mut impl RngCore,
    ) -> Result<Type1Proof, SdkError> {
        Ok(Type1Prover::prove(
            statement,
            witness,
            &self.params.pedersen,
            self.mode,
            rng,
        )?)
    }

    pub fn prove_type2(
        &self,
        statement: &Type2Statement,
        witness: &Type2Witness,
        rng: &mut impl RngCore,
    ) -> Result<Type2Proof, SdkError> {
        Ok(Type2Prover::prove(
            statement,
            witness,
            &self.params.pedersen,
            self.mode,
            rng,
        )?)
    }

    pub fn prove_type3(
        &self,
        statement: &Type3Statement,
        witness: &Type3Witness,
        rng: &mut impl RngCore,
    ) -> Result<Type3Proof, SdkError> {
        Ok(Type3Prover::prove(
            statement,
            witness,
            &self.params.pedersen,
            self.mode,
            rng,
        )?)
    }

    pub fn prove_type4_batch(
        &self,
        statement: &Type4BatchStatement,
        witness: &Type4BatchWitness,
        rng: &mut impl RngCore,
    ) -> Result<Type4BatchProof, SdkError> {
        Ok(Type4BatchProver::prove(
            statement,
            witness,
            &self.params.pedersen,
            self.mode,
            rng,
        )?)
    }
}
