#![forbid(unsafe_code)]

pub mod error;
pub mod params;
pub mod prover;
pub mod verifier;

use ark_bn254::{Fq, G1Affine};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use error::SdkError;
use verange_core::commitment::Commitment;
use verange_core::curve::Scalar;
use verange_core::transcript::{java_encode_point, java_encode_scalar};
pub use verange_proof::type1::{Type1Proof, Type1Statement, Type1Witness};
pub use verange_proof::type2::{Type2Proof, Type2Statement, Type2Witness};
pub use verange_proof::type2p::{Type2PProof, Type2PStatement, Type2PWitness};
pub use verange_proof::type3::{Type3Proof, Type3Statement, Type3Witness};
pub use verange_proof::type4_batch::{Type4BatchProof, Type4BatchStatement, Type4BatchWitness};

pub use error::SdkError as VerangeSdkError;
pub use params::Parameters;
pub use prover::Prover;
pub use verifier::Verifier;

pub fn serialize_type2p_proof(proof: &Type2PProof) -> Vec<u8> {
    let mut out = Vec::new();
    encode_commitment_vec(&proof.ys, &mut out);
    encode_commitment(&proof.big_r, &mut out);
    encode_commitment(&proof.big_s, &mut out);
    encode_commitment(&proof.big_u, &mut out);
    encode_commitment_vec(&proof.cws, &mut out);
    encode_commitment_vec(&proof.cms, &mut out);
    encode_commitment_vec(&proof.cfk, &mut out);
    encode_commitment_vec(&proof.ctk, &mut out);
    encode_commitment_vec(&proof.ctk_kprime, &mut out);
    encode_scalar(&proof.eta1, &mut out);
    encode_scalar(&proof.eta2, &mut out);
    encode_scalar(&proof.eta3, &mut out);
    encode_scalar(&proof.eta4, &mut out);
    encode_scalar_vec(&proof.vs, &mut out);
    encode_scalar_vec(&proof.us, &mut out);
    out
}

pub fn deserialize_type2p_proof(bytes: &[u8]) -> Result<Type2PProof, SdkError> {
    let mut reader = Reader::new(bytes);

    let ys = decode_commitment_vec(&mut reader)?;
    let big_r = decode_commitment(&mut reader)?;
    let big_s = decode_commitment(&mut reader)?;
    let big_u = decode_commitment(&mut reader)?;
    let cws = decode_commitment_vec(&mut reader)?;
    let cms = decode_commitment_vec(&mut reader)?;
    let cfk = decode_commitment_vec(&mut reader)?;
    let ctk = decode_commitment_vec(&mut reader)?;
    let ctk_kprime = decode_commitment_vec(&mut reader)?;
    let eta1 = decode_scalar(&mut reader)?;
    let eta2 = decode_scalar(&mut reader)?;
    let eta3 = decode_scalar(&mut reader)?;
    let eta4 = decode_scalar(&mut reader)?;
    let vs = decode_scalar_vec(&mut reader)?;
    let us = decode_scalar_vec(&mut reader)?;

    if !reader.is_finished() {
        return Err(SdkError::Deserialize("trailing bytes in proof payload"));
    }

    Ok(Type2PProof {
        ys,
        big_r,
        big_s,
        big_u,
        cws,
        cms,
        cfk,
        ctk,
        ctk_kprime,
        eta1,
        eta2,
        eta3,
        eta4,
        vs,
        us,
    })
}

fn encode_commitment_vec(values: &[Commitment], out: &mut Vec<u8>) {
    encode_u32(values.len() as u32, out);
    for value in values {
        encode_commitment(value, out);
    }
}

fn decode_commitment_vec(reader: &mut Reader<'_>) -> Result<Vec<Commitment>, SdkError> {
    let len = decode_u32(reader)? as usize;
    let mut values = Vec::with_capacity(len);
    for _ in 0..len {
        values.push(decode_commitment(reader)?);
    }
    Ok(values)
}

fn encode_scalar_vec(values: &[Scalar], out: &mut Vec<u8>) {
    encode_u32(values.len() as u32, out);
    for value in values {
        encode_scalar(value, out);
    }
}

fn decode_scalar_vec(reader: &mut Reader<'_>) -> Result<Vec<Scalar>, SdkError> {
    let len = decode_u32(reader)? as usize;
    let mut values = Vec::with_capacity(len);
    for _ in 0..len {
        values.push(decode_scalar(reader)?);
    }
    Ok(values)
}

fn encode_commitment(value: &Commitment, out: &mut Vec<u8>) {
    out.extend_from_slice(&java_encode_point(value.point()));
}

fn decode_commitment(reader: &mut Reader<'_>) -> Result<Commitment, SdkError> {
    let bytes = reader.take(64)?;
    let x_bytes = &bytes[..32];
    let y_bytes = &bytes[32..64];

    if x_bytes.iter().all(|b| *b == 0) && y_bytes.iter().all(|b| *b == 0) {
        return Ok(Commitment::identity());
    }

    let x = Fq::from_be_bytes_mod_order(x_bytes);
    let y = Fq::from_be_bytes_mod_order(y_bytes);
    let affine = G1Affine::new_unchecked(x, y);
    if !affine.is_on_curve() {
        return Err(SdkError::Deserialize(
            "invalid commitment point (off-curve)",
        ));
    }
    if !affine.is_in_correct_subgroup_assuming_on_curve() {
        return Err(SdkError::Deserialize(
            "invalid commitment point (wrong subgroup)",
        ));
    }
    Ok(Commitment::new(affine.into_group()))
}

fn encode_scalar(value: &Scalar, out: &mut Vec<u8>) {
    out.extend_from_slice(&java_encode_scalar(value));
}

fn decode_scalar(reader: &mut Reader<'_>) -> Result<Scalar, SdkError> {
    let bytes = reader.take(32)?;
    Ok(Scalar::from_be_bytes_mod_order(bytes))
}

fn encode_u32(value: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn decode_u32(reader: &mut Reader<'_>) -> Result<u32, SdkError> {
    let bytes = reader.take(4)?;
    let mut arr = [0u8; 4];
    arr.copy_from_slice(bytes);
    Ok(u32::from_be_bytes(arr))
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8], SdkError> {
        if self.offset + count > self.bytes.len() {
            return Err(SdkError::Deserialize("unexpected end of byte stream"));
        }
        let start = self.offset;
        self.offset += count;
        Ok(&self.bytes[start..self.offset])
    }

    fn is_finished(&self) -> bool {
        self.offset == self.bytes.len()
    }
}
