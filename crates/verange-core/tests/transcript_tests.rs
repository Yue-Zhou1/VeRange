use ark_bn254::{Fr, G1Projective};
use ark_ec::Group;
use serde::Deserialize;
use std::fs;
use verange_core::transcript::{java_encode_point, java_encode_scalar, Transcript, TranscriptMode};

#[derive(Debug, Deserialize)]
struct HashVector {
    label: String,
    scalars: Vec<u64>,
    expected_hex: String,
}

#[test]
fn transcript_tests_scalar_encoding_is_fixed_32bytes() {
    let encoded = java_encode_scalar(&Fr::from(1u64));
    assert_eq!(encoded.len(), 32);
    assert_eq!(encoded[31], 1);
    assert!(encoded[..31].iter().all(|b| *b == 0));
}

#[test]
fn transcript_tests_point_encoding_is_two_field_elements() {
    let point = G1Projective::generator();
    let encoded = java_encode_point(&point);

    assert_eq!(encoded.len(), 64);
}

#[test]
fn transcript_tests_java_compat_challenge_matches_fixture() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/java_hash_vectors.json");
    let content = fs::read_to_string(path).expect("fixture file");
    let vectors: Vec<HashVector> = serde_json::from_str(&content).expect("valid json");

    for vector in vectors {
        let mut transcript = Transcript::new(vector.label.as_bytes(), TranscriptMode::JavaCompat);
        for scalar in vector.scalars {
            transcript.append_scalar(b"s", &Fr::from(scalar));
        }

        let challenge = transcript.challenge_scalar(b"challenge");
        let got = hex::encode(java_encode_scalar(&challenge));
        assert_eq!(got, vector.expected_hex);
    }
}

#[test]
fn transcript_tests_canonical_mode_is_domain_separated() {
    let mut canonical = Transcript::new(b"type1", TranscriptMode::Canonical);
    let mut java_compat = Transcript::new(b"type1", TranscriptMode::JavaCompat);

    for scalar in [1u64, 2u64, 3u64, 5u64] {
        let value = Fr::from(scalar);
        canonical.append_scalar(b"s", &value);
        java_compat.append_scalar(b"s", &value);
    }

    let canonical_challenge = canonical.challenge_scalar(b"challenge");
    let java_compat_challenge = java_compat.challenge_scalar(b"challenge");
    assert_ne!(
        canonical_challenge, java_compat_challenge,
        "canonical mode should be challenge-domain-separated from java compat"
    );
}
