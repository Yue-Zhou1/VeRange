use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type1::{Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use verange_proof::type2p::{Type2PProver, Type2PStatement, Type2PVerifier, Type2PWitness};
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};
use verange_proof::type4_batch::{
    Type4BatchProver, Type4BatchStatement, Type4BatchVerifier, Type4BatchWitness,
};

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

#[test]
fn property_tests_single_field_mutation_fails() {
    let mut rng = ChaCha20Rng::from_seed([201u8; 32]);

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
    let mut proof_1 = Type1Prover::prove(&stmt_1, &wit_1, &params_1, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove type1");
    proof_1.eta1 += Fr::from(1u64);
    assert!(!Type1Verifier::verify(&stmt_1, &proof_1, &params_1, TranscriptMode::JavaCompat)
        .expect("verify type1"));

    let params_2 = sample_params(4);
    let stmt_2 = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let wit_2 = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };
    let mut proof_2 = Type2PProver::prove(&stmt_2, &wit_2, &params_2, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove type2p");
    proof_2.inner.eta1 += Fr::from(1u64);
    assert!(!Type2PVerifier::verify(&stmt_2, &proof_2, &params_2, TranscriptMode::JavaCompat)
        .expect("verify type2p"));

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
    let mut proof_3 = Type3Prover::prove(&stmt_3, &wit_3, &params_3, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove type3");
    proof_3.inner.eta2 += Fr::from(1u64);
    assert!(!Type3Verifier::verify(&stmt_3, &proof_3, &params_3, TranscriptMode::JavaCompat)
        .expect("verify type3"));

    let params_4 = sample_params(4);
    let stmt_4 = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 4,
        b: 8,
    };
    let wit_4 = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };
    let mut proof_4 = Type4BatchProver::prove(&stmt_4, &wit_4, &params_4, TranscriptMode::JavaCompat, &mut rng)
        .expect("prove type4_batch");
    proof_4.inner.eta2 += Fr::from(1u64);
    assert!(
        !Type4BatchVerifier::verify(&stmt_4, &proof_4, &params_4, TranscriptMode::JavaCompat)
            .expect("verify type4_batch")
    );
}

#[test]
fn property_tests_type1_witness_boundaries() {
    let params = sample_params(4);
    let statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };

    for value in [BigUint::from(0u32), BigUint::from(255u32)] {
        let witness = Type1Witness {
            values: vec![value],
        };
        let mut rng = ChaCha20Rng::from_seed([211u8; 32]);
        let proof = Type1Prover::prove(
            &statement,
            &witness,
            &params,
            TranscriptMode::JavaCompat,
            &mut rng,
        )
        .expect("prove type1 boundary");
        assert!(
            Type1Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                .expect("verify type1 boundary")
        );
    }
}
