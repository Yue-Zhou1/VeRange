use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type1::{Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use verange_proof::type2::{Type2Prover, Type2Statement, Type2Verifier, Type2Witness};
use verange_proof::type2p::{Type2PProver, Type2PStatement, Type2PVerifier, Type2PWitness};
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};
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

fn assert_cross_mode_rejected(result: Result<bool, ProofError>) {
    assert!(matches!(result, Ok(false) | Err(_)));
}

#[test]
fn mode_tests_proofs_are_mode_bound() {
    let mut rng = ChaCha20Rng::from_seed([11u8; 32]);

    let params_1 = sample_params(4);
    let stmt_1 = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let wit_1 = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };
    let proof_1_c = Type1Prover::prove(
        &stmt_1,
        &wit_1,
        &params_1,
        TranscriptMode::Canonical,
        &mut rng,
    )
    .expect("prove type1 canonical");
    assert!(
        Type1Verifier::verify(&stmt_1, &proof_1_c, &params_1, TranscriptMode::Canonical)
            .expect("verify type1 canonical")
    );
    assert_cross_mode_rejected(Type1Verifier::verify(
        &stmt_1,
        &proof_1_c,
        &params_1,
        TranscriptMode::JavaCompat,
    ));

    let proof_1_j = Type1Prover::prove(
        &stmt_1,
        &wit_1,
        &params_1,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type1 java");
    assert!(
        Type1Verifier::verify(&stmt_1, &proof_1_j, &params_1, TranscriptMode::JavaCompat)
            .expect("verify type1 java")
    );
    assert_cross_mode_rejected(Type1Verifier::verify(
        &stmt_1,
        &proof_1_j,
        &params_1,
        TranscriptMode::Canonical,
    ));

    let params_2 = sample_params(4);
    let stmt_2 = Type2Statement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let wit_2 = Type2Witness {
        values: vec![BigUint::from(181u32), BigUint::from(77u32)],
    };
    let proof_2_c = Type2Prover::prove(
        &stmt_2,
        &wit_2,
        &params_2,
        TranscriptMode::Canonical,
        &mut rng,
    )
    .expect("prove type2 canonical");
    assert!(
        Type2Verifier::verify(&stmt_2, &proof_2_c, &params_2, TranscriptMode::Canonical)
            .expect("verify type2 canonical")
    );
    assert_cross_mode_rejected(Type2Verifier::verify(
        &stmt_2,
        &proof_2_c,
        &params_2,
        TranscriptMode::JavaCompat,
    ));

    let proof_2_j = Type2Prover::prove(
        &stmt_2,
        &wit_2,
        &params_2,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2 java");
    assert!(
        Type2Verifier::verify(&stmt_2, &proof_2_j, &params_2, TranscriptMode::JavaCompat)
            .expect("verify type2 java")
    );
    assert_cross_mode_rejected(Type2Verifier::verify(
        &stmt_2,
        &proof_2_j,
        &params_2,
        TranscriptMode::Canonical,
    ));

    let params_2p = sample_params(4);
    let stmt_2p = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let wit_2p = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let proof_2p_c = Type2PProver::prove(
        &stmt_2p,
        &wit_2p,
        &params_2p,
        TranscriptMode::Canonical,
        &mut rng,
    )
    .expect("prove type2p canonical");
    assert!(
        Type2PVerifier::verify(
            &stmt_2p,
            &proof_2p_c,
            &params_2p,
            TranscriptMode::Canonical
        )
        .expect("verify type2p canonical")
    );
    assert_cross_mode_rejected(Type2PVerifier::verify(
        &stmt_2p,
        &proof_2p_c,
        &params_2p,
        TranscriptMode::JavaCompat,
    ));

    let proof_2p_j = Type2PProver::prove(
        &stmt_2p,
        &wit_2p,
        &params_2p,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type2p java");
    assert!(
        Type2PVerifier::verify(
            &stmt_2p,
            &proof_2p_j,
            &params_2p,
            TranscriptMode::JavaCompat
        )
        .expect("verify type2p java")
    );
    assert_cross_mode_rejected(Type2PVerifier::verify(
        &stmt_2p,
        &proof_2p_j,
        &params_2p,
        TranscriptMode::Canonical,
    ));

    let params_3 = sample_params(4);
    let stmt_3 = Type3Statement {
        nbits: 16,
        u: 4,
        v: 4,
        b: 8,
        tt: 2,
        aggregated: true,
    };
    let wit_3 = Type3Witness {
        values: vec![BigUint::from(131u32), BigUint::from(75u32)],
    };
    let proof_3_c = Type3Prover::prove(
        &stmt_3,
        &wit_3,
        &params_3,
        TranscriptMode::Canonical,
        &mut rng,
    )
    .expect("prove type3 canonical");
    assert!(
        Type3Verifier::verify(&stmt_3, &proof_3_c, &params_3, TranscriptMode::Canonical)
            .expect("verify type3 canonical")
    );
    assert_cross_mode_rejected(Type3Verifier::verify(
        &stmt_3,
        &proof_3_c,
        &params_3,
        TranscriptMode::JavaCompat,
    ));

    let proof_3_j = Type3Prover::prove(
        &stmt_3,
        &wit_3,
        &params_3,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type3 java");
    assert!(
        Type3Verifier::verify(&stmt_3, &proof_3_j, &params_3, TranscriptMode::JavaCompat)
            .expect("verify type3 java")
    );
    assert_cross_mode_rejected(Type3Verifier::verify(
        &stmt_3,
        &proof_3_j,
        &params_3,
        TranscriptMode::Canonical,
    ));

    let params_4 = sample_params(32);
    let stmt_4 = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let wit_4 = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let proof_4_c = Type4BatchProver::prove(
        &stmt_4,
        &wit_4,
        &params_4,
        TranscriptMode::Canonical,
        &mut rng,
    )
    .expect("prove type4 canonical");
    assert!(
        Type4BatchVerifier::verify(&stmt_4, &proof_4_c, &params_4, TranscriptMode::Canonical)
            .expect("verify type4 canonical")
    );
    assert_cross_mode_rejected(Type4BatchVerifier::verify(
        &stmt_4,
        &proof_4_c,
        &params_4,
        TranscriptMode::JavaCompat,
    ));

    let proof_4_j = Type4BatchProver::prove(
        &stmt_4,
        &wit_4,
        &params_4,
        TranscriptMode::JavaCompat,
        &mut rng,
    )
    .expect("prove type4 java");
    assert!(
        Type4BatchVerifier::verify(&stmt_4, &proof_4_j, &params_4, TranscriptMode::JavaCompat)
            .expect("verify type4 java")
    );
    assert_cross_mode_rejected(Type4BatchVerifier::verify(
        &stmt_4,
        &proof_4_j,
        &params_4,
        TranscriptMode::Canonical,
    ));
}
