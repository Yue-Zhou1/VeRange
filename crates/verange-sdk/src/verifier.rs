use crate::error::SdkError;
use crate::params::Parameters;
use verange_core::transcript::TranscriptMode;
use verange_proof::type2p::{Type2PProof, Type2PStatement, Type2PVerifier};

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
}
