use crate::ProofError;
use ark_ff::{Field, PrimeField};
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::arith::decompose_to_nary_padded;
use verange_core::commit_to;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};
use verange_core::transcript::{Transcript, TranscriptMode};
use verange_core::{sum_commitments, PedersenParams};
use verange_poly_commit::commit::{commit_poly, open_poly, verify_poly, PolyCommitParams};
use verange_poly_commit::polynomial::Polynomial;

#[derive(Clone, Debug)]
pub struct Type3Statement {
    pub nbits: usize,
    pub u: usize,
    pub v: usize,
    pub b: usize,
    pub tt: usize,
    pub aggregated: bool,
}

#[derive(Clone, Debug)]
pub struct Type3Witness {
    pub values: Vec<BigUint>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type3Proof {
    pub witness_commitments: Vec<Commitment>,
    pub c_d: Vec<Commitment>,
    pub c_w: Vec<Commitment>,
    pub c_r1: Commitment,
    pub c_r2: Commitment,
    pub cm_s: Vec<Commitment>,
    pub cm_b: Vec<Commitment>,
    pub pi_sx: Vec<Scalar>,
    pub pi_bx: Vec<Scalar>,
    pub sx_degree: usize,
    pub bx_degree: usize,
    pub y_s: Scalar,
    pub y_b: Scalar,
    pub eta1: Scalar,
    pub eta2: Scalar,
    pub djx: Vec<Scalar>,
    pub bjx: Vec<Scalar>,
    pub zs: Vec<Scalar>,
    pub x: Scalar,
}

pub struct Type3Prover;
pub struct Type3Verifier;

impl Type3Prover {
    pub fn prove(
        statement: &Type3Statement,
        witness: &Type3Witness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type3Proof, ProofError> {
        validate_statement(statement, params)?;
        validate_witness(statement, witness)?;

        let u = statement.u;
        let v = statement.v;
        let base_scalar = Scalar::from(statement.b as u64);
        let base_biguint = BigUint::from(statement.b as u64);
        let base_powers = build_base_powers(statement.nbits, base_scalar);

        let witness_count = if statement.aggregated {
            statement.tt
        } else {
            1
        };
        let mut witness_commitments = Vec::with_capacity(witness_count);
        let mut rcw = Vec::with_capacity(witness_count);
        for i in 0..witness_count {
            let r = random_scalar(rng);
            rcw.push(r);
            witness_commitments.push(commit_to(params, biguint_to_scalar(&witness.values[i]), r));
        }
        let gamma = challenge_gamma(&witness_commitments, mode);

        let mut zs = Vec::with_capacity(v + 1);
        let mut r_w = Vec::with_capacity(v);
        let mut r_d_cap = Vec::with_capacity(v);
        for _ in 0..v {
            zs.push(random_scalar(rng));
            r_w.push(random_scalar(rng));
            r_d_cap.push(random_scalar(rng));
        }
        zs.push(random_scalar(rng));
        let x = random_scalar(rng);

        let mut r_ds = Vec::with_capacity(u);
        for _ in 0..u {
            r_ds.push(random_scalar(rng));
        }

        let r_w_sum = r_w[..v - 1]
            .iter()
            .fold(Scalar::from(0u64), |acc, value| acc + *value);
        let gamma_target = if statement.aggregated {
            (0..statement.tt).fold(Scalar::from(0u64), |acc, i| {
                acc + pow_usize(gamma, i + 1) * rcw[i]
            })
        } else {
            rcw[0]
        };
        r_w[v - 1] = gamma_target - r_w_sum;

        let s_u = random_scalar(rng);
        let r1 = random_scalar(rng);
        let r2 = random_scalar(rng);

        let mut ds = vec![vec![Scalar::from(0u64); u]; v];
        let mut bs_digits = vec![vec![Scalar::from(0u64); v]; u];
        let mut bjk = vec![vec![Scalar::from(0u64); v]; u];
        let mut dds = vec![vec![Scalar::from(0u64); u]; v];
        fill_type3_matrices(
            statement,
            witness,
            &base_biguint,
            &base_powers,
            gamma,
            &mut ds,
            &mut bs_digits,
            &mut bjk,
            &mut dds,
        )?;

        let l0 = l0_polynomial(&zs);
        let mut d_x = Vec::with_capacity(u);
        let mut bj_hat_x = Vec::with_capacity(u);
        for row in 0..u {
            let mut duv_lx = Polynomial::zero();
            let mut bjk_lk = Polynomial::zero();
            for col in 0..v {
                let l_col = lagrange_polynomial(&zs, col + 1)?;
                duv_lx = duv_lx.add(&poly_mul_scalar(&l_col, bs_digits[row][col]));
                bjk_lk = bjk_lk.add(&poly_mul_scalar(&l_col, bjk[row][col]));
            }
            let d_poly = poly_mul_scalar(&l0, r_ds[row]).add(&duv_lx);
            d_x.push(d_poly);
            bj_hat_x.push(bjk_lk);
        }

        let mut bj_x = Vec::with_capacity(u);
        for row in 0..u {
            let mut term = d_x[row].clone();
            for digit in 1..statement.b {
                term = term.mul(&d_x[row].sub(&poly_constant(Scalar::from(digit as u64))));
            }
            bj_x.push(term.long_divide(&l0).map_err(map_poly_err)?.0);
        }

        let mut c_d = Vec::with_capacity(v);
        let mut c_w = Vec::with_capacity(v);
        let mut wk = Vec::with_capacity(v);
        for col in 0..v {
            c_d.push(commit_with_basis_and_h(
                &params.gs,
                &dds[col],
                &params.h,
                r_d_cap[col],
            )?);
            let w_col = ds[col]
                .iter()
                .fold(Scalar::from(0u64), |acc, value| acc + *value);
            wk.push(w_col);
            c_w.push(commit_to(params, w_col, r_w[col]));
        }

        let mut dvbv = Polynomial::zero();
        for row in 0..u {
            dvbv = dvbv.add(&d_x[row].mul(&bj_hat_x[row]));
        }
        let mut wulu = Polynomial::zero();
        for col in 0..v {
            let l_col = lagrange_polynomial(&zs, col + 1)?;
            wulu = wulu.add(&poly_mul_scalar(&l_col, wk[col]));
        }
        let s_x =
            poly_constant(s_u).add(&wulu.sub(&dvbv).long_divide(&l0).map_err(map_poly_err)?.0);

        let sx_degree = s_x.degree();
        let (m_s, n_s) = matrix_dims_square(sx_degree);
        let blinding_s = (0..n_s).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let params_s = PolyCommitParams::new(n_s + 1).map_err(map_poly_err)?;
        let cm_s =
            commit_poly(&s_x, m_s, n_s, sx_degree, &blinding_s, &params_s).map_err(map_poly_err)?;

        let c_r1 = commit_with_basis_and_h(&params.gs, &r_ds, &params.h, r1)?;
        let c_r2 = commit_to(params, s_u, r2);

        let mut djx = Vec::with_capacity(v);
        for col in 0..v {
            djx.push(d_x[col].evaluate(x));
        }
        let l0_value = l0.evaluate(x);
        let mut bjx = Vec::with_capacity(u);
        for row in 0..u {
            bjx.push(bj_hat_x[row].evaluate(x));
        }

        let beta = challenge_beta(&c_d, &c_w, &c_r1, &c_r2, mode);
        let mut b_x = Polynomial::zero();
        for row in 0..u {
            b_x = b_x.add(&poly_mul_scalar(&bj_x[row], pow_usize(beta, row + 1)));
        }
        let y_b = b_x.evaluate(x);

        let bx_degree = b_x.degree();
        let (m_b, n_b) = matrix_dims_square(bx_degree);
        let blinding_b = (0..n_b).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let params_b = PolyCommitParams::new(n_b + 1).map_err(map_poly_err)?;
        let cm_b =
            commit_poly(&b_x, m_b, n_b, bx_degree, &blinding_b, &params_b).map_err(map_poly_err)?;

        let y_s = s_x.evaluate(x);
        let pi_sx = open_poly(&s_x, x, m_s, n_s, sx_degree, &blinding_s).map_err(map_poly_err)?;
        let pi_bx = open_poly(&b_x, x, m_b, n_b, bx_degree, &blinding_b).map_err(map_poly_err)?;

        let mut lkx_eval = Vec::with_capacity(v);
        for col in 0..v {
            lkx_eval.push(lagrange_polynomial(&zs, col + 1)?.evaluate(x));
        }
        let r_dlu = (0..v).fold(Scalar::from(0u64), |acc, col| {
            acc + lkx_eval[col] * r_d_cap[col]
        });
        let r_wlu = (0..v).fold(Scalar::from(0u64), |acc, col| {
            acc + lkx_eval[col] * r_w[col]
        });
        let eta1 = r_dlu + l0_value * r1;
        let eta2 = r_wlu + l0_value * r2;

        Ok(Type3Proof {
            witness_commitments,
            c_d,
            c_w,
            c_r1,
            c_r2,
            cm_s,
            cm_b,
            pi_sx,
            pi_bx,
            sx_degree,
            bx_degree,
            y_s,
            y_b,
            eta1,
            eta2,
            djx,
            bjx,
            zs,
            x,
        })
    }
}

impl Type3Verifier {
    pub fn verify(
        statement: &Type3Statement,
        proof: &Type3Proof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        validate_proof_shape(statement, proof)?;

        let u = statement.u;
        let v = statement.v;
        let gamma = challenge_gamma(&proof.witness_commitments, mode);

        let l0 = l0_polynomial(&proof.zs);
        let l0_value = l0.evaluate(proof.x);
        let mut lkx_eval = Vec::with_capacity(v);
        for col in 0..v {
            lkx_eval.push(lagrange_polynomial(&proof.zs, col + 1)?.evaluate(proof.x));
        }

        let beta = challenge_beta(&proof.c_d, &proof.c_w, &proof.c_r1, &proof.c_r2, mode);

        let params_b = PolyCommitParams::new(proof.pi_bx.len()).map_err(map_poly_err)?;
        let params_s = PolyCommitParams::new(proof.pi_sx.len()).map_err(map_poly_err)?;
        let b_b = verify_poly(
            &proof.cm_b,
            proof.x,
            proof.y_b,
            &proof.pi_bx,
            proof.bx_degree,
            &params_b,
        )
        .map_err(map_poly_err)?;
        let b_s = verify_poly(
            &proof.cm_s,
            proof.x,
            proof.y_s,
            &proof.pi_sx,
            proof.sx_degree,
            &params_s,
        )
        .map_err(map_poly_err)?;
        if !b_b || !b_s {
            return Ok(false);
        }

        let eq1_lhs = commit_with_basis_and_h(&params.gs, &proof.djx, &params.h, proof.eta1)?;
        let mut eq1_rhs = Commitment::identity();
        for col in 0..v {
            eq1_rhs = eq1_rhs.add(&proof.c_d[col].mul_scalar(lkx_eval[col]));
        }
        eq1_rhs = eq1_rhs.add(&proof.c_r1.mul_scalar(l0_value));
        let b1 = eq1_lhs == eq1_rhs;

        let mut bx_eval = Scalar::from(0u64);
        for row in 0..u {
            let mut term = Scalar::from(1u64);
            for digit in 0..statement.b {
                term *= proof.djx[row] - Scalar::from(digit as u64);
            }
            bx_eval += pow_usize(beta, row + 1) * term;
        }
        let b2 = bx_eval == proof.y_b * l0_value;

        let djbj = inner_product(&proof.djx, &proof.bjx)?;
        let eq3_lhs = commit_to(params, proof.y_s * l0_value + djbj, proof.eta2);
        let mut eq3_rhs = proof.c_r2.mul_scalar(l0_value);
        for col in 0..v {
            eq3_rhs = eq3_rhs.add(&proof.c_w[col].mul_scalar(lkx_eval[col]));
        }
        let b3 = eq3_lhs == eq3_rhs;

        let sum_cw = sum_commitments(&proof.c_w);
        let b4 = if statement.tt == 1 {
            proof.witness_commitments[0] == sum_cw
        } else {
            let mut agg = Commitment::identity();
            for i in 0..statement.tt {
                agg = agg.add(&proof.witness_commitments[i].mul_scalar(pow_usize(gamma, i + 1)));
            }
            agg == sum_cw
        };

        Ok(b1 && b2 && b3 && b4)
    }
}

fn fill_type3_matrices(
    statement: &Type3Statement,
    witness: &Type3Witness,
    base_biguint: &BigUint,
    base_powers: &[Scalar],
    gamma: Scalar,
    ds: &mut [Vec<Scalar>],
    bs_digits: &mut [Vec<Scalar>],
    bjk: &mut [Vec<Scalar>],
    dds: &mut [Vec<Scalar>],
) -> Result<(), ProofError> {
    let u = statement.u;
    let v = statement.v;
    let uv = u * v;

    if !statement.aggregated {
        let digits = decompose_to_nary_padded(&witness.values[0], base_biguint, uv)?;
        for i in 0..uv {
            let row = i / v;
            let col = i % v;
            if i < statement.nbits {
                let digit = biguint_to_scalar(&digits[i]);
                let bpow = base_powers[i];
                ds[col][row] = digit * bpow;
                bs_digits[row][col] = digit;
                bjk[row][col] = bpow;
                dds[col][row] = digit;
            }
        }
        return Ok(());
    }

    let actual_mod = (v / statement.tt) * u;
    let mut tt_counter = 0usize;
    let mut current_digits = Vec::new();
    for i in 0..uv {
        let row = i / v;
        let col = i % v;
        let mod_agg = i % actual_mod;
        if mod_agg == 0 && tt_counter < statement.tt {
            current_digits = decompose_to_nary_padded(
                &witness.values[tt_counter],
                base_biguint,
                uv / statement.tt,
            )?;
            tt_counter += 1;
        }

        let in_range = i < uv / statement.tt + (tt_counter - 1) * actual_mod
            && i >= (tt_counter - 1) * actual_mod;
        if in_range {
            if statement.tt == 4 && i >= u * (v - 1) {
                continue;
            }
            let bpow = pow_usize(gamma, tt_counter) * base_powers[mod_agg];
            let digit = biguint_to_scalar(&current_digits[mod_agg]);
            ds[col][row] = digit * bpow;
            bs_digits[row][col] = digit;
            bjk[row][col] = bpow;
            dds[col][row] = digit;
        }
    }

    Ok(())
}

fn validate_statement(
    statement: &Type3Statement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.u == 0 || statement.v == 0 {
        return Err(ProofError::InvalidStatement("U and V must be > 0"));
    }
    if statement.u != statement.v {
        return Err(ProofError::InvalidStatement(
            "Type3 currently requires U == V",
        ));
    }
    if statement.u != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement U must match params.gs length",
        ));
    }
    if statement.nbits == 0 || statement.nbits > statement.u * statement.v {
        return Err(ProofError::InvalidStatement(
            "nbits must satisfy 0 < nbits <= U*V",
        ));
    }
    if statement.tt == 0 {
        return Err(ProofError::InvalidStatement("tt must be > 0"));
    }
    if statement.aggregated && statement.tt <= 1 {
        return Err(ProofError::InvalidStatement(
            "aggregated mode requires tt > 1",
        ));
    }
    if statement.aggregated && (statement.v % statement.tt != 0) {
        return Err(ProofError::InvalidStatement(
            "aggregated Type3 requires V divisible by tt",
        ));
    }
    if statement.aggregated && statement.nbits < (statement.u * statement.v) / statement.tt {
        return Err(ProofError::InvalidStatement(
            "aggregated Type3 requires nbits >= (U*V)/tt",
        ));
    }
    Ok(())
}

fn validate_witness(statement: &Type3Statement, witness: &Type3Witness) -> Result<(), ProofError> {
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

fn validate_proof_shape(statement: &Type3Statement, proof: &Type3Proof) -> Result<(), ProofError> {
    let expected_ys = if statement.aggregated {
        statement.tt
    } else {
        1
    };
    if proof.witness_commitments.len() != expected_ys {
        return Err(ProofError::InvalidProof(
            "invalid witness commitments length",
        ));
    }
    if proof.c_d.len() != statement.v || proof.c_w.len() != statement.v {
        return Err(ProofError::InvalidProof("invalid c_d/c_w length"));
    }
    if proof.djx.len() != statement.u || proof.bjx.len() != statement.u {
        return Err(ProofError::InvalidProof("invalid djx/bjx length"));
    }
    if proof.zs.len() != statement.v + 1 {
        return Err(ProofError::InvalidProof(
            "invalid interpolation point count",
        ));
    }
    if proof.cm_s.is_empty()
        || proof.cm_b.is_empty()
        || proof.pi_sx.is_empty()
        || proof.pi_bx.is_empty()
    {
        return Err(ProofError::InvalidProof(
            "polynomial commitment/proof vectors must be non-empty",
        ));
    }
    Ok(())
}

fn challenge_gamma(witness_commitments: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type3-gamma", mode);
    for commitment in witness_commitments {
        transcript.append_point(b"W", commitment.point());
    }
    transcript.challenge_scalar(b"gamma")
}

fn challenge_beta(
    c_d: &[Commitment],
    c_w: &[Commitment],
    c_r1: &Commitment,
    c_r2: &Commitment,
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type3-beta", mode);
    for commitment in c_d {
        transcript.append_point(b"cd", commitment.point());
    }
    for commitment in c_w {
        transcript.append_point(b"cw", commitment.point());
    }
    transcript.append_point(b"R1", c_r1.point());
    transcript.append_point(b"R2", c_r2.point());
    transcript.challenge_scalar(b"beta")
}

fn matrix_dims_square(degree: usize) -> (usize, usize) {
    if degree == 0 {
        return (0, 0);
    }
    let mut m = (degree as f64).sqrt().ceil() as usize;
    let mut n = m;
    if degree < m * n {
        n = n.saturating_sub(1);
        if degree < m * n {
            m = m.saturating_sub(1);
        }
    }
    (m, n)
}

fn lagrange_polynomial(zs: &[Scalar], index: usize) -> Result<Polynomial, ProofError> {
    if index >= zs.len() {
        return Err(ProofError::InvalidProof(
            "invalid lagrange polynomial index",
        ));
    }
    let z_n = zs[index];
    let mut numerator = poly_constant(Scalar::from(1u64));
    let mut denominator = Scalar::from(1u64);
    for (i, z_i) in zs.iter().enumerate() {
        if i == index {
            continue;
        }
        numerator = numerator.mul(&poly_x_minus(*z_i));
        denominator *= z_n - *z_i;
    }
    let denominator_inv = denominator.inverse().ok_or(ProofError::InvalidProof(
        "lagrange denominator inverse does not exist",
    ))?;
    Ok(poly_mul_scalar(&numerator, denominator_inv))
}

fn l0_polynomial(zs: &[Scalar]) -> Polynomial {
    let mut out = poly_constant(Scalar::from(1u64));
    for z in zs {
        out = out.mul(&poly_x_minus(*z));
    }
    out
}

fn poly_constant(value: Scalar) -> Polynomial {
    Polynomial::from_coeffs(vec![value])
}

fn poly_x_minus(value: Scalar) -> Polynomial {
    Polynomial::from_coeffs(vec![-value, Scalar::from(1u64)])
}

fn poly_mul_scalar(poly: &Polynomial, scalar: Scalar) -> Polynomial {
    poly.mul(&poly_constant(scalar))
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

fn build_base_powers(nbits: usize, base: Scalar) -> Vec<Scalar> {
    (0..nbits).map(|i| pow_usize(base, i)).collect()
}

fn biguint_to_scalar(value: &BigUint) -> Scalar {
    Scalar::from_be_bytes_mod_order(&value.to_bytes_be())
}

fn map_poly_err(_: verange_poly_commit::PolyCommitError) -> ProofError {
    ProofError::InvalidProof("polynomial commitment operation failed")
}

fn pow_usize(base: Scalar, exp: usize) -> Scalar {
    base.pow([exp as u64])
}

fn random_scalar(rng: &mut impl RngCore) -> Scalar {
    let mut bytes = [0u8; 64];
    rng.fill_bytes(&mut bytes);
    Scalar::from_be_bytes_mod_order(&bytes)
}
