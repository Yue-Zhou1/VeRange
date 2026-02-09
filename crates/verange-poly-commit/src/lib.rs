#![forbid(unsafe_code)]

pub mod commit;
pub mod ntt;
pub mod polynomial;

use std::error::Error;
use std::fmt::{Display, Formatter};
use verange_core::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolyCommitError {
    InvalidDimensions(&'static str),
    DivisionByZeroPolynomial,
    MissingInverse(&'static str),
    InsufficientGenerators { required: usize, available: usize },
    InvalidDomainSize(usize),
    Core(CoreError),
}

impl Display for PolyCommitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDimensions(msg) => write!(f, "invalid dimensions: {msg}"),
            Self::DivisionByZeroPolynomial => write!(f, "division by zero polynomial"),
            Self::MissingInverse(msg) => write!(f, "inverse does not exist: {msg}"),
            Self::InsufficientGenerators {
                required,
                available,
            } => write!(
                f,
                "insufficient generators: required {required}, available {available}"
            ),
            Self::InvalidDomainSize(size) => {
                write!(f, "domain size must be a non-zero power of two, got {size}")
            }
            Self::Core(e) => write!(f, "{e}"),
        }
    }
}

impl Error for PolyCommitError {}

impl From<CoreError> for PolyCommitError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}
