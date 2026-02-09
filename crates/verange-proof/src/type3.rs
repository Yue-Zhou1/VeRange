use crate::type1::{Type1Proof, Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use crate::ProofError;
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;

#[derive(Clone, Debug)]
pub struct Type3Statement {
    pub nbits: usize,
    pub u: usize,
    pub v: usize,
    pub b: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type3Witness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug)]
pub struct Type3Proof {
    pub inner: Type1Proof,
}

pub struct Type3Prover;
pub struct Type3Verifier;

impl Type3Prover {
    pub fn prove(
        statement: &Type3Statement,
        witness: &Type3Witness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type3Proof, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.v,
            tt: statement.tt,
            aggregated: statement.aggregated,
        };
        let inner_witness = Type1Witness {
            values: witness.values.clone(),
        };
        let inner = Type1Prover::prove(&inner_statement, &inner_witness, params, mode, rng)?;
        Ok(Type3Proof { inner })
    }
}

impl Type3Verifier {
    pub fn verify(
        statement: &Type3Statement,
        proof: &Type3Proof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        let inner_statement = Type1Statement {
            nbits: statement.nbits,
            k: statement.v,
            tt: statement.tt,
            aggregated: statement.aggregated,
        };
        Type1Verifier::verify(&inner_statement, &proof.inner, params, mode)
    }
}

fn validate_statement(statement: &Type3Statement, params: &PedersenParams) -> Result<(), ProofError> {
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.u != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement U must match params.gs length",
        ));
    }
    if statement.v == 0 {
        return Err(ProofError::InvalidStatement("V must be > 0"));
    }
    Ok(())
}
