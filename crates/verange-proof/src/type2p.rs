use crate::utils::{
    biguint_to_scalar, build_base_powers, commit_with_basis_and_h, inner_product, invert_or_err,
    pow_usize, random_scalar,
};
use crate::ProofError;
use ark_ff::{One, Zero};
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::arith::decompose_to_nary_padded;
use verange_core::commit_to;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};
use verange_core::transcript::{Transcript, TranscriptMode};
use verange_core::{sum_commitments, PedersenParams};

#[derive(Clone, Debug)]
pub struct Type2PStatement {
    pub nbits: usize,
    pub k: usize,
    pub l: usize,
    pub b: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type2PWitness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type2PProof {
    pub ys: Vec<Commitment>,
    pub big_r: Commitment,
    pub big_s: Commitment,
    pub big_u: Commitment,
    pub cws: Vec<Commitment>,
    pub cms: Vec<Commitment>,
    pub cfk: Vec<Commitment>,
    pub ctk: Vec<Commitment>,
    pub ctk_kprime: Vec<Commitment>,
    pub eta1: Scalar,
    pub eta2: Scalar,
    pub eta3: Scalar,
    pub eta4: Scalar,
    pub vs: Vec<Scalar>,
    pub us: Vec<Scalar>,
}

pub struct Type2PProver;
pub struct Type2PVerifier;

impl Type2PProver {
    pub fn prove(
        statement: &Type2PStatement,
        witness: &Type2PWitness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type2PProof, ProofError> {
        validate_statement(statement, params)?;
        validate_witness(statement, witness)?;

        let k = statement.k;
        let b = statement.b;
        let eta = statement.nbits - statement.l * (statement.k - 1);
        let rows = matrix_rows(statement);
        let base_scalar = Scalar::from(statement.b as u64);
        let base_biguint = BigUint::from(statement.b as u64);
        let base_powers = build_base_powers(statement.nbits, base_scalar);

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

        // Java reference hard-codes gamma = 1 in Type2P.
        let gamma = Scalar::one();

        let mut rmu = vec![Scalar::from(0u64); rows];
        let mut rv = vec![Scalar::from(0u64); rows];
        for i in 0..rows {
            rmu[i] = random_scalar(rng);
            rv[i] = random_scalar(rng);
        }

        let mut romega = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigt = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigm = (0..b).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigf = (0..k).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let rbigr = random_scalar(rng);
        let rbigs = random_scalar(rng);
        let rbigu = random_scalar(rng);

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

        let matrices =
            build_type2p_matrices(statement, witness, &base_biguint, &base_powers, gamma, true)?;

        let mut cws = Vec::with_capacity(k);
        for col in 0..k {
            let total = (0..rows).fold(Scalar::from(0u64), |acc, row| acc + matrices.ws[row][col]);
            cws.push(commit_to(params, total, romega[col]));
        }

        let mut cms = Vec::with_capacity(b);
        for i in 0..b {
            cms.push(commit_to(params, matrices.mc1[i], rbigm[i]));
        }

        let c_r_constant = rv
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let c_s_constants = (0..rows).map(|row| rmu[row] * rv[row]).collect::<Vec<_>>();
        let mut c_u_constant = Scalar::from(0u64);
        for row in 0..rows {
            let row_local = row % statement.l;
            c_u_constant += pow_usize(base_scalar, row_local) * rmu[row];
        }

        let big_r = commit_to(params, c_r_constant, rbigr);
        let big_s = commit_with_basis_and_h(&params.gs, &c_s_constants, &params.h, rbigs)?;
        let big_u = commit_to(params, c_u_constant, rbigu);

        let alpha = challenge_alpha(&cws, &cms, mode);

        let mut fs = vec![vec![Scalar::from(0u64); k]; rows];
        for row in 0..rows {
            for col in 0..k {
                if !matrices.bks[row][col].is_zero() {
                    fs[row][col] = invert_or_err(
                        alpha + matrices.digits[row][col],
                        "alpha + digit inverse does not exist",
                    )?;
                }
            }
        }

        let mut ctk = Vec::with_capacity(k);
        for col in 0..k {
            let mut taos = vec![Scalar::from(0u64); rows];
            for row in 0..rows {
                if !matrices.bks[row][col].is_zero() {
                    let b_kf_rv = matrices.bks_inv[row][col] * fs[row][col] * rv[row];
                    let alpha_bk_wk = alpha * matrices.bks[row][col] + matrices.ws[row][col];
                    taos[row] = b_kf_rv + alpha_bk_wk * rmu[row];
                }
            }
            ctk.push(commit_with_basis_and_h(
                &params.gs, &taos, &params.h, rbigt[col],
            )?);
        }

        let pair_indices =
            pair_indices(k).ok_or(ProofError::InvalidStatement("Type2P supports K in [2, 4]"))?;
        let mut rt_pairs = Vec::with_capacity(pair_indices.len());
        for _ in 0..pair_indices.len() {
            rt_pairs.push(random_scalar(rng));
        }

        let mut ctk_kprime = Vec::with_capacity(pair_indices.len());
        for (pair_idx, (left, right)) in pair_indices.iter().copied().enumerate() {
            let mut taos_hat = vec![Scalar::from(0u64); rows];
            for row in 0..rows {
                let left_part = matrices.bks_inv[row][left]
                    * fs[row][left]
                    * (alpha * matrices.bks[row][right] + matrices.ws[row][right]);
                let right_part = matrices.bks_inv[row][right]
                    * fs[row][right]
                    * (alpha * matrices.bks[row][left] + matrices.ws[row][left]);
                taos_hat[row] = left_part + right_part;
            }
            ctk_kprime.push(commit_with_basis_and_h(
                &params.gs,
                &taos_hat,
                &params.h,
                rt_pairs[pair_idx],
            )?);
        }

        let mut cfk = Vec::with_capacity(k);
        for col in 0..k {
            let total = (0..rows).fold(Scalar::from(0u64), |acc, row| acc + fs[row][col]);
            cfk.push(commit_to(params, total, rbigf[col]));
        }

        let beta = challenge_beta(&cws, &cms, &cfk, &ctk, &ctk_kprime, mode);
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();
        let mut vs = vec![Scalar::from(0u64); rows];
        let mut us = vec![Scalar::from(0u64); rows];
        for row in 0..rows {
            let mut v_acc = Scalar::from(0u64);
            let mut u_acc = Scalar::from(0u64);
            let mut has_nonzero = false;
            for col in 0..k {
                if !matrices.bks[row][col].is_zero() {
                    has_nonzero = true;
                    v_acc += (alpha * matrices.bks[row][col] + matrices.ws[row][col]) * cl_es[col];
                    u_acc += matrices.bks_inv[row][col] * fs[row][col] * cl_es[col];
                }
            }
            if has_nonzero {
                vs[row] = v_acc + rv[row];
                us[row] = u_acc + rmu[row];
            }
        }

        let eta1_pair_term = pair_indices
            .iter()
            .enumerate()
            .fold(Scalar::from(0u64), |acc, (idx, (left, right))| {
                acc + rt_pairs[idx] * cl_es[*left] * cl_es[*right]
            });
        let eta1 = eta1_pair_term + inner_product(&rbigt, &cl_es)? + rbigs;

        let mut eta2 = rbigf[0] * cl_es[0];
        for col in 1..k {
            let exp = statement.l * (col - 1) + eta;
            let inv = invert_or_err(base_powers[exp], "base power inverse does not exist")?;
            eta2 += rbigf[col] * cl_es[col] * inv;
        }
        eta2 += rbigu;

        let alpha_c_inv = (0..b)
            .map(|i| {
                invert_or_err(
                    alpha + Scalar::from(i as u64),
                    "alpha + i inverse does not exist",
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let eta3 = inner_product(&rbigm, &alpha_c_inv)?
            - rbigf
                .iter()
                .fold(Scalar::from(0u64), |acc, value| acc + *value);

        let eta4 = inner_product(&romega, &cl_es)? + rbigr;

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
}

impl Type2PVerifier {
    pub fn verify(
        statement: &Type2PStatement,
        proof: &Type2PProof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        validate_proof_shape(statement, proof, params)?;

        let k = statement.k;
        let eta = statement.nbits - statement.l * (statement.k - 1);
        let rows = matrix_rows(statement);
        let base_scalar = Scalar::from(statement.b as u64);
        let base_powers = build_base_powers(statement.nbits, base_scalar);

        let gamma = Scalar::one();
        let bks = build_type2p_bks(statement, &base_powers, gamma)?;

        let alpha = challenge_alpha(&proof.cws, &proof.cms, mode);
        let beta = challenge_beta(
            &proof.cws,
            &proof.cms,
            &proof.cfk,
            &proof.ctk,
            &proof.ctk_kprime,
            mode,
        );
        let cl_es = (0..k).map(|i| pow_usize(beta, i + 1)).collect::<Vec<_>>();
        let e2s = cl_es
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + (*value * *value));

        let mut esk2 = Scalar::from(0u64);
        for row in 0..rows {
            for col in 0..k {
                if bks[row][col].is_zero() {
                    esk2 += cl_es[col] * cl_es[col];
                }
            }
        }

        let mut vj_muj_es2 = vec![Scalar::from(0u64); rows];
        for row in 0..rows {
            vj_muj_es2[row] = proof.vs[row] * proof.us[row] + esk2;
        }
        let eq1_lhs = commit_with_basis_and_h(&params.gs, &vj_muj_es2, &params.h, proof.eta1)?;
        let pair_indices =
            pair_indices(k).ok_or(ProofError::InvalidStatement("Type2P supports K in [2, 4]"))?;
        let mut eq1_rhs = Commitment::identity();
        for (idx, (left, right)) in pair_indices.iter().enumerate() {
            eq1_rhs = eq1_rhs.add(&proof.ctk_kprime[idx].mul_scalar(cl_es[*left] * cl_es[*right]));
        }
        for col in 0..k {
            eq1_rhs = eq1_rhs.add(&proof.ctk[col].mul_scalar(cl_es[col]));
        }
        eq1_rhs = eq1_rhs.add(&proof.big_s);
        let h_sum = params
            .gs
            .iter()
            .fold(CurvePoint::default(), |acc, point| acc + *point);
        eq1_rhs = eq1_rhs.add(&Commitment::new(h_sum * e2s));
        let b1 = eq1_lhs == eq1_rhs;

        let mut bj_1 = Scalar::from(0u64);
        for row in 0..rows {
            let row_local = row % statement.l;
            bj_1 += pow_usize(base_scalar, row_local) * proof.us[row];
        }
        let eq2_lhs = commit_to(params, bj_1, proof.eta2);
        let mut eq2_rhs = proof.cfk[0].mul_scalar(cl_es[0]);
        for col in 1..k {
            let exp = statement.l * (col - 1) + eta;
            let inv = invert_or_err(base_powers[exp], "base power inverse does not exist")?;
            eq2_rhs = eq2_rhs.add(&proof.cfk[col].mul_scalar(cl_es[col] * inv));
        }
        eq2_rhs = eq2_rhs.add(&proof.big_u);
        let b2 = eq2_lhs == eq2_rhs;

        let fk_sum = sum_commitments(&proof.cfk);
        let eq3_lhs = fk_sum.add(&Commitment::new(params.h * proof.eta3));
        let mut eq3_rhs = Commitment::identity();
        for (i, cm) in proof.cms.iter().enumerate() {
            let inv = invert_or_err(
                alpha + Scalar::from(i as u64),
                "alpha + i inverse does not exist",
            )?;
            eq3_rhs = eq3_rhs.add(&cm.mul_scalar(inv));
        }
        let b3 = eq3_lhs == eq3_rhs;

        let mut bk_sum = vec![Scalar::from(0u64); k];
        for col in 0..k {
            for row in 0..rows {
                bk_sum[col] += bks[row][col];
            }
        }
        let alpha_bk_ek = Scalar::from(alpha) * inner_product(&cl_es, &bk_sum)?;
        let vj_sum = proof
            .vs
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let eq4_lhs = commit_to(params, vj_sum - alpha_bk_ek, proof.eta4);
        let mut eq4_rhs = Commitment::identity();
        for (cw, e) in proof.cws.iter().zip(cl_es.iter()) {
            eq4_rhs = eq4_rhs.add(&cw.mul_scalar(*e));
        }
        eq4_rhs = eq4_rhs.add(&proof.big_r);
        let b4 = eq4_lhs == eq4_rhs;

        let sum_cws = sum_commitments(&proof.cws);
        let b5 = if statement.tt == 1 {
            proof.ys[0] == sum_cws
        } else {
            let mut agg = Commitment::identity();
            for (i, y) in proof.ys.iter().enumerate().take(statement.tt) {
                agg = agg.add(&y.mul_scalar(pow_usize(gamma, i + 1)));
            }
            agg == sum_cws
        };

        Ok(b1 && b2 && b3 && b4 && b5)
    }
}

#[derive(Clone, Debug)]
struct Type2PMatrices {
    bks: Vec<Vec<Scalar>>,
    bks_inv: Vec<Vec<Scalar>>,
    digits: Vec<Vec<Scalar>>,
    ws: Vec<Vec<Scalar>>,
    mc1: Vec<Scalar>,
}

fn build_type2p_matrices(
    statement: &Type2PStatement,
    witness: &Type2PWitness,
    base_biguint: &BigUint,
    base_powers: &[Scalar],
    gamma: Scalar,
    with_counts: bool,
) -> Result<Type2PMatrices, ProofError> {
    let rows = matrix_rows(statement);
    let k = statement.k;
    let b = statement.b;
    let eta = statement.nbits - statement.l * (statement.k - 1);

    let mut bks = vec![vec![Scalar::from(0u64); k]; rows];
    let mut bks_inv = vec![vec![Scalar::from(0u64); k]; rows];
    let mut digits_matrix = vec![vec![Scalar::from(0u64); k]; rows];
    let mut ws = vec![vec![Scalar::from(0u64); k]; rows];
    let mut mc1 = vec![Scalar::from(0u64); b];

    let blocks = if statement.aggregated {
        statement.tt
    } else {
        1
    };
    let mut all_digits = Vec::with_capacity(blocks);
    for block in 0..blocks {
        let digits =
            decompose_to_nary_padded(&witness.values[block], base_biguint, statement.nbits)?;
        all_digits.push(digits);
    }

    for block in 0..blocks {
        let gamma_t = pow_usize(gamma, block + 1);
        for row_local in 0..statement.l {
            let row = block * statement.l + row_local;
            for col in 0..k {
                let idx_opt = if col == 0 {
                    if row_local >= eta {
                        None
                    } else {
                        Some(row_local)
                    }
                } else {
                    Some(row_local + eta + (col - 1) * statement.l)
                };

                let Some(idx) = idx_opt else {
                    continue;
                };
                let bpow = gamma_t * base_powers[idx];
                let digit = biguint_to_scalar(&all_digits[block][idx]);
                let w = bpow * digit;
                bks[row][col] = bpow;
                bks_inv[row][col] = invert_or_err(bpow, "base power inverse does not exist")?;
                digits_matrix[row][col] = digit;
                ws[row][col] = w;
            }
        }
    }

    if with_counts {
        for (digit, slot) in mc1.iter_mut().enumerate() {
            let target = BigUint::from(digit as u64);
            let count = all_digits
                .iter()
                .map(|digits| digits.iter().filter(|value| **value == target).count())
                .sum::<usize>();
            *slot = Scalar::from(count as u64);
        }
    }

    Ok(Type2PMatrices {
        bks,
        bks_inv,
        digits: digits_matrix,
        ws,
        mc1,
    })
}

fn build_type2p_bks(
    statement: &Type2PStatement,
    base_powers: &[Scalar],
    gamma: Scalar,
) -> Result<Vec<Vec<Scalar>>, ProofError> {
    let rows = matrix_rows(statement);
    let k = statement.k;
    let eta = statement.nbits - statement.l * (statement.k - 1);
    let mut bks = vec![vec![Scalar::from(0u64); k]; rows];
    let blocks = if statement.aggregated {
        statement.tt
    } else {
        1
    };

    for block in 0..blocks {
        let gamma_t = pow_usize(gamma, block + 1);
        for row_local in 0..statement.l {
            let row = block * statement.l + row_local;
            for col in 0..k {
                let idx_opt = if col == 0 {
                    if row_local >= eta {
                        None
                    } else {
                        Some(row_local)
                    }
                } else {
                    Some(row_local + eta + (col - 1) * statement.l)
                };
                if let Some(idx) = idx_opt {
                    bks[row][col] = gamma_t * base_powers[idx];
                }
            }
        }
    }

    Ok(bks)
}

fn validate_statement(
    statement: &Type2PStatement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if !(2..=4).contains(&statement.k) {
        return Err(ProofError::InvalidStatement(
            "Type2P currently supports K in [2, 4]",
        ));
    }
    if statement.nbits == 0 {
        return Err(ProofError::InvalidStatement("nbits must be > 0"));
    }
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.tt == 0 {
        return Err(ProofError::InvalidStatement("tt must be > 0"));
    }
    if statement.l == 0 {
        return Err(ProofError::InvalidStatement("L must be > 0"));
    }
    if statement.nbits < statement.l * (statement.k - 1) {
        return Err(ProofError::InvalidStatement(
            "nbits must be >= L*(K-1) for Type2P decomposition",
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

    let required_rows = matrix_rows(statement);
    if params.gs.len() != required_rows {
        return Err(ProofError::InvalidStatement(
            "params.gs length must equal Type2P row count",
        ));
    }

    Ok(())
}

fn validate_witness(
    statement: &Type2PStatement,
    witness: &Type2PWitness,
) -> Result<(), ProofError> {
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
    statement: &Type2PStatement,
    proof: &Type2PProof,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    let expected_ys = if statement.aggregated {
        statement.tt
    } else {
        1
    };
    if proof.ys.len() != expected_ys {
        return Err(ProofError::InvalidProof("invalid Y commitments length"));
    }
    if proof.cws.len() != statement.k
        || proof.cfk.len() != statement.k
        || proof.ctk.len() != statement.k
    {
        return Err(ProofError::InvalidProof("invalid cws/cfk/ctk length"));
    }
    if proof.cms.len() != statement.b {
        return Err(ProofError::InvalidProof("invalid cms length"));
    }
    let expected_pairs = pair_indices(statement.k)
        .ok_or(ProofError::InvalidStatement(
            "Type2P currently supports K in [2, 4]",
        ))?
        .len();
    if proof.ctk_kprime.len() != expected_pairs {
        return Err(ProofError::InvalidProof("invalid ctk_kprime length"));
    }
    if proof.vs.len() != params.gs.len() || proof.us.len() != params.gs.len() {
        return Err(ProofError::InvalidProof("invalid vs/us vector length"));
    }
    Ok(())
}

fn matrix_rows(statement: &Type2PStatement) -> usize {
    if statement.aggregated {
        statement.l * statement.tt
    } else {
        statement.l
    }
}

fn pair_indices(k: usize) -> Option<Vec<(usize, usize)>> {
    match k {
        2 => Some(vec![(0, 1)]),
        3 => Some(vec![(0, 1), (1, 2), (2, 0)]),
        4 => Some(vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]),
        _ => None,
    }
}

fn challenge_alpha(cws: &[Commitment], cms: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type2p-alpha", mode);
    for commitment in cws {
        transcript.append_point(b"cw", commitment.point());
    }
    for commitment in cms {
        transcript.append_point(b"cm", commitment.point());
    }
    transcript.challenge_scalar(b"alpha")
}

fn challenge_beta(
    cws: &[Commitment],
    cms: &[Commitment],
    cfk: &[Commitment],
    ctk: &[Commitment],
    ctk_kprime: &[Commitment],
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type2p-beta", mode);
    for commitment in cws {
        transcript.append_point(b"cw", commitment.point());
    }
    for commitment in cms {
        transcript.append_point(b"cm", commitment.point());
    }
    for commitment in cfk {
        transcript.append_point(b"cf", commitment.point());
    }
    for commitment in ctk {
        transcript.append_point(b"ct", commitment.point());
    }
    for commitment in ctk_kprime {
        transcript.append_point(b"ctk", commitment.point());
    }
    transcript.challenge_scalar(b"beta")
}
