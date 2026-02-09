use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};

fn sample_params(u: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..u)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn type3_tests_valid_aggregated_case_verifies() {
    let params = sample_params(4);
    let statement = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let witness = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([61u8; 32]);
    let proof = Type3Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove");

    assert!(Type3Verifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verify"));
}

#[test]
fn type3_tests_tamper_fails() {
    let params = sample_params(4);
    let statement = Type3Statement {
        nbits: 8,
        u: 4,
        v: 2,
        b: 4,
        tt: 1,
        aggregated: false,
    };
    let witness = Type3Witness {
        values: vec![BigUint::from(119u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([71u8; 32]);
    let mut proof = Type3Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove");
    proof.inner.eta1 += Fr::from(1u64);

    assert!(!Type3Verifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verify"));
}
