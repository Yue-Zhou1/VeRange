use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use verange_core::transcript::TranscriptMode;
use verange_sdk::metrics::run_default_proof_metrics;

fn temp_log_path() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("verange-proof-metrics-{stamp}.log"))
}

#[test]
fn metrics_tests_writes_log_for_all_proof_types() {
    let mut rng = ChaCha20Rng::from_seed([29u8; 32]);
    let path = temp_log_path();

    let records = run_default_proof_metrics(&path, TranscriptMode::JavaCompat, &mut rng)
        .expect("metrics collection should succeed");
    assert_eq!(records.len(), 5, "expected type1/type2/type2p/type3/type4_batch");

    let content = fs::read_to_string(&path).expect("log file must be readable");
    assert!(content.contains("proof_type=type1"));
    assert!(content.contains("proof_type=type2"));
    assert!(content.contains("proof_type=type2p"));
    assert!(content.contains("proof_type=type3"));
    assert!(content.contains("proof_type=type4_batch"));
    assert!(content.contains("proof_size_bytes="));
    assert!(content.contains("prove_time_us="));
    assert!(content.contains("verify_time_us="));
    assert!(content.contains("statement="));

    let _ = fs::remove_file(path);
}
