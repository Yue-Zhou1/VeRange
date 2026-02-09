#![forbid(unsafe_code)]

pub mod commitment;
pub mod curve;
pub mod params;
pub mod scalar;
pub mod transcript;
pub mod arith;
pub mod vector;

use crate::commitment::Commitment;
use crate::curve::Scalar;
pub use crate::params::PedersenParams;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    InvalidGenerator(&'static str),
    EmptyGeneratorBasis,
    InsufficientGenerators { required: usize, available: usize },
    VectorLengthMismatch { left: usize, right: usize },
    InvalidRadix,
}

impl Display for CoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGenerator(name) => write!(f, "generator {name} must not be identity"),
            Self::EmptyGeneratorBasis => write!(f, "generator basis list must not be empty"),
            Self::InsufficientGenerators {
                required,
                available,
            } => write!(
                f,
                "insufficient generators: required {required}, available {available}"
            ),
            Self::VectorLengthMismatch { left, right } => {
                write!(f, "vector length mismatch: left {left}, right {right}")
            }
            Self::InvalidRadix => write!(f, "radix must be greater than 1"),
        }
    }
}

impl Error for CoreError {}

pub fn commit_to(params: &PedersenParams, m: Scalar, r: Scalar) -> Commitment {
    Commitment::new(params.g * m + params.h * r)
}

pub fn mul_g(params: &PedersenParams, m: Scalar) -> Commitment {
    Commitment::new(params.g * m)
}

pub fn mul_h(params: &PedersenParams, r: Scalar) -> Commitment {
    Commitment::new(params.h * r)
}

pub fn sum_commitments(commitments: &[Commitment]) -> Commitment {
    commitments
        .iter()
        .cloned()
        .reduce(|acc, item| acc.add(&item))
        .unwrap_or_else(Commitment::identity)
}
