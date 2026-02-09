# VeRange Rust SDK Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a clean Rust SDK for VeRange so external users can run prover/verifier APIs for Type1/Type2/Type2P/Type3/Type4_batch proofs, with Java-compatible behavior and test vectors.

**Architecture:** Use a Rust workspace with `core` primitives, `poly-commit` layer, `proof` modules per VeRange type, and a stable top-level SDK API. Keep protocol math independent from IO/serialization, and gate Java-compatibility in explicit config profiles.

**Tech Stack:** Rust stable, `ark-bn254`, `ark-ec`, `ark-ff`, `ark-serialize`, `sha3`/`tiny-keccak`, `rand_core` + `rand_chacha`, `serde`, `thiserror`, `zeroize`, `proptest`, `criterion`.

## Scope Notes From Java Branch

- Core primitives are Pedersen-style commitments over BN128 (`src/main/java/config/BouncyKey.java`, `src/main/java/commitment/*`).
- Fiat-Shamir uses Keccak256 and custom 32-byte fixed encoding (`src/main/java/utils/HashUtils.java`).
- Main proof variants live in `src/main/java/zkp/Type1.java`, `src/main/java/zkp/Type2.java`, `src/main/java/zkp/Type2P.java`, `src/main/java/zkp/Type3.java`, `src/main/java/zkp/Type4_batch.java`.
- Polynomial commitment system is custom (not KZG) and used by Type3/Type4 (`src/main/java/zkp/PolynomialCommitment.java`, `src/main/java/zkp/Polynomial.java`).
- `Type4.verify_batch()` is currently stubbed to always `true` (`src/main/java/zkp/Type4.java`), so Rust should prioritize `Type4_batch` verification path first.

### Task 1: Create Rust Workspace Skeleton

**Files:**
- Create: `Cargo.toml`
- Create: `crates/verange-core/Cargo.toml`
- Create: `crates/verange-poly-commit/Cargo.toml`
- Create: `crates/verange-proof/Cargo.toml`
- Create: `crates/verange-sdk/Cargo.toml`
- Create: `crates/verange-sdk/src/lib.rs`
- Create: `.github/workflows/ci.yml`

**Step 1: Write failing workspace smoke test**

Create `crates/verange-sdk/tests/workspace_smoke.rs` with one import test for `verange_sdk`.

**Step 2: Run test to verify it fails**

Run: `cargo test -p verange-sdk workspace_smoke -q`
Expected: FAIL with unresolved crate/module before scaffolding is complete.

**Step 3: Add workspace and crates**

Define workspace members and minimal lib stubs for each crate.

**Step 4: Run test to verify it passes**

Run: `cargo test -p verange-sdk workspace_smoke -q`
Expected: PASS.

**Step 5: Commit**

Run:
```bash
git add Cargo.toml crates .github/workflows/ci.yml
git commit -m "chore: scaffold verange rust workspace"
```

### Task 2: Implement Curve, Scalar, and Commitment Core

**Files:**
- Create: `crates/verange-core/src/lib.rs`
- Create: `crates/verange-core/src/curve.rs`
- Create: `crates/verange-core/src/scalar.rs`
- Create: `crates/verange-core/src/commitment.rs`
- Create: `crates/verange-core/src/params.rs`
- Test: `crates/verange-core/tests/commitment_tests.rs`

**Step 1: Write failing tests**

Add tests for:
- `commit(m, r) == g*m + h*r`
- additive homomorphism
- identity and generator-basis handling (`gs` list length checks)

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-core commitment_tests -q`
Expected: FAIL with missing types/functions.

**Step 3: Implement minimal commitment primitives**

Add:
- `PedersenParams { g, h, gs, order }`
- `Commitment` wrapper for BN254 G1 points
- `commit`, `mul_g`, `mul_h`, `sum`, `mul_scalar`

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-core commitment_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-core
git commit -m "feat(core): add bn254 commitment primitives"
```

### Task 3: Implement Java-Compatible Transcript Hashing

**Files:**
- Create: `crates/verange-core/src/transcript.rs`
- Create: `crates/verange-core/tests/transcript_tests.rs`
- Create: `fixtures/java_hash_vectors.json`

**Step 1: Write failing transcript vector tests**

Add tests for:
- fixed-32-byte big-endian integer encoding
- commitment coordinate concatenation
- Keccak256 challenge derivation compatibility

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-core transcript_tests -q`
Expected: FAIL due to missing transcript.

**Step 3: Implement transcript**

Add:
- domain-separated challenge labels
- `append_scalar`, `append_point`, `challenge_scalar`
- explicit `JavaCompat` mode for byte-level compatibility

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-core transcript_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-core fixtures/java_hash_vectors.json
git commit -m "feat(core): add keccak transcript with java-compat encoding"
```

### Task 4: Port Vector and Arithmetic Utilities

**Files:**
- Create: `crates/verange-core/src/vector.rs`
- Create: `crates/verange-core/src/arith.rs`
- Test: `crates/verange-core/tests/vector_tests.rs`

**Step 1: Write failing vector tests**

Cover:
- elementwise add/sub/mul
- inner product
- power vector generation
- n-ary decomposition for witness integers

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-core vector_tests -q`
Expected: FAIL.

**Step 3: Implement vector/math utilities**

Port logic from Java `VectorB`, `VectorP`, and decomposition helpers with strict bounds and no hidden modulo mistakes.

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-core vector_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-core/src/vector.rs crates/verange-core/src/arith.rs crates/verange-core/tests/vector_tests.rs
git commit -m "feat(core): add vector operations and decomposition helpers"
```

### Task 5: Port Polynomial and Polynomial Commitment Layer

**Files:**
- Create: `crates/verange-poly-commit/src/lib.rs`
- Create: `crates/verange-poly-commit/src/polynomial.rs`
- Create: `crates/verange-poly-commit/src/ntt.rs`
- Create: `crates/verange-poly-commit/src/commit.rs`
- Test: `crates/verange-poly-commit/tests/polynomial_commit_tests.rs`

**Step 1: Write failing tests**

Add tests for:
- polynomial arithmetic (`plus/minus/times/divide/eval`)
- vanishing polynomial
- `poly_commit`, `poly_eval`, `poly_verify`
- batch verify relation

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-poly-commit polynomial_commit_tests -q`
Expected: FAIL.

**Step 3: Implement commitment scheme**

Port `setupHijs`, `PolyCommit1`, `PolyEval1`, `PolyVerify`, and `PolyVerifyBatch` with explicit dimension checks and typed errors.

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-poly-commit polynomial_commit_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-poly-commit
git commit -m "feat(poly-commit): port polynomial commitment and verification"
```

### Task 6: Implement Type1 Proof (First End-to-End SDK Proof)

**Files:**
- Create: `crates/verange-proof/src/type1.rs`
- Modify: `crates/verange-proof/src/lib.rs`
- Create: `crates/verange-proof/tests/type1_tests.rs`
- Create: `crates/verange-proof/tests/fixtures/type1_java_vectors.json`

**Step 1: Write failing prove/verify test**

Add:
- valid proof passes
- tampered proof fails
- aggregation (`TT > 1`) case

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-proof type1_tests -q`
Expected: FAIL.

**Step 3: Implement Type1 prover/verifier**

Mirror equations and challenge sequence from Java Type1 with clean structs:
- `Type1Statement`
- `Type1Witness`
- `Type1Proof`
- `Type1Prover::prove`
- `Type1Verifier::verify`

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-proof type1_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-proof/src/type1.rs crates/verange-proof/tests/type1_tests.rs crates/verange-proof/tests/fixtures/type1_java_vectors.json crates/verange-proof/src/lib.rs
git commit -m "feat(proof): implement verange type1 proof"
```

### Task 7: Implement Type2 and Type2P Proofs

**Files:**
- Create: `crates/verange-proof/src/type2.rs`
- Create: `crates/verange-proof/src/type2p.rs`
- Modify: `crates/verange-proof/src/lib.rs`
- Create: `crates/verange-proof/tests/type2_tests.rs`
- Create: `crates/verange-proof/tests/type2p_tests.rs`

**Step 1: Write failing tests for Type2 and Type2P**

Include:
- single proof and aggregated proof paths
- K=3 / K=4 branches for Type2P combinational commitments

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-proof type2_tests type2p_tests -q`
Expected: FAIL.

**Step 3: Implement Type2 and Type2P**

Port prover/verifier equations with typed matrix builders and branch-safe combinator logic.

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-proof type2_tests type2p_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-proof/src/type2.rs crates/verange-proof/src/type2p.rs crates/verange-proof/tests/type2_tests.rs crates/verange-proof/tests/type2p_tests.rs crates/verange-proof/src/lib.rs
git commit -m "feat(proof): implement verange type2 and type2p proofs"
```

### Task 8: Implement Type3 Proof with Polynomial Commitment Integration

**Files:**
- Create: `crates/verange-proof/src/type3.rs`
- Modify: `crates/verange-proof/src/lib.rs`
- Create: `crates/verange-proof/tests/type3_tests.rs`

**Step 1: Write failing Type3 tests**

Include valid, tamper, and aggregated (`TT`) cases.

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-proof type3_tests -q`
Expected: FAIL.

**Step 3: Implement Type3**

Port:
- polynomial commitments `cmS`/`cmB`
- evaluation proofs `pi_Sx`/`pi_Bx`
- verify equations 1-4

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-proof type3_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-proof/src/type3.rs crates/verange-proof/tests/type3_tests.rs crates/verange-proof/src/lib.rs
git commit -m "feat(proof): implement verange type3 proof"
```

### Task 9: Implement Type4_batch Proof and Verification

**Files:**
- Create: `crates/verange-proof/src/type4_batch.rs`
- Modify: `crates/verange-proof/src/lib.rs`
- Create: `crates/verange-proof/tests/type4_batch_tests.rs`

**Step 1: Write failing Type4_batch tests**

Include:
- valid proof
- wrong `qv` relation fails
- wrong evaluation tuple fails

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-proof type4_batch_tests -q`
Expected: FAIL.

**Step 3: Implement Type4_batch**

Port full equation checks (`b1..b8`) from Java Type4_batch verifier and ensure no stubbed verification methods.

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-proof type4_batch_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-proof/src/type4_batch.rs crates/verange-proof/tests/type4_batch_tests.rs crates/verange-proof/src/lib.rs
git commit -m "feat(proof): implement verange type4 batch proof"
```

### Task 10: Build Public SDK API (Prover/Verifier Ergonomics)

**Files:**
- Create: `crates/verange-sdk/src/error.rs`
- Create: `crates/verange-sdk/src/params.rs`
- Create: `crates/verange-sdk/src/prover.rs`
- Create: `crates/verange-sdk/src/verifier.rs`
- Modify: `crates/verange-sdk/src/lib.rs`
- Create: `crates/verange-sdk/tests/sdk_api_tests.rs`

**Step 1: Write failing SDK API tests**

Define desired API:
- `Parameters::bn254_java_compat()`
- `prove_type2p(...)`
- `verify_type2p(...)`
- serialized proof round-trip

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-sdk sdk_api_tests -q`
Expected: FAIL.

**Step 3: Implement SDK façade**

Expose clean API that hides low-level polynomial/commitment internals and returns typed errors.

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-sdk sdk_api_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-sdk
git commit -m "feat(sdk): expose stable prover and verifier api"
```

### Task 11: Add Cross-Language Compatibility and Property Tests

**Files:**
- Create: `crates/verange-proof/tests/java_compat_tests.rs`
- Create: `crates/verange-proof/tests/property_tests.rs`
- Create: `scripts/export_java_vectors.sh`

**Step 1: Write failing compatibility tests**

Check Rust verifier against vectors generated by Java branch for Type1/Type2P/Type3/Type4_batch.

**Step 2: Run tests to verify fail**

Run: `cargo test -p verange-proof java_compat_tests property_tests -q`
Expected: FAIL until vector parser + proofs are wired.

**Step 3: Implement vector import + property tests**

Add:
- JSON fixtures loader
- corruption tests (single-field mutation must fail verify)
- witness range boundary tests

**Step 4: Run tests to verify pass**

Run: `cargo test -p verange-proof java_compat_tests property_tests -q`
Expected: PASS.

**Step 5: Commit**

```bash
git add crates/verange-proof/tests scripts/export_java_vectors.sh
git commit -m "test: add java compatibility and property tests"
```

### Task 12: Documentation, Examples, and Release Hygiene

**Files:**
- Create: `README-rust-sdk.md`
- Create: `examples/type2p_basic.rs`
- Create: `examples/type3_aggregated.rs`
- Create: `docs/security-notes.md`
- Modify: `README.md`

**Step 1: Write failing doc tests**

Add runnable examples that compile with `cargo test --doc`.

**Step 2: Run doc tests to verify fail**

Run: `cargo test --doc -p verange-sdk`
Expected: FAIL before examples compile.

**Step 3: Write docs and examples**

Document:
- parameter selection
- proving and verifying
- serialization format
- compatibility mode and security caveats

**Step 4: Run full verification suite**

Run:
```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```
Expected: all PASS.

**Step 5: Commit**

```bash
git add README.md README-rust-sdk.md examples docs
git commit -m "docs: add sdk usage guide examples and security notes"
```

## Non-Negotiable Quality Gates

- No global mutable protocol state in library paths.
- No `println!` in prover/verifier core.
- Canonical proof serialization only (`ark-serialize` + optional serde wrappers).
- `#[forbid(unsafe_code)]` in SDK and proof crates.
- Constant-time scalar/field handling via underlying curve libraries.
- Every proof module must include:
  - valid proof test
  - malformed proof test
  - transcript replay determinism test
  - statement/proof mismatch test

## Suggested Delivery Order

1. Type1
2. Type2P
3. Type3
4. Type4_batch
5. Type2
6. Optional: BIC and BPPP compatibility modules

