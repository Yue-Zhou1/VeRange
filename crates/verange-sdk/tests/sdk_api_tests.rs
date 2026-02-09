use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_sdk::{
    deserialize_type2p_proof, serialize_type2p_proof, Parameters, Prover, Type2PStatement,
    Type2PWitness, Verifier,
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
