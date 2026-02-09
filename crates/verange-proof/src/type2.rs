use crate::utils::{
    biguint_to_scalar, build_base_powers, commit_with_basis_and_h, inner_product, invert_or_err,
    pow_usize, random_scalar,
};
use crate::ProofError;
use ark_ff::Zero;
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::arith::decompose_to_nary_padded;
use verange_core::commit_to;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};
use verange_core::transcript::{Transcript, TranscriptMode};
use verange_core::{sum_commitments, PedersenParams};

#[derive(Clone, Debug)]
pub struct Type2Statement {
    pub nbits: usize,
    pub k: usize,
    pub l: usize,
    pub b: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type2Witness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type2Proof {
    pub ys: Vec<Commitment>,
    pub big_r: Commitment,
    pub big_s: Commitment,
    pub cws: Vec<Commitment>,
    pub cms: Vec<Commitment>,
    pub cvk: Vec<Commitment>,
    pub ctk: Vec<Commitment>,
    pub eta1: Scalar,
    pub eta2: Scalar,
    pub eta3: Scalar,
    pub vs: Vec<Vec<Scalar>>,
    pub us: Vec<Vec<Scalar>>,
}

pub struct Type2Prover;
pub struct Type2Verifier;

impl Type2Prover {
    pub fn prove(
        statement: &Type2Statement,
        witness: &Type2Witness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type2Proof, ProofError> {
        validate_statement(statement, params)?;
        validate_witness(statement, witness)?;

        let l = params.gs.len();
        let k = statement.k;
        let b = statement.b;
        let base_scalar = Scalar::from(statement.b as u64);
        let base_biguint = BigUint::from(statement.b as u64);

        let witness_count = if statement.aggregated {
            statement.tt
        } else {
            1
        };
        let mut ys = Vec::with_capacity(witness_count);
        let mut rcm = Vec::with_capacity(witness_count);
        for i in 0..witness_count {
            let r = random_scalar(rng);
            rcm.push(r);
            ys.push(commit_to(params, biguint_to_scalar(&witness.values[i]), r));
        }
        let gamma = challenge_gamma(&ys, mode);

        let mut rmu = vec![vec![Scalar::from(0u64); k]; l];
        let mut rv = vec![vec![Scalar::from(0u64); k]; l];
        for j in 0..l {
            for i in 0..k {
                rmu[j][i] = random_scalar(rng);
                rv[j][i] = random_scalar(rng);
            }
        }

        let mut romega = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigt = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigm = (0..b).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigv = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigr = random_scalar(rng);
        let rbigs = random_scalar(rng);

        let romega_sum = romega[..k - 1]
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let gamma_target = if statement.aggregated {
            (0..statement.tt).fold(Scalar::from(0u64), |acc, i| {
                acc + pow_usize(gamma, i + 1) * rcm[i]
            })
        } else {
            rcm[0]
        };
        romega[k - 1] = gamma_target - romega_sum;

        let bs = build_base_powers(statement.nbits, base_scalar);
        let mut ds = vec![vec![Scalar::from(0u64); k]; l];
        let mut bdigits = vec![vec![Scalar::from(0u64); k]; l];
        let mut bks = vec![vec![Scalar::from(0u64); k]; l];
        let mut bks_inv = vec![vec![Scalar::from(0u64); k]; l];
        let mut dds = vec![vec![Scalar::from(0u64); l]; k];
        let mut mc1 = vec![Scalar::from(0u64); b];

        if !statement.aggregated {
            let digits =
                decompose_to_nary_padded(&witness.values[0], &base_biguint, statement.nbits)?;
            for i in 0..(l * k) {
                let row = i / k;
                let col = i % k;
                if i < statement.nbits {
                    let digit = biguint_to_scalar(&digits[i]);
                    let bpow = bs[i];
                    let w = digit * bpow;
                    ds[row][col] = w;
                    bdigits[row][col] = digit;
                    bks[row][col] = bpow;
                    bks_inv[row][col] = invert_or_err(bpow, "base power inverse does not exist")?;
                    dds[col][row] = w;
                }
            }
            for (digit, slot) in mc1.iter_mut().enumerate() {
                let count = digits
                    .iter()
                    .filter(|value| **value == BigUint::from(digit as u64))
                    .count();
                *slot = Scalar::from(count as u64);
            }
        } else {
            let actual_mod = k.div_ceil(statement.tt) * l;
            let mut tt_counter = 0usize;
            let mut narys_t = Vec::with_capacity(statement.tt);
            let mut current_digits = Vec::new();
            for i in 0..(l * k) {
                let row = i / k;
                let col = i % k;
                let mod_agg = i % actual_mod;
                if mod_agg == 0 {
                    current_digits = decompose_to_nary_padded(
                        &witness.values[tt_counter],
                        &base_biguint,
                        statement.nbits / statement.tt,
                    )?;
                    narys_t.push(current_digits.clone());
                    tt_counter += 1;
                }

                let in_range = i < statement.nbits / statement.tt + (tt_counter - 1) * actual_mod
                    && i >= (tt_counter - 1) * actual_mod;
                if in_range {
                    let digit = biguint_to_scalar(&current_digits[mod_agg]);
                    let bpow = pow_usize(gamma, tt_counter) * bs[mod_agg];
                    let w = digit * bpow;
                    ds[row][col] = w;
                    bdigits[row][col] = digit;
                    bks[row][col] = bpow;
                    bks_inv[row][col] =
                        invert_or_err(bpow, "aggregated base power inverse does not exist")?;
                    dds[col][row] = w;
                }
            }
            for (digit, slot) in mc1.iter_mut().enumerate() {
                let target = BigUint::from(digit as u64);
                let count = narys_t
                    .iter()
                    .map(|digits| digits.iter().filter(|value| **value == target).count())
                    .sum::<usize>();
                *slot = Scalar::from(count as u64);
            }
        }

        let mut cws = Vec::with_capacity(k);
        for i in 0..k {
            let total = dds[i]
                .iter()
                .fold(Scalar::from(0u64), |acc, value| acc + *value);
            cws.push(commit_to(params, total, romega[i]));
        }

        let mut cms = Vec::with_capacity(b);
        for i in 0..b {
            cms.push(commit_to(params, mc1[i], rbigm[i]));
        }

        let mut cvk = Vec::with_capacity(k);
        for i in 0..k {
            let mut acc = Scalar::from(0u64);
            for j in 0..l {
                if !bks[j][i].is_zero() {
                    acc += bks[j][i] * rmu[j][i];
                }
            }
            cvk.push(commit_to(params, acc, rbigv[i]));
        }

        let alpha = challenge_alpha(&cws, &cms, &cvk, mode);

        let mut c_r_constant = Scalar::from(0u64);
        let mut c_s_constants = vec![Scalar::from(0u64); l];
        for j in 0..l {
            let mut row_sum = Scalar::from(0u64);
            for i in 0..k {
                if !bks[j][i].is_zero() {
                    c_r_constant += rv[j][i];
                    row_sum += rmu[j][i] * rv[j][i];
                }
            }
            c_s_constants[j] = row_sum;
        }
        let big_r = commit_to(params, c_r_constant, rbigr);
        let big_s = commit_with_basis_and_h(&params.gs, &c_s_constants, &params.h, rbigs)?;

        let mut fs = vec![vec![Scalar::from(0u64); k]; l];
        for j in 0..l {
            for i in 0..k {
                if !bks[j][i].is_zero() {
                    fs[j][i] = invert_or_err(
                        alpha + bdigits[j][i],
                        "alpha + digit inverse does not exist",
                    )?;
                }
            }
        }

        let mut ctk = Vec::with_capacity(k);
        for i in 0..k {
            let mut taos = vec![Scalar::from(0u64); l];
            for j in 0..l {
                if !bks[j][i].is_zero() {
                    let b_kf_rv = bks_inv[j][i] * fs[j][i] * rv[j][i];
                    let alpha_bk_wk = alpha * bks[j][i] + ds[j][i];
                    taos[j] = b_kf_rv + alpha_bk_wk * rmu[j][i];
                }
            }
            ctk.push(commit_with_basis_and_h(
                &params.gs, &taos, &params.h, rbigt[i],
            )?);
        }

        let beta = challenge_beta(&cws, &cms, &cvk, &ctk, mode);
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();
        let einverse = cl_es
            .iter()
            .map(|value| invert_or_err(*value, "epsilon inverse does not exist"))
            .collect::<Result<Vec<_>, _>>()?;

        let mut vs = vec![vec![Scalar::from(0u64); k]; l];
        let mut us = vec![vec![Scalar::from(0u64); k]; l];
        for j in 0..l {
            for i in 0..k {
                if !bks[j][i].is_zero() {
                    vs[j][i] = ds[j][i] * cl_es[i] + rv[j][i];
                    us[j][i] = bks_inv[j][i] * fs[j][i] * cl_es[i] + rmu[j][i];
                }
            }
        }

        let eta1 = inner_product(&rbigt, &cl_es)? + rbigs;
        let alpha_c_inv = (0..b)
            .map(|i| {
                invert_or_err(
                    alpha + Scalar::from(i as u64),
                    "alpha + i inverse does not exist",
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let eta2 = inner_product(&rbigm, &alpha_c_inv)? + inner_product(&rbigv, &einverse)?;
        let eta3 = inner_product(&romega, &cl_es)? + rbigr;

        Ok(Type2Proof {
            ys,
            big_r,
            big_s,
            cws,
            cms,
            cvk,
            ctk,
            eta1,
            eta2,
            eta3,
            vs,
            us,
        })
    }
}

impl Type2Verifier {
    pub fn verify(
        statement: &Type2Statement,
        proof: &Type2Proof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        validate_proof_shape(statement, proof, params)?;

        let l = params.gs.len();
        let k = statement.k;
        let base_scalar = Scalar::from(statement.b as u64);

        let gamma = challenge_gamma(&proof.ys, mode);
        let bs = build_base_powers(statement.nbits, base_scalar);

        let mut bks = vec![vec![Scalar::from(0u64); k]; l];
        let lk = l * k;
        if statement.tt == 1 {
            for i in 0..lk {
                let row = i / k;
                let col = i % k;
                if i < statement.nbits {
                    bks[row][col] = bs[i];
                }
            }
        } else {
            let actual_mod = k.div_ceil(statement.tt) * l;
            let mut tt_counter = 0usize;
            for i in 0..lk {
                let row = i / k;
                let col = i % k;
                let mod_agg = i % actual_mod;
                if mod_agg == 0 {
                    tt_counter += 1;
                }
                let in_range = i < statement.nbits / statement.tt + (tt_counter - 1) * actual_mod
                    && i >= (tt_counter - 1) * actual_mod;
                if in_range {
                    bks[row][col] = pow_usize(gamma, tt_counter) * bs[mod_agg];
                }
            }
        }

        let alpha = challenge_alpha(&proof.cws, &proof.cms, &proof.cvk, mode);
        let beta = challenge_beta(&proof.cws, &proof.cms, &proof.cvk, &proof.ctk, mode);
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();
        let einverse = cl_es
            .iter()
            .map(|value| invert_or_err(*value, "epsilon inverse does not exist"))
            .collect::<Result<Vec<_>, _>>()?;
        let e2s = cl_es
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + (*value * *value));

        let mut udotvs = vec![Scalar::from(0u64); l];
        let mut muprime_sum = Scalar::from(0u64);
        for j in 0..l {
            let mut udotv = Scalar::from(0u64);
            for i in 0..k {
                if !bks[j][i].is_zero() {
                    let vprime = bks[j][i] * alpha * cl_es[i] + proof.vs[j][i];
                    udotv += proof.us[j][i] * vprime;
                    muprime_sum += bks[j][i] * proof.us[j][i] * einverse[i];
                } else {
                    udotv += cl_es[i] * cl_es[i];
                }
            }
            udotvs[j] = udotv;
        }

        let eq1_lhs = commit_with_basis_and_h(&params.gs, &udotvs, &params.h, proof.eta1)?;
        let mut eq1_rhs = Commitment::identity();
        for (ctk, e) in proof.ctk.iter().zip(cl_es.iter()) {
            eq1_rhs = eq1_rhs.add(&ctk.mul_scalar(*e));
        }
        eq1_rhs = eq1_rhs.add(&proof.big_s);
        let h_sum = params
            .gs
            .iter()
            .fold(CurvePoint::default(), |acc, value| acc + *value);
        eq1_rhs = eq1_rhs.add(&Commitment::new(h_sum * e2s));
        let b1 = eq1_lhs == eq1_rhs;

        let eq2_lhs = commit_to(params, muprime_sum, proof.eta2);
        let mut eq2_rhs = Commitment::identity();
        for (i, cm) in proof.cms.iter().enumerate() {
            let inv = invert_or_err(
                alpha + Scalar::from(i as u64),
                "alpha + i inverse does not exist",
            )?;
            eq2_rhs = eq2_rhs.add(&cm.mul_scalar(inv));
        }
        for (cvk, inv) in proof.cvk.iter().zip(einverse.iter()) {
            eq2_rhs = eq2_rhs.add(&cvk.mul_scalar(*inv));
        }
        let b2 = eq2_lhs == eq2_rhs;

        let vsum = proof
            .vs
            .iter()
            .flat_map(|row| row.iter())
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let eq3_lhs = commit_to(params, vsum, proof.eta3);
        let mut eq3_rhs = Commitment::identity();
        for (cw, e) in proof.cws.iter().zip(cl_es.iter()) {
            eq3_rhs = eq3_rhs.add(&cw.mul_scalar(*e));
        }
        eq3_rhs = eq3_rhs.add(&proof.big_r);
        let b3 = eq3_lhs == eq3_rhs;

        let sum_cws = sum_commitments(&proof.cws);
        let b4 = if statement.tt == 1 {
            proof.ys[0] == sum_cws
        } else {
            let mut agg = Commitment::identity();
            for (i, y) in proof.ys.iter().enumerate().take(statement.tt) {
                agg = agg.add(&y.mul_scalar(pow_usize(gamma, i + 1)));
            }
            agg == sum_cws
        };

        Ok(b1 && b2 && b3 && b4)
    }
}

fn validate_statement(
    statement: &Type2Statement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if statement.nbits == 0 {
        return Err(ProofError::InvalidStatement("nbits must be > 0"));
    }
    if statement.k == 0 {
        return Err(ProofError::InvalidStatement("k must be > 0"));
    }
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.l != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement L must match params.gs length",
        ));
    }
    if statement.nbits > statement.l * statement.k {
        return Err(ProofError::InvalidStatement("L*K must be >= nbits"));
    }
    if !statement.aggregated && statement.tt != 1 {
        return Err(ProofError::InvalidStatement(
            "non-aggregated mode requires tt == 1",
        ));
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
    if statement.tt == 0 {
        return Err(ProofError::InvalidStatement("tt must be > 0"));
    }
    Ok(())
}

fn validate_witness(statement: &Type2Statement, witness: &Type2Witness) -> Result<(), ProofError> {
    if witness.values.is_empty() {
        return Err(ProofError::InvalidWitness(
            "at least one witness value is required",
        ));
    }
    let expected = if statement.aggregated {
        statement.tt
    } else {
        1
    };
    if witness.values.len() != expected {
        return Err(ProofError::InvalidWitness(
            "witness values length does not match statement",
        ));
    }
    Ok(())
}

fn validate_proof_shape(
    statement: &Type2Statement,
    proof: &Type2Proof,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    let l = params.gs.len();
    let k = statement.k;
    let expected_ys = if statement.aggregated {
        statement.tt
    } else {
        1
    };

    if proof.ys.len() != expected_ys {
        return Err(ProofError::InvalidProof("invalid Y commitments length"));
    }
    if proof.cws.len() != k || proof.cvk.len() != k || proof.ctk.len() != k {
        return Err(ProofError::InvalidProof("invalid cws/cvk/ctk length"));
    }
    if proof.cms.len() != statement.b {
        return Err(ProofError::InvalidProof("invalid cms length"));
    }
    if proof.vs.len() != l || proof.vs.iter().any(|row| row.len() != k) {
        return Err(ProofError::InvalidProof("invalid vs matrix shape"));
    }
    if proof.us.len() != l || proof.us.iter().any(|row| row.len() != k) {
        return Err(ProofError::InvalidProof("invalid us matrix shape"));
    }
    Ok(())
}

fn challenge_gamma(ys: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type2-gamma", mode);
    for commitment in ys {
        transcript.append_point(b"Y", commitment.point());
    }
    transcript.challenge_scalar(b"gamma")
}

fn challenge_alpha(
    cws: &[Commitment],
    cms: &[Commitment],
    cvk: &[Commitment],
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type2-alpha", mode);
    for commitment in cws {
        transcript.append_point(b"cw", commitment.point());
    }
    for commitment in cms {
        transcript.append_point(b"cm", commitment.point());
    }
    for commitment in cvk {
        transcript.append_point(b"cv", commitment.point());
    }
    transcript.challenge_scalar(b"alpha")
}

fn challenge_beta(
    cws: &[Commitment],
    cms: &[Commitment],
    cvk: &[Commitment],
    ctk: &[Commitment],
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type2-beta", mode);
    for commitment in cws {
        transcript.append_point(b"cw", commitment.point());
    }
    for commitment in cms {
        transcript.append_point(b"cm", commitment.point());
    }
    for commitment in cvk {
        transcript.append_point(b"cv", commitment.point());
    }
    for commitment in ctk {
        transcript.append_point(b"ct", commitment.point());
    }
    transcript.challenge_scalar(b"beta")
}
