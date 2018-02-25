
Purpose:
---
Proof Of Concept that Rust signed message can be verfied by Solidity  smart contract.

Requirements:
----
1. Truffle 4.0.6 or greater globally installed
2. EthereumJS TestRPC v4.0.1  or greater
3. Node 8 or greater
4. npm 5.6.0 or greater
5. rustc 1.22.1 or greater

Setup:
----
Recommended (just run the script)

sudo ./solrus.sh

However, one can go through the steps one by one.
1. testrpc
2. clone repo
3. cd into lib
4. cargo test -- --nocapture
5. cd into smartContract
6. npm install
7. truffle migrate --reset or npm run migrate
8. truffle test or npm run test



NOTE:
---
ethkey and num256 crates are in a special folder. You would need to clone via git.

For more information on parity , https://github.com/paritytech/parity

ethkey setup:
----
1. cd into lib (inside Rust source code)
2. mkdir repo
3. cd repo
4. git clone https://github.com/paritytech/parity.git
5. cd into parity
6. cargo build -p ethkey-cli --release
7. ./target/release/ethkey --help  [optional]


For num256 crate , https://github.com/althea-mesh/althea_rs



