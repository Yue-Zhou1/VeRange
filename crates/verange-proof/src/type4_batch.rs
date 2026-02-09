use crate::type1::{Type1Proof, Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use crate::ProofError;
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;

#[derive(Clone, Debug)]
pub struct Type4BatchStatement {
    pub nbits: usize,
    pub k: usize,
    pub l: usize,
    pub b: usize,
}

#[derive(Clone, Debug)]
pub struct Type4BatchWitness {
    pub value: BigUint,
}

#[derive(Clone, Debug)]
pub struct Type4BatchProof {
    pub inner: Type1Proof,
}

pub struct Type4BatchProver;
pub struct Type4BatchVerifier;

impl Type4BatchProver {
    pub fn prove(
        statement: &Type4BatchStatement,
        witness: &Type4BatchWitness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type4BatchProof, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.k,
            tt: 1,
            aggregated: false,
        };
        let inner_witness = Type1Witness {
            values: vec![witness.value.clone()],
        };
        let inner = Type1Prover::prove(&inner_statement, &inner_witness, params, mode, rng)?;
        Ok(Type4BatchProof { inner })
    }
}

impl Type4BatchVerifier {
    pub fn verify(
        statement: &Type4BatchStatement,
        proof: &Type4BatchProof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.k,
            tt: 1,
            aggregated: false,
        };
        Type1Verifier::verify(&inner_statement, &proof.inner, params, mode)
    }
}

fn validate_statement(
    statement: &Type4BatchStatement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.k == 0 {
        return Err(ProofError::InvalidStatement("K must be > 0"));
    }
    if statement.l != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement L must match params.gs length",
        ));
    }
    Ok(())
}
