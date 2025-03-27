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
