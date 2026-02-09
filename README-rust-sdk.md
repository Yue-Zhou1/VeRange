# VeRange Rust SDK

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
use verange_sdk::{
    deserialize_type2p_proof, serialize_type2p_proof, Parameters, Prover, Type2PStatement,
    Type2PWitness, Verifier,
};

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

// SDK binary format roundtrip.
let bytes = serialize_type2p_proof(&proof);
let decoded = deserialize_type2p_proof(&bytes)?;
assert_eq!(proof, decoded);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## SDK API Coverage

`verange-sdk` now exposes prover/verifier calls for all currently ported proof systems:

- `Prover::prove_type1` / `Verifier::verify_type1`
- `Prover::prove_type2` / `Verifier::verify_type2`
- `Prover::prove_type2p` / `Verifier::verify_type2p`
- `Prover::prove_type3` / `Verifier::verify_type3`
- `Prover::prove_type4_batch` / `Verifier::verify_type4_batch`

The crate root also re-exports the corresponding statement/witness/proof structs:

- `Type1Statement`, `Type1Witness`, `Type1Proof`
- `Type2Statement`, `Type2Witness`, `Type2Proof`
- `Type2PStatement`, `Type2PWitness`, `Type2PProof`
- `Type3Statement`, `Type3Witness`, `Type3Proof`
- `Type4BatchStatement`, `Type4BatchWitness`, `Type4BatchProof`

See `crates/verange-sdk/examples/sdk_all_types.rs` for a single-file SDK usage walkthrough covering Type1/Type2/Type2P/Type3/Type4_batch.

Run it with:

```bash
cargo run -p verange-sdk --example sdk_all_types
```

## Running SDK Tests

Run all SDK API integration tests:

```bash
cargo test -p verange-sdk sdk_api_tests
```

Run one type-specific test:

```bash
cargo test -p verange-sdk sdk_api_tests_type1_end_to_end
cargo test -p verange-sdk sdk_api_tests_type2_end_to_end
cargo test -p verange-sdk sdk_api_tests_type3_end_to_end
cargo test -p verange-sdk sdk_api_tests_type4_batch_end_to_end
```

## Type2P Binary Format

`serialize_type2p_proof()` and `deserialize_type2p_proof()` use a deterministic, manual byte layout:

- Scalars: 32-byte big-endian field elements.
- Commitments: 64 bytes (`x || y`, each 32-byte big-endian). Identity is 64 zero bytes.
- Vectors: 4-byte big-endian length prefix, then elements.

Field order in the payload:

1. `ys`
2. `big_r`
3. `big_s`
4. `big_u`
5. `cws`
6. `cms`
7. `cfk`
8. `ctk`
9. `ctk_kprime`
10. `eta1`
11. `eta2`
12. `eta3`
13. `eta4`
14. `vs`
15. `us`

Notes:

- This format is for the current native Type2P proof struct, not the older wrapped-Type1 encoding.
- Treat it as an SDK wire format: if you need long-term storage compatibility across versions, add versioning at your application boundary.

## Type4_batch Parameter Requirements

The Type4_batch prover/verifier (`verange_proof::type4_batch`) requires:

- `nbits > 0` and `nbits` is a power of two.
- `b >= 2`.
- `b * nbits` is also a power of two.
- `statement.l == params.gs.len()`.
- `params.gs` must be large enough for internal polynomial-commitment vectors (`qv` and commitment columns).

Practical guidance:

- For `nbits = 16` and `b = 8`, use at least `l = 20`; examples/tests use `l = 32`.
- If verification fails early with statement/proof shape errors, first increase generator basis length.

## Status

Current Rust parity pass includes full prover/verifier equation ports for:

- `type1`
- `type2`
- `type2p`
- `type3`
- `type4_batch`

API ergonomics and compatibility tooling are still being refined.
