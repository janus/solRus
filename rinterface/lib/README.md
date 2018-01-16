
Purpose:
---
Proof Of Concept that Rust signed message can be verfied by Solidity  smart contract.

Requirements:
----
1. Truffle 3.2.1 or greater globally installed
2. EthereumJS TestRPC v4.0.1  or greater
3. Node 4 or greater
4. npm 3 or greater
5. rustc 1.22.1 or greater

Setup:
----
1. testrpc (on a different console)
2. Clone repo
3. cd into lib
4. cargo test -- --nocapture
5. cd into smartContract
6. npm install
7. truffle compile
8. truffle migrate (or truffle migrate --reset)
9. truffle test


NOTE:
---
secp256k1 and num256 crates are in a special folder. You would need to clone via git.
For more information on secp256k1 , https://github.com/apoelstra/rust-secp256k1/tree/master/depend/secp256k1
https://github.com/apoelstra/rust-secp256k1
For num256 crate , https://github.com/althea-mesh/althea_rs

