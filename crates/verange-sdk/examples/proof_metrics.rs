use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_sdk::metrics::run_default_proof_metrics;

fn parse_mode(arg: Option<&str>) -> TranscriptMode {
    match arg {
        Some("canonical") => TranscriptMode::Canonical,
        _ => TranscriptMode::JavaCompat,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let log_path = args
        .next()
        .unwrap_or_else(|| "logs/proof_metrics.log".to_string());
    let mode = parse_mode(args.next().as_deref());

    let mut rng = ChaCha20Rng::from_seed([77u8; 32]);
    let records = run_default_proof_metrics(&log_path, mode, &mut rng)?;

    println!("wrote {} records to {}", records.len(), log_path);
    for record in &records {
        println!(
            "{} | proof_size_bytes={} prove_time_us={} verify_time_us={} verified={} | params_basis_len={} | {}",
            record.proof_type,
            record.proof_size_bytes,
            record.prove_time.as_micros(),
            record.verify_time.as_micros(),
            record.verified,
            record.params_basis_len,
            record.statement
        );
    }

    Ok(())
}
