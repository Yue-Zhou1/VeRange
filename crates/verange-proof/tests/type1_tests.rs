use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::PedersenParams;
use verange_core::transcript::TranscriptMode;
use verange_proof::type1::{Type1Prover, Type1Statement, Type1Verifier, Type1Witness};

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn type1_tests_valid_proof_verifies() {
    let params = sample_params(4);
    let statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let witness = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([7u8; 32]);
    let proof = Type1Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("proof generation");

    assert!(Type1Verifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verification"));
}

#[test]
fn type1_tests_tampered_proof_fails() {
    let params = sample_params(4);
    let statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let witness = Type1Witness {
        values: vec![BigUint::from(91u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([11u8; 32]);
    let mut proof = Type1Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("proof generation");

    proof.eta1 += Fr::from(1u64);

    assert!(!Type1Verifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verification"));
}

#[test]
fn type1_tests_aggregated_case_verifies() {
    let params = sample_params(4);
    let statement = Type1Statement {
        nbits: 16,
        k: 4,
        tt: 2,
        aggregated: true,
    };
    let witness = Type1Witness {
        values: vec![BigUint::from(120u32), BigUint::from(203u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([13u8; 32]);
    let proof = Type1Prover::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("proof generation");

    assert!(Type1Verifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verification"));
}
