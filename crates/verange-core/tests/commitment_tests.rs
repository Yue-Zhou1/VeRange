use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use verange_core::{commitment::Commitment, commit_to, CoreError, PedersenParams};

fn sample_params() -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..8)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();

    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn commitment_tests_commit_matches_linear_combination() {
    let params = sample_params();
    let m = Fr::from(11u64);
    let r = Fr::from(29u64);

    let c = commit_to(&params, m, r);
    let expected = Commitment::new(params.g * m + params.h * r);

    assert_eq!(c, expected);
}

#[test]
fn commitment_tests_commitment_is_additively_homomorphic() {
    let params = sample_params();
    let m1 = Fr::from(3u64);
    let r1 = Fr::from(5u64);
    let m2 = Fr::from(13u64);
    let r2 = Fr::from(17u64);

    let c1 = commit_to(&params, m1, r1);
    let c2 = commit_to(&params, m2, r2);

    let lhs = c1.add(&c2);
    let rhs = commit_to(&params, m1 + m2, r1 + r2);

    assert_eq!(lhs, rhs);
}

#[test]
fn commitment_tests_identity_and_basis_checks_behave() {
    let params = sample_params();
    let c = commit_to(&params, Fr::from(2u64), Fr::from(9u64));

    assert_eq!(Commitment::identity().add(&c), c);
    assert!(params.require_generators(4).is_ok());

    let err = params.require_generators(16).expect_err("should fail");
    assert!(matches!(
        err,
        CoreError::InsufficientGenerators {
            required: 16,
            available: 8
        }
    ));
}
