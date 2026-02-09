use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_sdk::{
    deserialize_type2p_proof, serialize_type2p_proof, Parameters, Prover, Type1Statement,
    Type1Witness, Type2PStatement, Type2PWitness, Type2Statement, Type2Witness, Type3Statement,
    Type3Witness, Type4BatchStatement, Type4BatchWitness, Verifier,
};

#[test]
fn sdk_api_tests_type2p_end_to_end_and_roundtrip() {
    let params = Parameters::bn254_java_compat(4).expect("params");
    let prover = Prover::new(params.clone(), TranscriptMode::JavaCompat);
    let verifier = Verifier::new(params, TranscriptMode::JavaCompat);

    let statement = Type2PStatement {
        nbits: 12,
        k: 3,
        l: 4,
        b: 8,
        tt: 1,
        aggregated: false,
    };
    let witness = Type2PWitness {
        values: vec![BigUint::from(987u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([101u8; 32]);
    let proof = prover
        .prove_type2p(&statement, &witness, &mut rng)
        .expect("prove");

    assert!(verifier.verify_type2p(&statement, &proof).expect("verify"));

    let encoded = serialize_type2p_proof(&proof);
    let decoded = deserialize_type2p_proof(&encoded).expect("decode");

    assert_eq!(proof, decoded);
    assert!(verifier
        .verify_type2p(&statement, &decoded)
        .expect("verify decoded"));
}

#[test]
fn sdk_api_tests_type1_end_to_end() {
    let params = Parameters::bn254_java_compat(4).expect("params");
    let prover = Prover::new(params.clone(), TranscriptMode::JavaCompat);
    let verifier = Verifier::new(params, TranscriptMode::JavaCompat);

    let statement = Type1Statement {
        nbits: 8,
        k: 2,
        tt: 1,
        aggregated: false,
    };
    let witness = Type1Witness {
        values: vec![BigUint::from(173u32)],
    };

    let mut rng = ChaCha20Rng::from_seed([111u8; 32]);
    let proof = prover
        .prove_type1(&statement, &witness, &mut rng)
        .expect("prove type1");
    assert!(verifier
        .verify_type1(&statement, &proof)
        .expect("verify type1"));
}

#[test]
fn sdk_api_tests_type2_end_to_end() {
    let params = Parameters::bn254_java_compat(4).expect("params");
    let prover = Prover::new(params.clone(), TranscriptMode::JavaCompat);
    let verifier = Verifier::new(params, TranscriptMode::JavaCompat);

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

    let mut rng = ChaCha20Rng::from_seed([121u8; 32]);
    let proof = prover
        .prove_type2(&statement, &witness, &mut rng)
        .expect("prove type2");
    assert!(verifier
        .verify_type2(&statement, &proof)
        .expect("verify type2"));
}

#[test]
fn sdk_api_tests_type3_end_to_end() {
    let params = Parameters::bn254_java_compat(4).expect("params");
    let prover = Prover::new(params.clone(), TranscriptMode::JavaCompat);
    let verifier = Verifier::new(params, TranscriptMode::JavaCompat);

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

    let mut rng = ChaCha20Rng::from_seed([131u8; 32]);
    let proof = prover
        .prove_type3(&statement, &witness, &mut rng)
        .expect("prove type3");
    assert!(verifier
        .verify_type3(&statement, &proof)
        .expect("verify type3"));
}

#[test]
fn sdk_api_tests_type4_batch_end_to_end() {
    let params = Parameters::bn254_java_compat(32).expect("params");
    let prover = Prover::new(params.clone(), TranscriptMode::JavaCompat);
    let verifier = Verifier::new(params, TranscriptMode::JavaCompat);

    let statement = Type4BatchStatement {
        nbits: 16,
        k: 4,
        l: 32,
        b: 8,
    };
    let witness = Type4BatchWitness {
        value: BigUint::from(1337u32),
    };

    let mut rng = ChaCha20Rng::from_seed([141u8; 32]);
    let proof = prover
        .prove_type4_batch(&statement, &witness, &mut rng)
        .expect("prove type4_batch");
    assert!(verifier
        .verify_type4_batch(&statement, &proof)
        .expect("verify type4_batch"));
}
