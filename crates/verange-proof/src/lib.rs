#![forbid(unsafe_code)]

pub mod type1;
pub mod type2;
pub mod type2p;
pub mod type3;

use std::error::Error;
use std::fmt::{Display, Formatter};
use verange_core::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofError {
    InvalidStatement(&'static str),
    InvalidWitness(&'static str),
    InvalidProof(&'static str),
    Core(CoreError),
}

impl Display for ProofError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatement(msg) => write!(f, "invalid statement: {msg}"),
            Self::InvalidWitness(msg) => write!(f, "invalid witness: {msg}"),
            Self::InvalidProof(msg) => write!(f, "invalid proof: {msg}"),
            Self::Core(e) => write!(f, "{e}"),
        }
    }
}

impl Error for ProofError {}

impl From<CoreError> for ProofError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}
