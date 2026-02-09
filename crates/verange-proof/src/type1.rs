use crate::ProofError;
use ark_ff::{Field, PrimeField};
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::commit_to;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};
use verange_core::transcript::{Transcript, TranscriptMode};
use verange_core::{sum_commitments, PedersenParams};

#[derive(Clone, Debug)]
pub struct Type1Statement {
    pub nbits: usize,
    pub k: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type1Witness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug)]
pub struct Type1Proof {
    pub ys: Vec<Commitment>,
    pub big_r: Commitment,
    pub big_s: Commitment,
    pub cws: Vec<Commitment>,
    pub cts: Vec<Commitment>,
    pub eta1: Scalar,
    pub eta2: Scalar,
    pub vs: Vec<Vec<Scalar>>,
}

pub struct Type1Prover;
pub struct Type1Verifier;

impl Type1Prover {
    pub fn prove(
        statement: &Type1Statement,
        witness: &Type1Witness,
        params: &PedersenParams,
        transcript_mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type1Proof, ProofError> {
        validate_statement(statement, params)?;
        validate_witness(statement, witness)?;

        let l = params.gs.len();
        let k = statement.k;
        let tt = statement.tt;

        let witness_count = if statement.aggregated { tt } else { 1 };
        let mut rcm = Vec::with_capacity(witness_count);
        let mut ys = Vec::with_capacity(witness_count);
        for i in 0..witness_count {
            let r = random_scalar(rng);
            rcm.push(r);
            ys.push(commit_to(params, biguint_to_scalar(&witness.values[i]), r));
        }

        let gamma = challenge_gamma(&ys, transcript_mode);

        let mut rs = vec![vec![Scalar::from(0u64); k]; l];
        let mut rk_sum = vec![Scalar::from(0u64); l];
        for j in 0..l {
            let mut sum = Scalar::from(0u64);
            for i in 0..k {
                let r = random_scalar(rng);
                rs[j][i] = r;
                sum += r;
            }
            rk_sum[j] = sum;
        }

        let gamma_r = random_scalar(rng);
        let gamma_s = random_scalar(rng);

        let gamma_t = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let mut gamma_w = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let sum_gamma_w = gamma_w[..k - 1]
            .iter()
            .fold(Scalar::from(0u64), |acc, v| acc + *v);
        let gamma_target = if statement.aggregated {
            (0..tt).fold(Scalar::from(0u64), |acc, i| {
                acc + pow_usize(gamma, i + 1) * rcm[i]
            })
        } else {
            rcm[0]
        };
        gamma_w[k - 1] = gamma_target - sum_gamma_w;

        let mut bs = vec![vec![Scalar::from(0u64); k]; l];
        let mut bsr = vec![vec![Scalar::from(0u64); l]; k];
        let mut ds = vec![vec![Scalar::from(0u64); l]; k];

        fill_bits_matrices(statement, witness, gamma, &mut bs, &mut bsr, &mut ds);

        let mut cws = Vec::with_capacity(k);
        let mut cts = Vec::with_capacity(k);
        for i in 0..k {
            let ds_sum = ds[i]
                .iter()
                .fold(Scalar::from(0u64), |acc, value| acc + *value);
            cws.push(commit_to(params, ds_sum, gamma_w[i]));

            let mut ts = vec![Scalar::from(0u64); l];
            for j in 0..l {
                ts[j] = rs[j][i] * (bsr[i][j] - Scalar::from(2u64) * ds[i][j]);
            }
            cts.push(commit_with_basis_and_h(&params.gs, &ts, &params.h, gamma_t[i])?);
        }

        let mut rjs = vec![Scalar::from(0u64); l];
        for j in 0..l {
            let mut sum = Scalar::from(0u64);
            for i in 0..k {
                sum += rs[j][i] * rs[j][i];
            }
            rjs[j] = -sum;
        }

        let big_r_message = rk_sum
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let big_r = commit_to(params, big_r_message, gamma_r);
        let big_s = commit_with_basis_and_h(&params.gs, &rjs, &params.h, gamma_s)?;

        let beta = challenge_beta(&cts, &cws, &big_r, &big_s, transcript_mode);
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();

        let eta1 = inner_product(&gamma_t, &cl_es)? + gamma_s;
        let eta2 = inner_product(&gamma_w, &cl_es)? + gamma_r;

        let mut vs = vec![vec![Scalar::from(0u64); k]; l];
        for j in 0..l {
            for i in 0..k {
                vs[j][i] = bs[j][i] * cl_es[i] + rs[j][i];
            }
        }

        Ok(Type1Proof {
            ys,
            big_r,
            big_s,
            cws,
            cts,
            eta1,
            eta2,
            vs,
        })
    }
}

impl Type1Verifier {
    pub fn verify(
        statement: &Type1Statement,
        proof: &Type1Proof,
        params: &PedersenParams,
        transcript_mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        validate_proof_shape(statement, proof, params)?;

        let l = params.gs.len();
        let k = statement.k;
        let tt = statement.tt;

        let gamma = challenge_gamma(&proof.ys, transcript_mode);

        let mut bsr = vec![vec![Scalar::from(0u64); k]; l];
        let lk = l * k;
        if tt == 1 {
            for i in 0..lk {
                let idx = i / k;
                let mod_idx = i % k;
                bsr[idx][mod_idx] = if i < statement.nbits {
                    pow_usize(Scalar::from(2u64), i)
                } else {
                    Scalar::from(0u64)
                };
            }
        } else {
            let actual_mod = if k % tt == 0 { k / tt } else { k / tt + 1 } * l;
            let mut tt_counter = 0usize;
            for i in 0..lk {
                let idx = i / k;
                let mod_idx = i % k;
                let mod_agg = i % actual_mod;
                if mod_agg == 0 {
                    tt_counter += 1;
                }
                let in_range = i < statement.nbits / tt + (tt_counter - 1) * actual_mod
                    && i >= (tt_counter - 1) * actual_mod;
                bsr[idx][mod_idx] = if in_range {
                    pow_usize(gamma, tt_counter) * pow_usize(Scalar::from(2u64), mod_agg)
                } else {
                    Scalar::from(0u64)
                };
            }
        }

        let beta = challenge_beta(
            &proof.cts,
            &proof.cws,
            &proof.big_r,
            &proof.big_s,
            transcript_mode,
        );
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();

        let mut udotvs = vec![Scalar::from(0u64); l];
        let mut vsum = Scalar::from(0u64);
        for j in 0..l {
            let mut us = vec![Scalar::from(0u64); k];
            for i in 0..k {
                us[i] = bsr[j][i] * cl_es[i] - proof.vs[j][i];
                vsum += proof.vs[j][i];
            }
            udotvs[j] = inner_product(&us, &proof.vs[j])?;
        }

        let eq1_lhs = commit_with_basis_and_h(&params.gs, &udotvs, &params.h, proof.eta1)?;
        let mut eq1_rhs = Commitment::identity();
        for (idx, c) in proof.cts.iter().enumerate() {
            eq1_rhs = eq1_rhs.add(&c.mul_scalar(cl_es[idx]));
        }
        eq1_rhs = eq1_rhs.add(&proof.big_s);
        let b1 = eq1_lhs == eq1_rhs;

        let eq2_lhs = commit_to(params, vsum, proof.eta2);
        let mut eq2_rhs = Commitment::identity();
        for (idx, c) in proof.cws.iter().enumerate() {
            eq2_rhs = eq2_rhs.add(&c.mul_scalar(cl_es[idx]));
        }
        eq2_rhs = eq2_rhs.add(&proof.big_r);
        let b2 = eq2_lhs == eq2_rhs;

        let sum_cws = sum_commitments(&proof.cws);
        let b3 = if tt == 1 {
            proof.ys[0] == sum_cws
        } else {
            let mut agg = Commitment::identity();
            for i in 0..tt {
                agg = agg.add(&proof.ys[i].mul_scalar(pow_usize(gamma, i + 1)));
            }
            agg == sum_cws
        };

        Ok(b1 && b2 && b3)
    }
}

fn validate_statement(statement: &Type1Statement, params: &PedersenParams) -> Result<(), ProofError> {
    if statement.nbits == 0 {
        return Err(ProofError::InvalidStatement("nbits must be > 0"));
    }
    if statement.k == 0 {
        return Err(ProofError::InvalidStatement("k must be > 0"));
    }
    if params.gs.is_empty() {
        return Err(ProofError::InvalidStatement("generator basis must be non-empty"));
    }
    if params.gs.len() * statement.k < statement.nbits {
        return Err(ProofError::InvalidStatement("L*K must be >= nbits"));
    }
    if statement.aggregated && statement.tt <= 1 {
        return Err(ProofError::InvalidStatement(
            "aggregated mode requires tt > 1",
        ));
    }
    if statement.aggregated && statement.nbits % statement.tt != 0 {
        return Err(ProofError::InvalidStatement(
            "nbits must be divisible by tt in aggregated mode",
        ));
    }
    Ok(())
}

fn validate_witness(statement: &Type1Statement, witness: &Type1Witness) -> Result<(), ProofError> {
    if witness.values.is_empty() {
        return Err(ProofError::InvalidWitness("at least one witness value is required"));
    }
    let expected = if statement.aggregated { statement.tt } else { 1 };
    if witness.values.len() != expected {
        return Err(ProofError::InvalidWitness(
            "witness values length does not match statement",
        ));
    }
    Ok(())
}

fn validate_proof_shape(
    statement: &Type1Statement,
    proof: &Type1Proof,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    let l = params.gs.len();
    let k = statement.k;
    let expected_ys = if statement.aggregated { statement.tt } else { 1 };

    if proof.ys.len() != expected_ys {
        return Err(ProofError::InvalidProof("invalid Y commitments length"));
    }
    if proof.cws.len() != k || proof.cts.len() != k {
        return Err(ProofError::InvalidProof("invalid cws/cts length"));
    }
    if proof.vs.len() != l || proof.vs.iter().any(|row| row.len() != k) {
        return Err(ProofError::InvalidProof("invalid vs matrix shape"));
    }
    Ok(())
}

fn fill_bits_matrices(
    statement: &Type1Statement,
    witness: &Type1Witness,
    gamma: Scalar,
    bs: &mut [Vec<Scalar>],
    bsr: &mut [Vec<Scalar>],
    ds: &mut [Vec<Scalar>],
) {
    let l = bs.len();
    let k = statement.k;
    let lk = l * k;

    if !statement.aggregated {
        let w0 = &witness.values[0];
        for i in 0..lk {
            let idx = i / k;
            let mod_idx = i % k;
            if i < statement.nbits {
                let b = if test_bit(w0, i) {
                    Scalar::from(1u64)
                } else {
                    Scalar::from(0u64)
                };
                let twopowers = pow_usize(Scalar::from(2u64), i);
                let w = b * twopowers;
                bs[idx][mod_idx] = w;
                bsr[mod_idx][idx] = twopowers;
                ds[mod_idx][idx] = w;
            }
        }
        return;
    }

    let tt = statement.tt;
    let actual_mod = if k % tt == 0 { k / tt } else { k / tt + 1 } * l;
    let mut tt_counter = 0usize;
    for i in 0..lk {
        let idx = i / k;
        let mod_idx = i % k;
        let mod_agg = i % actual_mod;
        if mod_agg == 0 {
            tt_counter += 1;
        }
        let witness_idx = tt_counter - 1;
        let b = if test_bit(&witness.values[witness_idx], mod_agg) {
            Scalar::from(1u64)
        } else {
            Scalar::from(0u64)
        };

        let in_range = i < statement.nbits / tt + (tt_counter - 1) * actual_mod
            && i >= (tt_counter - 1) * actual_mod;
        if in_range {
            let twopowers = pow_usize(gamma, tt_counter) * pow_usize(Scalar::from(2u64), mod_agg);
            let w = b * twopowers;
            bs[idx][mod_idx] = w;
            bsr[mod_idx][idx] = twopowers;
            ds[mod_idx][idx] = w;
        }
    }
}

fn challenge_gamma(ys: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type1-gamma", mode);
    for commitment in ys {
        transcript.append_point(b"Y", commitment.point());
    }
    transcript.challenge_scalar(b"gamma")
}

fn challenge_beta(
    cts: &[Commitment],
    cws: &[Commitment],
    big_r: &Commitment,
    big_s: &Commitment,
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type1-beta", mode);
    for c in cts {
        transcript.append_point(b"ct", c.point());
    }
    for c in cws {
        transcript.append_point(b"cw", c.point());
    }
    transcript.append_point(b"R", big_r.point());
    transcript.append_point(b"S", big_s.point());
    transcript.challenge_scalar(b"beta")
}

fn commit_with_basis_and_h(
    basis: &[CurvePoint],
    coeffs: &[Scalar],
    h: &CurvePoint,
    r: Scalar,
) -> Result<Commitment, ProofError> {
    if basis.len() != coeffs.len() {
        return Err(ProofError::InvalidStatement(
            "basis and coefficient lengths must match",
        ));
    }

    let mut point = *h * r;
    for (g, c) in basis.iter().zip(coeffs.iter()) {
        point += *g * *c;
    }
    Ok(Commitment::new(point))
}

fn inner_product(a: &[Scalar], b: &[Scalar]) -> Result<Scalar, ProofError> {
    if a.len() != b.len() {
        return Err(ProofError::InvalidProof(
            "inner-product vectors must have same length",
        ));
    }
    Ok(a.iter()
        .zip(b.iter())
        .fold(Scalar::from(0u64), |acc, (x, y)| acc + (*x * *y)))
}

fn biguint_to_scalar(value: &BigUint) -> Scalar {
    Scalar::from_be_bytes_mod_order(&value.to_bytes_be())
}

fn test_bit(value: &BigUint, bit_index: usize) -> bool {
    ((value >> bit_index) & BigUint::from(1u8)) == BigUint::from(1u8)
}

fn pow_usize(base: Scalar, exp: usize) -> Scalar {
    base.pow([exp as u64])
}

fn random_scalar(rng: &mut impl RngCore) -> Scalar {
    let mut bytes = [0u8; 64];
    rng.fill_bytes(&mut bytes);
    Scalar::from_be_bytes_mod_order(&bytes)
}
