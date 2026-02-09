use crate::error::SdkError;
use crate::params::Parameters;
use crate::prover::Prover;
use crate::verifier::Verifier;
use crate::{
    serialize_type2p_proof, Type1Proof, Type1Statement, Type1Witness, Type2PProof, Type2PStatement,
    Type2PWitness, Type2Proof, Type2Statement, Type2Witness, Type3Proof, Type3Statement,
    Type3Witness, Type4BatchProof, Type4BatchStatement, Type4BatchWitness,
};
use rand_core::RngCore;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use verange_core::commitment::Commitment;
use verange_core::curve::Scalar;
use verange_core::transcript::{java_encode_point, java_encode_scalar, TranscriptMode};
use num_bigint::BigUint;

const LEN_PREFIX_BYTES: usize = 4;

#[derive(Clone, Debug)]
pub struct ProofMetricsRecord {
    pub proof_type: &'static str,
    pub statement: String,
    pub params_basis_len: usize,
    pub transcript_mode: TranscriptMode,
    pub proof_size_bytes: usize,
    pub prove_time: Duration,
    pub verify_time: Duration,
    pub verified: bool,
}

#[derive(Debug)]
pub enum MetricsError {
    Sdk(SdkError),
    Io(io::Error),
}

impl Display for MetricsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sdk(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl Error for MetricsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sdk(err) => Some(err),
            Self::Io(err) => Some(err),
        }
    }
}

impl From<SdkError> for MetricsError {
    fn from(value: SdkError) -> Self {
        Self::Sdk(value)
    }
}

impl From<io::Error> for MetricsError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn collect_default_proof_metrics(
    mode: TranscriptMode,
    rng: &mut impl RngCore,
) -> Result<Vec<ProofMetricsRecord>, SdkError> {
    let mut records = Vec::new();

    let params_4 = Parameters::bn254_java_compat(4)?;
    let prover_4 = Prover::new(params_4.clone(), mode);
    let verifier_4 = Verifier::new(params_4.clone(), mode);

    let type1_statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let type1_witness = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let prove_start = Instant::now();
    let type1_proof = prover_4.prove_type1(&type1_statement, &type1_witness, rng)?;
    let prove_time = prove_start.elapsed();
    let verify_start = Instant::now();
    let verified = verifier_4.verify_type1(&type1_statement, &type1_proof)?;
    let verify_time = verify_start.elapsed();
    records.push(ProofMetricsRecord {
        proof_type: "type1",
        statement: format!(
            "nbits={},k={},tt={},aggregated={}",
            type1_statement.nbits, type1_statement.k, type1_statement.tt, type1_statement.aggregated
        ),
        params_basis_len: params_4.pedersen.gs.len(),
        transcript_mode: mode,
        proof_size_bytes: estimate_type1_proof_size_bytes(&type1_proof),
        prove_time,
        verify_time,
        verified,
    });

    let type2_statement = Type2Statement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let type2_witness = Type2Witness {
        values: vec![BigUint::from(181u32), BigUint::from(77u32)],
    };
    let prove_start = Instant::now();
    let type2_proof = prover_4.prove_type2(&type2_statement, &type2_witness, rng)?;
    let prove_time = prove_start.elapsed();
    let verify_start = Instant::now();
    let verified = verifier_4.verify_type2(&type2_statement, &type2_proof)?;
    let verify_time = verify_start.elapsed();
    records.push(ProofMetricsRecord {
        proof_type: "type2",
        statement: format!(
            "nbits={},k={},l={},b={},tt={},aggregated={}",
            type2_statement.nbits,
            type2_statement.k,
            type2_statement.l,
            type2_statement.b,
            type2_statement.tt,
            type2_statement.aggregated
        ),
        params_basis_len: params_4.pedersen.gs.len(),
        transcript_mode: mode,
        proof_size_bytes: estimate_type2_proof_size_bytes(&type2_proof),
        prove_time,
        verify_time,
        verified,
    });

    let type2p_statement = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let type2p_witness = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let prove_start = Instant::now();
    let type2p_proof = prover_4.prove_type2p(&type2p_statement, &type2p_witness, rng)?;
    let prove_time = prove_start.elapsed();
    let verify_start = Instant::now();
    let verified = verifier_4.verify_type2p(&type2p_statement, &type2p_proof)?;
    let verify_time = verify_start.elapsed();
    records.push(ProofMetricsRecord {
        proof_type: "type2p",
        statement: format!(
            "nbits={},k={},l={},b={},tt={},aggregated={}",
            type2p_statement.nbits,
            type2p_statement.k,
            type2p_statement.l,
            type2p_statement.b,
            type2p_statement.tt,
            type2p_statement.aggregated
        ),
        params_basis_len: params_4.pedersen.gs.len(),
        transcript_mode: mode,
        proof_size_bytes: estimate_type2p_proof_size_bytes(&type2p_proof),
        prove_time,
        verify_time,
        verified,
    });

    let type3_statement = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let type3_witness = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let prove_start = Instant::now();
    let type3_proof = prover_4.prove_type3(&type3_statement, &type3_witness, rng)?;
    let prove_time = prove_start.elapsed();
    let verify_start = Instant::now();
    let verified = verifier_4.verify_type3(&type3_statement, &type3_proof)?;
    let verify_time = verify_start.elapsed();
    records.push(ProofMetricsRecord {
        proof_type: "type3",
        statement: format!(
            "nbits={},u={},v={},b={},tt={},aggregated={}",
            type3_statement.nbits,
            type3_statement.u,
            type3_statement.v,
            type3_statement.b,
            type3_statement.tt,
            type3_statement.aggregated
        ),
        params_basis_len: params_4.pedersen.gs.len(),
        transcript_mode: mode,
        proof_size_bytes: estimate_type3_proof_size_bytes(&type3_proof),
        prove_time,
        verify_time,
        verified,
    });

    let params_32 = Parameters::bn254_java_compat(32)?;
    let prover_32 = Prover::new(params_32.clone(), mode);
    let verifier_32 = Verifier::new(params_32.clone(), mode);

    let type4_statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let type4_witness = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let prove_start = Instant::now();
    let type4_proof = prover_32.prove_type4_batch(&type4_statement, &type4_witness, rng)?;
    let prove_time = prove_start.elapsed();
    let verify_start = Instant::now();
    let verified = verifier_32.verify_type4_batch(&type4_statement, &type4_proof)?;
    let verify_time = verify_start.elapsed();
    records.push(ProofMetricsRecord {
        proof_type: "type4_batch",
        statement: format!(
            "nbits={},k={},l={},b={}",
            type4_statement.nbits, type4_statement.k, type4_statement.l, type4_statement.b
        ),
        params_basis_len: params_32.pedersen.gs.len(),
        transcript_mode: mode,
        proof_size_bytes: estimate_type4_batch_proof_size_bytes(&type4_proof),
        prove_time,
        verify_time,
        verified,
    });

    Ok(records)
}

pub fn write_proof_metrics_log(
    path: impl AsRef<Path>,
    records: &[ProofMetricsRecord],
) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    let mut file = File::create(path)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    writeln!(file, "verange_proof_metrics_v1")?;
    writeln!(file, "generated_at_unix={timestamp}")?;
    writeln!(file, "records={}", records.len())?;

    for record in records {
        writeln!(
            file,
            "proof_type={} transcript_mode={:?} params_basis_len={} statement={} proof_size_bytes={} prove_time_us={} verify_time_us={} verified={}",
            record.proof_type,
            record.transcript_mode,
            record.params_basis_len,
            record.statement,
            record.proof_size_bytes,
            record.prove_time.as_micros(),
            record.verify_time.as_micros(),
            record.verified
        )?;
    }

    Ok(())
}

pub fn run_default_proof_metrics(
    path: impl AsRef<Path>,
    mode: TranscriptMode,
    rng: &mut impl RngCore,
) -> Result<Vec<ProofMetricsRecord>, MetricsError> {
    let records = collect_default_proof_metrics(mode, rng)?;
    write_proof_metrics_log(path, &records)?;
    Ok(records)
}

fn encode_u32(value: u32, out: &mut Vec<u8>) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn encode_usize(value: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(&(value as u64).to_be_bytes());
}

fn encode_commitment(value: &Commitment, out: &mut Vec<u8>) {
    out.extend_from_slice(&java_encode_point(value.point()));
}

fn encode_scalar(value: &Scalar, out: &mut Vec<u8>) {
    out.extend_from_slice(&java_encode_scalar(value));
}

fn encode_commitment_vec(values: &[Commitment], out: &mut Vec<u8>) {
    encode_u32(values.len() as u32, out);
    for value in values {
        encode_commitment(value, out);
    }
}

fn encode_scalar_vec(values: &[Scalar], out: &mut Vec<u8>) {
    encode_u32(values.len() as u32, out);
    for value in values {
        encode_scalar(value, out);
    }
}

fn encode_scalar_matrix(values: &[Vec<Scalar>], out: &mut Vec<u8>) {
    encode_u32(values.len() as u32, out);
    for row in values {
        encode_scalar_vec(row, out);
    }
}

fn estimate_type1_proof_size_bytes(proof: &Type1Proof) -> usize {
    let mut out = Vec::with_capacity(LEN_PREFIX_BYTES + proof.ys.len() * 64);
    encode_commitment_vec(&proof.ys, &mut out);
    encode_commitment(&proof.big_r, &mut out);
    encode_commitment(&proof.big_s, &mut out);
    encode_commitment_vec(&proof.cws, &mut out);
    encode_commitment_vec(&proof.cts, &mut out);
    encode_scalar(&proof.eta1, &mut out);
    encode_scalar(&proof.eta2, &mut out);
    encode_scalar_matrix(&proof.vs, &mut out);
    out.len()
}

fn estimate_type2_proof_size_bytes(proof: &Type2Proof) -> usize {
    let mut out = Vec::new();
    encode_commitment_vec(&proof.ys, &mut out);
    encode_commitment(&proof.big_r, &mut out);
    encode_commitment(&proof.big_s, &mut out);
    encode_commitment_vec(&proof.cws, &mut out);
    encode_commitment_vec(&proof.cms, &mut out);
    encode_commitment_vec(&proof.cvk, &mut out);
    encode_commitment_vec(&proof.ctk, &mut out);
    encode_scalar(&proof.eta1, &mut out);
    encode_scalar(&proof.eta2, &mut out);
    encode_scalar(&proof.eta3, &mut out);
    encode_scalar_matrix(&proof.vs, &mut out);
    encode_scalar_matrix(&proof.us, &mut out);
    out.len()
}

fn estimate_type2p_proof_size_bytes(proof: &Type2PProof) -> usize {
    serialize_type2p_proof(proof).len()
}

fn estimate_type3_proof_size_bytes(proof: &Type3Proof) -> usize {
    let mut out = Vec::new();
    encode_commitment_vec(&proof.witness_commitments, &mut out);
    encode_commitment_vec(&proof.c_d, &mut out);
    encode_commitment_vec(&proof.c_w, &mut out);
    encode_commitment(&proof.c_r1, &mut out);
    encode_commitment(&proof.c_r2, &mut out);
    encode_commitment_vec(&proof.cm_s, &mut out);
    encode_commitment_vec(&proof.cm_b, &mut out);
    encode_scalar_vec(&proof.pi_sx, &mut out);
    encode_scalar_vec(&proof.pi_bx, &mut out);
    encode_usize(proof.sx_degree, &mut out);
    encode_usize(proof.bx_degree, &mut out);
    encode_scalar(&proof.y_s, &mut out);
    encode_scalar(&proof.y_b, &mut out);
    encode_scalar(&proof.eta1, &mut out);
    encode_scalar(&proof.eta2, &mut out);
    encode_scalar_vec(&proof.djx, &mut out);
    encode_scalar_vec(&proof.bjx, &mut out);
    encode_scalar_vec(&proof.zs, &mut out);
    encode_scalar(&proof.x, &mut out);
    out.len()
}

fn estimate_type4_batch_proof_size_bytes(proof: &Type4BatchProof) -> usize {
    let mut out = Vec::new();
    encode_commitment(&proof.w, &mut out);
    encode_commitment(&proof.cm_a, &mut out);
    encode_commitment(&proof.cm_e, &mut out);
    encode_commitment(&proof.cm_aprime, &mut out);
    encode_commitment(&proof.cm_eprime, &mut out);
    encode_commitment_vec(&proof.com_g, &mut out);
    encode_commitment_vec(&proof.com_p, &mut out);
    encode_commitment_vec(&proof.com_q, &mut out);
    encode_scalar(&proof.b, &mut out);
    encode_scalar(&proof.eprime, &mut out);
    encode_scalar(&proof.g_eval_x, &mut out);
    encode_scalar(&proof.g_eval_wx, &mut out);
    encode_scalar(&proof.f_eval_x, &mut out);
    encode_scalar(&proof.fprime_eval_x, &mut out);
    encode_scalar(&proof.p_eval_x, &mut out);
    encode_scalar(&proof.rprime_1, &mut out);
    encode_scalar(&proof.rprime_2, &mut out);
    encode_scalar(&proof.rprime_3, &mut out);
    encode_scalar(&proof.rprime_4, &mut out);
    encode_scalar_vec(&proof.qv, &mut out);
    encode_usize(proof.align_degree, &mut out);
    out.len()
}
