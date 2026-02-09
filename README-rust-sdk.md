# VeRange Rust SDK (WIP)

This repository now includes an incremental Rust SDK under `crates/`.

## Crates

- `crates/verange-core`: curve/commitment/transcript/vector primitives.
- `crates/verange-poly-commit`: polynomial and custom polynomial-commitment layer.
- `crates/verange-proof`: proof modules (`type1`, `type2`, `type2p`, `type3`, `type4_batch`).
- `crates/verange-sdk`: user-facing prover/verifier API for SDK use.

## Quick Start

```rust
use num_bigint::BigUint;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaCha20Rng;
use verange_core::transcript::TranscriptMode;
use verange_sdk::{Parameters, Prover, Type2PStatement, Type2PWitness, Verifier};

let params = Parameters::bn254_java_compat(4)?;
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

let mut rng = ChaCha20Rng::from_seed([1u8; 32]);
let proof = prover.prove_type2p(&statement, &witness, &mut rng)?;
assert!(verifier.verify_type2p(&statement, &proof)?);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Status

The Rust SDK is under active refactor and does not yet fully replicate every Java proof equation path.
