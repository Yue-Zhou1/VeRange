use std::error::Error;
use std::fmt::{Display, Formatter};
use verange_core::CoreError;
use verange_proof::ProofError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SdkError {
    InvalidParameter(&'static str),
    Deserialize(&'static str),
    Core(CoreError),
    Proof(ProofError),
}

impl Display for SdkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter(msg) => write!(f, "invalid parameter: {msg}"),
            Self::Deserialize(msg) => write!(f, "deserialize error: {msg}"),
            Self::Core(e) => write!(f, "{e}"),
            Self::Proof(e) => write!(f, "{e}"),
        }
    }
}

impl Error for SdkError {}

impl From<CoreError> for SdkError {
    fn from(value: CoreError) -> Self {
        Self::Core(value)
    }
}

impl From<ProofError> for SdkError {
    fn from(value: ProofError) -> Self {
        Self::Proof(value)
    }
}
