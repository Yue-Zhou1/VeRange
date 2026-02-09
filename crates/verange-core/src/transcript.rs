use crate::curve::{CurvePoint, Scalar};
use ark_ec::CurveGroup;
use ark_ff::{BigInteger, PrimeField};
use tiny_keccak::{Hasher, Keccak};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TranscriptMode {
    Canonical,
    JavaCompat,
}

#[derive(Clone, Debug)]
pub struct Transcript {
    mode: TranscriptMode,
    state: Vec<u8>,
}

impl Transcript {
    pub fn new(label: &[u8], mode: TranscriptMode) -> Self {
        let mut state = Vec::new();
        state.extend_from_slice(b"verange-transcript-v1");
        append_with_label(&mut state, b"domain", label);
        Self { mode, state }
    }

    pub fn append_scalar(&mut self, label: &[u8], scalar: &Scalar) {
        let bytes = match self.mode {
            TranscriptMode::Canonical => java_encode_scalar(scalar).to_vec(),
            TranscriptMode::JavaCompat => java_encode_scalar(scalar).to_vec(),
        };
        append_with_label(&mut self.state, label, &bytes);
    }

    pub fn append_point(&mut self, label: &[u8], point: &CurvePoint) {
        let bytes = match self.mode {
            TranscriptMode::Canonical => java_encode_point(point),
            TranscriptMode::JavaCompat => java_encode_point(point),
        };
        append_with_label(&mut self.state, label, &bytes);
    }

    pub fn challenge_scalar(&self, label: &[u8]) -> Scalar {
        let mut input = self.state.clone();
        append_with_label(&mut input, b"challenge", label);
        let digest = keccak256(&input);
        Scalar::from_be_bytes_mod_order(&digest)
    }
}

pub fn java_encode_scalar(scalar: &Scalar) -> Vec<u8> {
    left_pad_32(scalar.into_bigint().to_bytes_be())
}

pub fn java_encode_point(point: &CurvePoint) -> Vec<u8> {
    if point == &CurvePoint::default() {
        return vec![0u8; 64];
    }

    let affine = point.into_affine();
    let mut out = Vec::with_capacity(64);
    out.extend_from_slice(&left_pad_32(affine.x.into_bigint().to_bytes_be()));
    out.extend_from_slice(&left_pad_32(affine.y.into_bigint().to_bytes_be()));
    out
}

fn append_with_label(state: &mut Vec<u8>, label: &[u8], bytes: &[u8]) {
    state.extend_from_slice(&(label.len() as u32).to_be_bytes());
    state.extend_from_slice(label);
    state.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    state.extend_from_slice(bytes);
}

fn keccak256(bytes: &[u8]) -> [u8; 32] {
    let mut keccak = Keccak::v256();
    keccak.update(bytes);

    let mut out = [0u8; 32];
    keccak.finalize(&mut out);
    out
}

fn left_pad_32(mut bytes: Vec<u8>) -> Vec<u8> {
    if bytes.len() > 32 {
        bytes = bytes[bytes.len() - 32..].to_vec();
    }
    if bytes.len() == 32 {
        return bytes;
    }

    let mut out = vec![0u8; 32];
    let start = 32 - bytes.len();
    out[start..].copy_from_slice(&bytes);
    out
}
