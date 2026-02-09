use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type2::{Type2Prover, Type2Statement, Type2Verifier, Type2Witness};

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn type2_tests_valid_aggregated_proof_verifies() {
    let params = sample_params(4);
    let statement = Type2Statement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let witness = Type2Witness {
        values: vec![BigUint::from(181u32), BigUint::from(77u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([21u8; 32]);
    let proof = Type2Prover::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove");

    assert!(
        Type2Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
            .expect("verify")
    );
}

#[test]
fn type2_tests_tamper_fails_verification() {
    let params = sample_params(4);
    let statement = Type2Statement {
        nbits: 8,
        k: 2,
        l: 4,
        b: 4,
        tt: 1,
        aggregated: false,
    };
    let witness = Type2Witness {
        values: vec![BigUint::from(99u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([31u8; 32]);
    let mut proof = Type2Prover::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove");
    proof.eta2 += Fr::from(1u64);

    assert!(
        !Type2Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
            .expect("verify")
    );
}
