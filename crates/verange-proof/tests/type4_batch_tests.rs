use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type4_batch::{
    Type4BatchProver, Type4BatchStatement, Type4BatchVerifier, Type4BatchWitness,
};
use verange_proof::ProofError;

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn type4_batch_tests_valid_proof_verifies() {
    let params = sample_params(32);
    let statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };

    let mut rng = ChaCha20Rng::from_seed([81u8; 32]);
    let proof = Type4BatchProver::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove");

    assert!(
        Type4BatchVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
            .expect("verify")
    );
}

#[test]
fn type4_batch_tests_tamper_fails() {
    let params = sample_params(32);
    let statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness = Type4BatchWitness {
        value: BigUint::from(901u32),
    };

    let mut rng = ChaCha20Rng::from_seed([91u8; 32]);
    let mut proof = Type4BatchProver::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove");
    proof.rprime_2 += Fr::from(1u64);

    assert!(
        !Type4BatchVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
            .expect("verify")
    );
}

#[test]
fn type4_batch_tests_k_mismatch_is_rejected() {
    let params = sample_params(32);
    let statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };

    let mut rng = ChaCha20Rng::from_seed([101u8; 32]);
    let proof = Type4BatchProver::prove(
        &statement,
        &witness,
        &params,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove");

    let mismatched_statement = Type4BatchStatement {
        nbits: 16,
        k: 999,
        l: 32,
        b: 8,
    };
    let err = Type4BatchVerifier::verify(
        &mismatched_statement,
        &proof,
        &params,
        TranscriptMode::JavaCompat,
    )
    .expect_err("verify should reject mismatched Type4_batch k");
    assert!(matches!(err, ProofError::InvalidStatement(_)));
}
