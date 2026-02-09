use crate::utils::{
    biguint_to_scalar, commit_with_basis_and_h, invert_or_err, pow_usize, random_scalar,
};
use crate::ProofError;
use ark_ff::{One, Zero};
use num_bigint::BigUint;
use rand_core::RngCore;
use verange_core::arith::decompose_to_nary;
use verange_core::commit_to;
use verange_core::commitment::Commitment;
use verange_core::curve::{CurvePoint, Scalar};
use verange_core::transcript::{Transcript, TranscriptMode};
use verange_core::PedersenParams;
use verange_poly_commit::ntt;
use verange_poly_commit::polynomial::{vanishing_polynomial, Polynomial};

#[derive(Clone, Debug)]
pub struct Type4BatchStatement {
    pub nbits: usize,
    pub k: usize,
    pub l: usize,
    pub b: usize,
}

#[derive(Clone, Debug)]
pub struct Type4BatchWitness {
    pub value: BigUint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Type4BatchProof {
    pub w: Commitment,
    pub cm_a: Commitment,
    pub cm_e: Commitment,
    pub cm_aprime: Commitment,
    pub cm_eprime: Commitment,
    pub com_g: Vec<Commitment>,
    pub com_p: Vec<Commitment>,
    pub com_q: Vec<Commitment>,
    pub b: Scalar,
    pub eprime: Scalar,
    pub g_eval_x: Scalar,
    pub g_eval_wx: Scalar,
    pub f_eval_x: Scalar,
    pub fprime_eval_x: Scalar,
    pub p_eval_x: Scalar,
    pub rprime_1: Scalar,
    pub rprime_2: Scalar,
    pub rprime_3: Scalar,
    pub rprime_4: Scalar,
    pub qv: Vec<Scalar>,
    pub align_degree: usize,
}

pub struct Type4BatchProver;
pub struct Type4BatchVerifier;

impl Type4BatchProver {
    pub fn prove(
        statement: &Type4BatchStatement,
        witness: &Type4BatchWitness,
        params: &PedersenParams,
        mode: TranscriptMode,
        rng: &mut impl RngCore,
    ) -> Result<Type4BatchProof, ProofError> {
        validate_statement(statement, params)?;

        let domain = ntt::domain(statement.nbits).map_err(map_poly_err)?;
        let witness_scalar = biguint_to_scalar(&witness.value);
        let r_cw = random_scalar(rng);
        let w = commit_to(params, witness_scalar, r_cw);

        let _alpha = random_scalar(rng);
        let _beta = random_scalar(rng);
        let g = compute_g(&domain, &witness.value, statement.b)?;

        let a = random_scalar(rng);
        let e = random_scalar(rng);
        let r_a = random_scalar(rng);
        let r_aprime = random_scalar(rng);
        let r_e = random_scalar(rng);
        let r_eprime = random_scalar(rng);

        let g_degree = g.degree();
        let align_degree = if statement.b * g_degree == 0 {
            0
        } else {
            statement.b * g_degree - 1
        };
        let (m, n) = calculate_matrix_mn(align_degree);
        let blindings = (0..n).map(|_| random_scalar(rng)).collect::<Vec<_>>();
        let h_g = setup_hijs(&g, m, n, align_degree, &blindings)?;
        let com_g = commit_h_matrix(&h_g, &params.gs)?;

        let cm_a = commit_to(params, a, r_a);
        let cm_e = commit_to(params, e, r_e);
        let cm_aprime = commit_with_first_generator_and_h(params, a, r_aprime)?;
        let cm_eprime = commit_with_first_generator_and_h(params, e, r_eprime)?;

        let gamma = challenge_gamma(&com_g, mode);
        let theta = gamma * gamma;
        let gprime = g.add(&poly_constant(gamma * a));

        let f = compute_f_batch(&domain, &g, statement.b)?;
        let fprime = compute_fprime(&g, statement.b);
        let p = compute_p(&domain, &f, &fprime, theta)?;

        let b = gprime.evaluate(Scalar::one());
        let eprime = a + gamma * e;

        let h_p = setup_hijs(&p, m, n, align_degree, &blindings)?;
        let com_p = commit_h_matrix(&h_p, &params.gs)?;

        let x = challenge_x(&com_g, &com_p, mode);
        let x_omega = x * domain[1];

        let g_eval_x = g.evaluate(x);
        let g_eval_wx = g.evaluate(x_omega);
        let f_eval_x = f.evaluate(x);
        let fprime_eval_x = fprime.evaluate(x);
        let p_eval_x = p.evaluate(x);

        let rs = compute_rs(g_eval_x, g_eval_wx, b, p_eval_x, x, x_omega)?;
        let rho = x * x;
        let q = compute_q_batch(&rs, &g, &gprime, &p, rho, x, x_omega)?;

        let h_q = setup_hijs(&q, m, n, align_degree, &blindings)?;
        let com_q = commit_h_matrix(&h_q, &params.gs)?;

        let z = challenge_z(&com_g, &com_p, &com_q, mode);

        let z_minus_1 = z - Scalar::one();
        let z_minus_x = z - x;
        let z_minus_wx = z - x_omega;
        let rho2 = rho * rho;

        let mut qv = Vec::with_capacity(n + 1);
        for i in 0..=n {
            let mut qv_i = Scalar::zero();
            for j in 0..=m {
                let part_g = h_g[j][i] * (z_minus_1 + rho * z_minus_x * z_minus_wx);
                let part_p = h_p[j][i] * rho2 * z_minus_1 * z_minus_wx;
                let part_q = h_q[j][i] * z_minus_1 * z_minus_x * z_minus_wx;
                qv_i += (part_g + part_p - part_q) * pow_usize(z, j);
            }
            if i == 0 {
                qv_i += rho * gamma * a * z_minus_x * z_minus_wx;
            }
            qv.push(qv_i);
        }

        if qv.len() > params.gs.len() {
            return Err(ProofError::InvalidStatement(
                "params.gs length is too small for Type4_batch qv vector",
            ));
        }

        let rprime_1 = rho * gamma * z_minus_x * z_minus_wx * r_aprime;
        let rprime_2 = r_cw + gamma * r_a;
        let rprime_3 = r_a + gamma * r_e;
        let rprime_4 = r_aprime + gamma * r_eprime;

        Ok(Type4BatchProof {
            w,
            cm_a,
            cm_e,
            cm_aprime,
            cm_eprime,
            com_g,
            com_p,
            com_q,
            b,
            eprime,
            g_eval_x,
            g_eval_wx,
            f_eval_x,
            fprime_eval_x,
            p_eval_x,
            rprime_1,
            rprime_2,
            rprime_3,
            rprime_4,
            qv,
            align_degree,
        })
    }
}

impl Type4BatchVerifier {
    pub fn verify(
        statement: &Type4BatchStatement,
        proof: &Type4BatchProof,
        params: &PedersenParams,
        mode: TranscriptMode,
    ) -> Result<bool, ProofError> {
        validate_statement(statement, params)?;
        validate_proof_shape(proof, params)?;

        let domain = ntt::domain(statement.nbits).map_err(map_poly_err)?;
        let gamma = challenge_gamma(&proof.com_g, mode);
        let theta = gamma * gamma;
        let x = challenge_x(&proof.com_g, &proof.com_p, mode);
        let rho = x * x;
        let z = challenge_z(&proof.com_g, &proof.com_p, &proof.com_q, mode);

        let x_omega = x * domain[1];
        let z_minus_1 = z - Scalar::one();
        let z_minus_x = z - x;
        let z_minus_wx = z - x_omega;
        let rho2 = rho * rho;

        let rho_z_x_z_wx = rho * z_minus_x * z_minus_wx;
        let rho2_z_1_z_wx = rho2 * z_minus_1 * z_minus_wx;
        let z_1_z_x_z_wx_negate = -(z_minus_1 * z_minus_x * z_minus_wx);

        let m = proof.com_p.len() - 1;
        let n = proof.qv.len() - 1;
        if proof.align_degree < m * n {
            return Err(ProofError::InvalidProof(
                "align_degree must be >= m*n in Type4_batch proof",
            ));
        }
        let eta = proof.align_degree - m * n;

        let eq1_lhs = commit_with_basis_and_h(
            &params.gs[..proof.qv.len()],
            &proof.qv,
            &params.h,
            proof.rprime_1,
        )?;
        let mut eq1_rhs = Commitment::identity();
        for (i, ((cg, cp), cq)) in proof
            .com_g
            .iter()
            .zip(proof.com_p.iter())
            .zip(proof.com_q.iter())
            .enumerate()
        {
            let z_u = pow_usize(z, i);
            let h_gpq = cg
                .mul_scalar((z_minus_1 + rho_z_x_z_wx) * z_u)
                .add(&cp.mul_scalar(rho2_z_1_z_wx * z_u))
                .add(&cq.mul_scalar(z_1_z_x_z_wx_negate * z_u));
            eq1_rhs = eq1_rhs.add(&h_gpq);
        }
        eq1_rhs = eq1_rhs.add(&proof.cm_aprime.mul_scalar(rho_z_x_z_wx * gamma));
        let b1 = eq1_lhs == eq1_rhs;

        let rs = compute_rs(
            proof.g_eval_x,
            proof.g_eval_wx,
            proof.b,
            proof.p_eval_x,
            x,
            x_omega,
        )?;
        let eq2_2 = rs[0].evaluate(z) * z_minus_1
            + rs[1].evaluate(z) * rho_z_x_z_wx
            + rs[2].evaluate(z) * rho2_z_1_z_wx;

        let mut eq2_1 = proof.qv[0];
        for (i, coeff) in proof.qv.iter().enumerate().skip(1) {
            let exp = (i - 1) * m + eta;
            eq2_1 += *coeff * pow_usize(z, exp);
        }
        let b2 = eq2_1 == eq2_2;

        let gx_bgwx = proof.g_eval_x - Scalar::from(statement.b as u64) * proof.g_eval_wx;
        let mut eq3_2 = gx_bgwx;
        let mut eq4_2 = proof.g_eval_x;
        for i in 1..statement.b {
            let i_scalar = Scalar::from(i as u64);
            eq3_2 *= i_scalar - gx_bgwx;
            eq4_2 *= i_scalar - proof.g_eval_x;
        }
        let b3 = proof.f_eval_x == eq3_2;
        let b4 = proof.fprime_eval_x == eq4_2;

        let omega_n_minus_1 = domain[domain.len() - 1];
        let x_n_minus_1_inv = invert_or_err(
            pow_usize(x, statement.nbits) - Scalar::one(),
            "x^N - 1 inverse does not exist",
        )?;
        let x_minus_omega_inv = invert_or_err(
            x - omega_n_minus_1,
            "x - omega_(N-1) inverse does not exist",
        )?;
        let eq5_2 = proof.f_eval_x * (x - omega_n_minus_1) * x_n_minus_1_inv
            + theta * proof.fprime_eval_x * x_minus_omega_inv;
        let b5 = proof.p_eval_x == eq5_2;

        let eq6_lhs = commit_to(params, proof.b, proof.rprime_2);
        let eq6_rhs = proof.w.add(&proof.cm_a.mul_scalar(gamma));
        let b6 = eq6_lhs == eq6_rhs;

        let eq7_lhs = commit_to(params, proof.eprime, proof.rprime_3);
        let eq7_rhs = proof.cm_a.add(&proof.cm_e.mul_scalar(gamma));
        let b7 = eq7_lhs == eq7_rhs;

        let eq8_lhs = commit_with_first_generator_and_h(params, proof.eprime, proof.rprime_4)?;
        let eq8_rhs = proof.cm_aprime.add(&proof.cm_eprime.mul_scalar(gamma));
        let b8 = eq8_lhs == eq8_rhs;

        Ok(b1 && b2 && b3 && b4 && b5 && b6 && b7 && b8)
    }
}

fn validate_statement(
    statement: &Type4BatchStatement,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if statement.nbits == 0 || !statement.nbits.is_power_of_two() {
        return Err(ProofError::InvalidStatement(
            "Type4_batch nbits must be a non-zero power of two",
        ));
    }
    if statement.b < 2 {
        return Err(ProofError::InvalidStatement("base B must be >= 2"));
    }
    if statement.k == 0 {
        return Err(ProofError::InvalidStatement("K must be > 0"));
    }
    if statement.k != 4 {
        return Err(ProofError::InvalidStatement(
            "Type4_batch currently requires K == 4",
        ));
    }
    if statement.l != params.gs.len() {
        return Err(ProofError::InvalidStatement(
            "statement L must match params.gs length",
        ));
    }
    let extended = statement
        .b
        .checked_mul(statement.nbits)
        .ok_or(ProofError::InvalidStatement("B * nbits overflows usize"))?;
    if !extended.is_power_of_two() {
        return Err(ProofError::InvalidStatement(
            "Type4_batch requires B * nbits to be a power of two",
        ));
    }
    Ok(())
}

fn validate_proof_shape(
    proof: &Type4BatchProof,
    params: &PedersenParams,
) -> Result<(), ProofError> {
    if proof.com_g.is_empty()
        || proof.com_p.is_empty()
        || proof.com_q.is_empty()
        || proof.qv.is_empty()
    {
        return Err(ProofError::InvalidProof(
            "commitment vectors and qv must be non-empty",
        ));
    }
    if proof.com_g.len() != proof.com_p.len() || proof.com_g.len() != proof.com_q.len() {
        return Err(ProofError::InvalidProof(
            "com_g/com_p/com_q must have same length",
        ));
    }
    if proof.qv.len() > params.gs.len() {
        return Err(ProofError::InvalidProof(
            "qv vector exceeds available gs generators",
        ));
    }
    Ok(())
}

fn challenge_gamma(com_g: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type4-batch-gamma", mode);
    for commitment in com_g {
        transcript.append_point(b"cg", commitment.point());
    }
    transcript.challenge_scalar(b"gamma")
}

fn challenge_x(com_g: &[Commitment], com_p: &[Commitment], mode: TranscriptMode) -> Scalar {
    let mut transcript = Transcript::new(b"type4-batch-x", mode);
    for commitment in com_g {
        transcript.append_point(b"cg", commitment.point());
    }
    for commitment in com_p {
        transcript.append_point(b"cp", commitment.point());
    }
    transcript.challenge_scalar(b"x")
}

fn challenge_z(
    com_g: &[Commitment],
    com_p: &[Commitment],
    com_q: &[Commitment],
    mode: TranscriptMode,
) -> Scalar {
    let mut transcript = Transcript::new(b"type4-batch-z", mode);
    for commitment in com_g {
        transcript.append_point(b"cg", commitment.point());
    }
    for commitment in com_p {
        transcript.append_point(b"cp", commitment.point());
    }
    for commitment in com_q {
        transcript.append_point(b"cq", commitment.point());
    }
    transcript.challenge_scalar(b"z")
}

fn calculate_matrix_mn(degree: usize) -> (usize, usize) {
    if degree == 0 {
        return (0, 0);
    }
    let mut m = ((degree / 3) as f64).sqrt().ceil() as usize;
    let mut n = 3 * m;
    if degree < m * n {
        m = m.saturating_sub(1);
        while degree < m * n {
            n = n.saturating_sub(1);
            if n == 0 {
                break;
            }
        }
    }
    (m, n)
}

fn compute_g(domain: &[Scalar], value: &BigUint, base: usize) -> Result<Polynomial, ProofError> {
    let base_biguint = BigUint::from(base as u64);
    let mut digits = decompose_to_nary(value, &base_biguint)?;
    if digits.is_empty() {
        digits.push(BigUint::from(0u64));
    }
    let digits = digits.iter().map(biguint_to_scalar).collect::<Vec<_>>();

    let mut evaluations = Vec::with_capacity(domain.len());
    let mut pre_eval = *digits
        .last()
        .ok_or(ProofError::InvalidWitness("witness decomposition is empty"))?;
    evaluations.push(pre_eval);
    for idx in (0..digits.len() - 1).rev() {
        let eval = Scalar::from(base as u64) * pre_eval + digits[idx];
        evaluations.push(eval);
        pre_eval = eval;
    }
    evaluations.reverse();
    while evaluations.len() < domain.len() {
        evaluations.push(Scalar::zero());
    }

    let mut coeffs = evaluations;
    ntt::ntt(&mut coeffs, true).map_err(map_poly_err)?;
    Ok(Polynomial::from_coeffs(coeffs))
}

fn compute_f_batch(
    domain: &[Scalar],
    g: &Polynomial,
    base: usize,
) -> Result<Polynomial, ProofError> {
    let extended_size = base * domain.len();
    let domain_bn = ntt::domain(extended_size).map_err(map_poly_err)?;
    let w_root = domain[1];

    let mut evals = Vec::with_capacity(domain_bn.len());
    for point in &domain_bn {
        let g_x = g.evaluate(*point);
        let g_xw = g.evaluate(*point * w_root);
        let part_a = g_x - Scalar::from(base as u64) * g_xw;
        let mut part_b = Scalar::one();
        for j in 1..base {
            part_b *= Scalar::from(j as u64) - part_a;
        }
        evals.push(part_a * part_b);
    }

    ntt::ntt(&mut evals, true).map_err(map_poly_err)?;
    Ok(Polynomial::from_coeffs(evals))
}

fn compute_fprime(g: &Polynomial, base: usize) -> Polynomial {
    let mut poly = g.clone();
    for i in 1..base {
        poly = poly.mul(&poly_constant(Scalar::from(i as u64)).sub(g));
    }
    poly
}

fn compute_p(
    domain: &[Scalar],
    f: &Polynomial,
    fprime: &Polynomial,
    theta: Scalar,
) -> Result<Polynomial, ProofError> {
    let x_n_minus_1 = vanishing_polynomial(domain.len());
    let x_minus_w_n_minus_1 = poly_x_minus(domain[domain.len() - 1]);
    let fx_part = f
        .mul(&x_minus_w_n_minus_1)
        .long_divide(&x_n_minus_1)
        .map_err(map_poly_err)?
        .0;
    let fprime_part = poly_mul_scalar(fprime, theta)
        .long_divide(&x_minus_w_n_minus_1)
        .map_err(map_poly_err)?
        .0;
    Ok(fx_part.add(&fprime_part))
}

fn compute_rs(
    g_x: Scalar,
    g_wx: Scalar,
    b: Scalar,
    p_x: Scalar,
    x: Scalar,
    wx: Scalar,
) -> Result<Vec<Polynomial>, ProofError> {
    let gx_factor = g_x * invert_or_err(x - wx, "x - wx inverse does not exist")?;
    let gwx_factor = g_wx * invert_or_err(wx - x, "wx - x inverse does not exist")?;

    let r0 = poly_mul_scalar(&poly_x_minus(wx), gx_factor)
        .add(&poly_mul_scalar(&poly_x_minus(x), gwx_factor));
    let r1 = poly_constant(b);
    let r2 = poly_constant(p_x);
    Ok(vec![r0, r1, r2])
}

fn compute_q_batch(
    rs: &[Polynomial],
    g: &Polynomial,
    gprime: &Polynomial,
    p: &Polynomial,
    rho: Scalar,
    x: Scalar,
    wx: Scalar,
) -> Result<Polynomial, ProofError> {
    let x_minus_x = poly_x_minus(x);
    let x_minus_wx = poly_x_minus(wx);
    let x_minus_1 = poly_x_minus(Scalar::one());
    let x_minus_x_x_minus_wx = x_minus_x.mul(&x_minus_wx);

    let part1 = g
        .sub(&rs[0])
        .long_divide(&x_minus_x_x_minus_wx)
        .map_err(map_poly_err)?
        .0;
    let part2 = poly_mul_scalar(&gprime.sub(&rs[1]), rho)
        .long_divide(&x_minus_1)
        .map_err(map_poly_err)?
        .0;
    let part3 = poly_mul_scalar(&p.sub(&rs[2]), rho * rho)
        .long_divide(&x_minus_x)
        .map_err(map_poly_err)?
        .0;
    Ok(part1.add(&part2).add(&part3))
}

fn setup_hijs(
    poly: &Polynomial,
    m: usize,
    n: usize,
    degree: usize,
    blindings: &[Scalar],
) -> Result<Vec<Vec<Scalar>>, ProofError> {
    if blindings.len() < n {
        return Err(ProofError::InvalidProof(
            "blinding vector is too short for setup_hijs",
        ));
    }
    if degree < poly.degree() || degree < m * n {
        return Err(ProofError::InvalidProof("invalid degree for setup_hijs"));
    }

    let mut coeffs = poly.coeffs().to_vec();
    while coeffs.len() <= degree {
        coeffs.push(Scalar::zero());
    }

    let eta = degree - (m * n);
    let mut h = vec![vec![Scalar::zero(); n + 1]; m + 1];

    h[0][0] = if eta == 0 && n > 0 {
        coeffs[0] - blindings[0]
    } else {
        coeffs[0]
    };
    if n > 0 {
        h[0][1..(n + 1)].copy_from_slice(&blindings[..n]);
    }

    for i in 1..=m {
        h[i][0] = if eta == 0 {
            Scalar::zero()
        } else if i < eta {
            coeffs[i]
        } else if i == eta && n > 0 {
            coeffs[i] - blindings[0]
        } else if i == eta {
            coeffs[i]
        } else {
            Scalar::zero()
        };
    }

    let mut flag = eta;
    for i in 1..=n {
        for row in h.iter_mut().take(m + 1).skip(1) {
            flag += 1;
            row[i] = coeffs[flag];
        }
    }

    if n > 0 {
        for (slot, blinding) in h[m][1..n].iter_mut().zip(blindings.iter().skip(1)) {
            *slot -= *blinding;
        }
    }

    Ok(h)
}

fn commit_with_first_generator_and_h(
    params: &PedersenParams,
    message: Scalar,
    blinding: Scalar,
) -> Result<Commitment, ProofError> {
    let first = params
        .gs
        .first()
        .ok_or(ProofError::InvalidStatement("params.gs must not be empty"))?;
    Ok(Commitment::new(*first * message + params.h * blinding))
}

fn commit_h_matrix(
    h: &[Vec<Scalar>],
    generators: &[CurvePoint],
) -> Result<Vec<Commitment>, ProofError> {
    let n = h
        .first()
        .ok_or(ProofError::InvalidProof("empty h matrix"))?
        .len();
    if generators.len() < n {
        return Err(ProofError::InvalidStatement(
            "params.gs length is too small for polynomial commitment rows",
        ));
    }
    let mut out = Vec::with_capacity(h.len());
    for row in h {
        out.push(commit_vector(&generators[..n], row)?);
    }
    Ok(out)
}

fn commit_vector(basis: &[CurvePoint], coeffs: &[Scalar]) -> Result<Commitment, ProofError> {
    if basis.len() != coeffs.len() {
        return Err(ProofError::InvalidProof(
            "basis and coefficient lengths must match in vector commitment",
        ));
    }
    let mut point = CurvePoint::default();
    for (g, c) in basis.iter().zip(coeffs.iter()) {
        point += *g * *c;
    }
    Ok(Commitment::new(point))
}

fn poly_constant(value: Scalar) -> Polynomial {
    Polynomial::from_coeffs(vec![value])
}

fn poly_x_minus(value: Scalar) -> Polynomial {
    Polynomial::from_coeffs(vec![-value, Scalar::one()])
}

fn poly_mul_scalar(poly: &Polynomial, scalar: Scalar) -> Polynomial {
    poly.mul(&poly_constant(scalar))
}

fn map_poly_err(_: verange_poly_commit::PolyCommitError) -> ProofError {
    ProofError::InvalidProof("polynomial operation failed")
}
