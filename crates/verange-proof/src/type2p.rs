use crate::type1::{Type1Proof, Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use crate::ProofError;
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;

#[derive(Clone, Debug)]
pub struct Type2PStatement {
    pub nbits: usize,
    pub k: usize,
    pub l: usize,
    pub b: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type2PWitness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug)]
pub struct Type2PProof {
    pub inner: Type1Proof,
}

pub struct Type2PProver;
pub struct Type2PVerifier;

impl Type2PProver {
    pub fn prove(
        statement: &Type2PStatement,
        witness: &Type2PWitness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type2PProof, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.k,
            tt: statement.tt,
            aggregated: statement.aggregated,
        };
        let inner_witness = Type1Witness {
            values: witness.values.clone(),
        };
        let inner = Type1Prover::prove(&inner_statement, &inner_witness, params, mode, rng)?;
        Ok(Type2PProof { inner })
    }
}

impl Type2PVerifier {
    pub fn verify(
        statement: &Type2PStatement,
        proof: &Type2PProof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.k,
            tt: statement.tt,
            aggregated: statement.aggregated,
        };
        Type1Verifier::verify(&inner_statement, &proof.inner, params, mode)
    }
}

fn validate_statement(
    statement: &Type2PStatement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if !(2..=4).contains(&statement.k) {
        return Err(ProofError::InvalidStatement(
            "Type2P currently supports K in [2, 4]",
        ));
    }
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.l != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement L must match params.gs length",
        ));
    }
    Ok(())
}
