# Security Notes

## Scope

The Rust implementation is currently a research-focused migration from Java and is still under active development.

## Current Caveats

- APIs and proof formats are not yet final.
- Compatibility fixtures currently validate deterministic behavior in Rust; direct Java-generated vectors are being integrated incrementally.
- Some higher-level proof modules currently reuse shared internals to keep interfaces stable while parity work continues.

## Recommended Usage

- Do not deploy this SDK in production yet.
- Pin exact commits when benchmarking.
- Treat all outputs as research artifacts unless independently audited.

## Validation Guidance

- Run `cargo test --workspace` before any benchmark/reporting run.
- Keep fixture updates deterministic and reviewable in Git.
- Validate corrupted-proof negative tests for every added proof module.
