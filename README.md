# VeRange
This repository contains the source code of the paper `VeRange: Verification-efficient Zero-knowledge Range Arguments
with Transparent Setup for Blockchain Applications and More` at [eprint](https://eprint.iacr.org/2025/528).

`Note that the code is for research only`.

There are two main branches:
* Java Implementation: A maven project of range arguments and polynomial commitment arguments. The implementation of all range arguments are in the Branch `java/src/main/java/zkp`
  * `BIC`: Implement the Bounded Integer Commitments of range arguments of the [BIC21](https://eprint.iacr.org/2021/540).
  * `BPPP`: Implement the Bulletproofs++ optimized version.
  * `PolynomialCommitment`: Implement the polynomial commitments adopted from [BCC+16](https://eprint.iacr.org/2016/263)  .
  * `VeRange-Type1`: Implement the Type-1 range arguments.
  * `VeRange-Type2`: Implement the Type-2 range arguments.
  * `VeRange-Type2B`: Implement the Type-2B range arguments.
  * `VeRange-Type3`: Implement the Type-3 range arguments.
  * `VeRange-Type4`: Implement the Type-4 range arguments.
* Solidity Implementation: A Truffle project of range arguments. The implementation of all range arguments are in the Branch `solidity/contracts`, with the same file names as mentioned above.

To run the tests
------------------------
* VeRangeJava
  * Navigate to Branch `java/src/test/java/ZKP`.

* VeRangeSolidity
  * Navigate to Branch `solidity/test`.

**Key points to know:**
------------------------
1. **Branches, not folders**: The main development work happens on other Git branches. The `java` branch houses a Maven-based Java implementation under `java/src/main/java/zkp`, and the `solidity` branch holds Solidity smart-contract code under `solidity/contracts`.
2. **Multiple range-argument variants**: On these branches you’ll find different variants such as BIC, Bulletproofs++, and various “VeRange-Type*” implementations.
3. **Testing locations**: Java tests are under `java/src/test/java/ZKP`, while Solidity tests live in `solidity/test`.
4. **Research-only**: The `README` emphasizes that the code is provided for research purposes and references the associated paper [eprint](https://eprint.iacr.org/2025/528).

The `main` branch now includes the Rust workspace refactor (`crates/`) in addition to cross-branch Java and Solidity references.
If your goal is to build or understand the code:
- **Explore each branch**. Check out `java` or `solidity` to inspect the source directories mentioned in the README.
- **Read the associated paper** to understand the cryptographic background and how the range arguments work.
- **Use Cargo for Rust**. The Rust code is organized as a multi-crate workspace and tested via `cargo test`.

## Rust SDK

This branch now includes a Rust SDK and proof implementation under the workspace crates:

- `crates/verange-core`
- `crates/verange-poly-commit`
- `crates/verange-proof`
- `crates/verange-sdk`

Implemented proof systems in Rust include:

- `type1`
- `type2`
- `type2p`
- `type3`
- `type4_batch`

See `README-rust-sdk.md` for SDK usage, Type2P binary-format details, and Type4 parameter requirements.

Useful commands:

- Run all Rust tests: `cargo test --workspace`
- Run proof crate tests only: `cargo test -p verange-proof`
- Run proof-mode separation tests: `cargo test -p verange-proof --test mode_tests`
- Run proof fuzz-like robustness tests: `cargo test -p verange-proof --test fuzz_like_tests`
- Run proof crate clippy gate: `cargo clippy -p verange-proof --all-targets --no-deps -- -D warnings -W clippy::all -A clippy::needless_range_loop -A clippy::too_many_arguments`
- Run SDK API tests only: `cargo test -p verange-sdk sdk_api_tests`
- Run SDK all-types example: `cargo run -p verange-sdk --example sdk_all_types`
- Run SDK proof-metrics logger: `cargo run -p verange-sdk --example proof_metrics -- logs/proof_metrics.log`
