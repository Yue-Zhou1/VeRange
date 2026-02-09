use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type2p::{Type2PProver, Type2PStatement, Type2PVerifier, Type2PWitness};

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn type2p_tests_k3_case_verifies() {
    let params = sample_params(4);
    let statement = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let witness = Type2PWitness {
        values: vec![BigUint::from(1234u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([41u8; 32]);
    let proof = Type2PProver::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove");

    assert!(Type2PVerifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verify"));
}

#[test]
fn type2p_tests_k4_case_verifies() {
    let params = sample_params(4);
    let statement = Type2PStatement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let witness = Type2PWitness {
        values: vec![BigUint::from(4321u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([51u8; 32]);
    let proof = Type2PProver::prove(&statement, &witness, &params, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove");

    assert!(Type2PVerifier::verify(
        &statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat
    )
    .expect("verify"));
}
