use crate::error::SdkError;
use crate::params::Parameters;
use verange_core::transcript::TranscriptMode;
use verange_proof::type1::{Type1Proof, Type1Statement, Type1Verifier};
use verange_proof::type2::{Type2Proof, Type2Statement, Type2Verifier};
use verange_proof::type2p::{Type2PProof, Type2PStatement, Type2PVerifier};
use verange_proof::type3::{Type3Proof, Type3Statement, Type3Verifier};
use verange_proof::type4_batch::{Type4BatchProof, Type4BatchStatement, Type4BatchVerifier};

#[derive(Clone, Debug)]
pub struct Verifier {
    params: Parameters,
    mode: TranscriptMode,
}

impl Verifier {
    pub fn new(params: Parameters, mode: TranscriptMode) -> Self {
        Self { params, mode }
    }

    pub fn verify_type2p(
        &self,
        statement: &Type2PStatement,
        proof: &Type2PProof,
    ) -> Result<bool, SdkError> {
        Ok(Type2PVerifier::verify(
            statement,
            proof,
            &self.params.pedersen,
            self.mode,
        )?)
    }

    pub fn verify_type1(
        &self,
        statement: &Type1Statement,
        proof: &Type1Proof,
    ) -> Result<bool, SdkError> {
        Ok(Type1Verifier::verify(
            statement,
            proof,
            &self.params.pedersen,
            self.mode,
        )?)
    }

    pub fn verify_type2(
        &self,
        statement: &Type2Statement,
        proof: &Type2Proof,
    ) -> Result<bool, SdkError> {
        Ok(Type2Verifier::verify(
            statement,
            proof,
            &self.params.pedersen,
            self.mode,
        )?)
    }

    pub fn verify_type3(
        &self,
        statement: &Type3Statement,
        proof: &Type3Proof,
    ) -> Result<bool, SdkError> {
        Ok(Type3Verifier::verify(
            statement,
            proof,
            &self.params.pedersen,
            self.mode,
        )?)
    }

    pub fn verify_type4_batch(
        &self,
        statement: &Type4BatchStatement,
        proof: &Type4BatchProof,
    ) -> Result<bool, SdkError> {
        Ok(Type4BatchVerifier::verify(
            statement,
            proof,
            &self.params.pedersen,
            self.mode,
        )?)
    }
}
