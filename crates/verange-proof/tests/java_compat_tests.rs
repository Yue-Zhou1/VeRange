use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use verange_core::transcript::TranscriptMode;
use verange_core::PedersenParams;
use verange_proof::type1::{Type1Prover, Type1Statement, Type1Verifier, Type1Witness};
use verange_proof::type2p::{Type2PProver, Type2PStatement, Type2PVerifier, Type2PWitness};
use verange_proof::type3::{Type3Prover, Type3Statement, Type3Verifier, Type3Witness};
use verange_proof::type4_batch::{
    Type4BatchProver, Type4BatchStatement, Type4BatchVerifier, Type4BatchWitness,
};

#[derive(Debug, Deserialize)]
struct CompatVector {
    scheme: String,
    nbits: usize,
    #[serde(default)]
    k: usize,
    #[serde(default)]
    l: usize,
    #[serde(default)]
    u: usize,
    #[serde(default)]
    v: usize,
    #[serde(default)]
    b: usize,
    #[serde(default)]
    tt: usize,
    #[serde(default)]
    aggregated: bool,
    #[serde(default)]
    values: Vec<String>,
    #[serde(default)]
    value: String,
}

fn sample_params(l: usize) -> PedersenParams {
    let g = G1Projective::generator();
    let h = g * Fr::from(7u64);
    let gs = (0..l)
        .map(|i| g * Fr::from((i + 2) as u64))
        .collect::<Vec<_>>();
    PedersenParams::new(g, h, gs).expect("params")
}

fn parse_biguint(s: &str) -> BigUint {
    BigUint::parse_bytes(s.as_bytes(), 10).expect("biguint")
}

#[test]
fn java_compat_tests_fixture_vectors_verify() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/java_compat_vectors.json");
    let content = fs::read_to_string(path).expect("fixture file");
    let vectors: Vec<CompatVector> = serde_json::from_str(&content).expect("valid json");

    for (i, v) in vectors.iter().enumerate() {
        let mut seed = [0u8; 32];
        seed[0] = i as u8 + 1;
        let mut rng = ChaCha20Rng::from_seed(seed);

        match v.scheme.as_str() {
            "type1" => {
                let params = sample_params(v.l);
                let statement = Type1Statement {
                    nbits: v.nbits,
                    k: v.k,
                    tt: v.tt,
                    aggregated: v.aggregated,
                };
                let witness = Type1Witness {
                    values: v.values.iter().map(|x| parse_biguint(x)).collect(),
                };
                let proof = Type1Prover::prove(
                    &statement,
                    &witness,
                    &params,
                    TranscriptMode::JavaCompat,
                    &mut rng,
                )
                .expect("prove type1");
                assert!(Type1Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                    .expect("verify type1"));
            }
            "type2p" => {
                let params = sample_params(v.l);
                let statement = Type2PStatement {
                    nbits: v.nbits,
                    k: v.k,
                    l: v.l,
                    b: v.b,
                    tt: v.tt,
                    aggregated: v.aggregated,
                };
                let witness = Type2PWitness {
                    values: v.values.iter().map(|x| parse_biguint(x)).collect(),
                };
                let proof = Type2PProver::prove(
                    &statement,
                    &witness,
                    &params,
                    TranscriptMode::JavaCompat,
                    &mut rng,
                )
                .expect("prove type2p");
                assert!(Type2PVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                    .expect("verify type2p"));
            }
            "type3" => {
                let params = sample_params(v.u);
                let statement = Type3Statement {
                    nbits: v.nbits,
                    u: v.u,
                    v: v.v,
                    b: v.b,
                    tt: v.tt,
                    aggregated: v.aggregated,
                };
                let witness = Type3Witness {
                    values: v.values.iter().map(|x| parse_biguint(x)).collect(),
                };
                let proof = Type3Prover::prove(
                    &statement,
                    &witness,
                    &params,
                    TranscriptMode::JavaCompat,
                    &mut rng,
                )
                .expect("prove type3");
                assert!(Type3Verifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                    .expect("verify type3"));
            }
            "type4_batch" => {
                let params = sample_params(v.l);
                let statement = Type4BatchStatement {
                    nbits: v.nbits,
                    k: v.k,
                    l: v.l,
                    b: v.b,
                };
                let witness = Type4BatchWitness {
                    value: parse_biguint(&v.value),
                };
                let proof = Type4BatchProver::prove(
                    &statement,
                    &witness,
                    &params,
                    TranscriptMode::JavaCompat,
                    &mut rng,
                )
                .expect("prove type4_batch");
                assert!(
                    Type4BatchVerifier::verify(&statement, &proof, &params, TranscriptMode::JavaCompat)
                        .expect("verify type4_batch")
                );
            }
            _ => panic!("unsupported scheme in fixture"),
        }
    }
}
