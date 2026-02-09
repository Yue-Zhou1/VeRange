use ark_bn254::Fr;
use verange_poly_commit::commit::{
    commit_poly, open_poly, open_poly_batch, verify_poly, verify_poly_batch, PolyCommitParams,
};
use verange_poly_commit::polynomial::{vanishing_polynomial, Polynomial};

#[test]
fn polynomial_commit_tests_polynomial_arithmetic() {
    let a = Polynomial::from_coeffs(vec![Fr::from(1u64), Fr::from(2u64)]);
    let b = Polynomial::from_coeffs(vec![Fr::from(3u64), Fr::from(4u64)]);

    let sum = a.add(&b);
    let diff = b.sub(&a);
    let prod = a.mul(&b);

    assert_eq!(sum.coeffs(), &[Fr::from(4u64), Fr::from(6u64)]);
    assert_eq!(diff.coeffs(), &[Fr::from(2u64), Fr::from(2u64)]);
    assert_eq!(
        prod.coeffs(),
        &[Fr::from(3u64), Fr::from(10u64), Fr::from(8u64)]
    );

    let numerator = Polynomial::from_coeffs(vec![Fr::from(-1i64), Fr::from(0u64), Fr::from(1u64)]);
    let denominator = Polynomial::from_coeffs(vec![Fr::from(-1i64), Fr::from(1u64)]);
    let (q, r) = numerator.long_divide(&denominator).expect("division");

    assert_eq!(q.coeffs(), &[Fr::from(1u64), Fr::from(1u64)]);
    assert_eq!(r.coeffs(), &[Fr::from(0u64)]);
}

#[test]
fn polynomial_commit_tests_vanishing_polynomial() {
    let z = vanishing_polynomial(4);

    assert_eq!(z.degree(), 4);
    assert_eq!(z.coeffs()[0], Fr::from(-1i64));
    assert_eq!(z.coeffs()[4], Fr::from(1u64));
}

#[test]
fn polynomial_commit_tests_single_poly_commit_open_verify() {
    let m = 1usize;
    let n = 2usize;
    let degree = 2usize;

    let params = PolyCommitParams::new(n + 1).expect("params");
    let poly = Polynomial::from_coeffs(vec![Fr::from(5u64), Fr::from(7u64), Fr::from(11u64)]);
    let blind = vec![Fr::from(13u64), Fr::from(17u64)];

    let cm = commit_poly(&poly, m, n, degree, &blind, &params).expect("commit");

    let x = Fr::from(9u64);
    let y = poly.evaluate(x);
    let pi = open_poly(&poly, x, m, n, degree, &blind).expect("open");

    assert!(verify_poly(&cm, x, y, &pi, degree, &params).expect("verify"));
}

#[test]
fn polynomial_commit_tests_batch_verify_relation() {
    let m = 1usize;
    let n = 2usize;
    let degree = 2usize;

    let params = PolyCommitParams::new(n + 1).expect("params");

    let p1 = Polynomial::from_coeffs(vec![Fr::from(2u64), Fr::from(3u64), Fr::from(5u64)]);
    let p2 = Polynomial::from_coeffs(vec![Fr::from(7u64), Fr::from(11u64), Fr::from(13u64)]);

    let b1 = vec![Fr::from(19u64), Fr::from(23u64)];
    let b2 = vec![Fr::from(29u64), Fr::from(31u64)];

    let cm1 = commit_poly(&p1, m, n, degree, &b1, &params).expect("cm1");
    let cm2 = commit_poly(&p2, m, n, degree, &b2, &params).expect("cm2");

    let x = Fr::from(37u64);
    let rho = Fr::from(41u64);

    let y1 = p1.evaluate(x);
    let y2 = p2.evaluate(x);
    let y_batch = rho * y1 + (rho * rho) * y2;

    let pi_batch = open_poly_batch(&[p1, p2], x, rho, m, n, degree, &[b1, b2]).expect("batch open");

    assert!(verify_poly_batch(&[cm1, cm2], x, rho, y_batch, &pi_batch, degree, &params).expect("batch verify"));
}
