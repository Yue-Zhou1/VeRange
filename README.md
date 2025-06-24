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

Because this `main` branch only includes documentation, you’ll need to switch to the respective branches to see the actual implementation. If your goal is to build or understand the code:
- **Explore each branch**. Check out `java` or `solidity` to inspect the source directories mentioned in the README.
- **Read the associated paper** to understand the cryptographic background and how the range arguments work.
- **Learn the build tools**: The Java code uses Maven, while the Solidity code relies on Truffle; familiarity with these toolchains will help you run tests and examples.
