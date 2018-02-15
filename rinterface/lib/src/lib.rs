extern crate ethkey;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate multihash;
extern crate num256;
extern crate rustc_hex;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::str;
use multihash::{encode, to_hex, Hash};
use num256::{Int256, PaddedHex, Uint256};
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use ethkey::{sign, KeyPair, Secret};
use ethkey::Message;
use std::str::FromStr;
use std::vec;
use rustc_hex::{FromHex, ToHex};
use jsonrpc_core::*;
use jsonrpc_http_server::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TwoInt256 {
    int1: Int256,
    int2: Int256,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResSigs {
    sig: String,
    address: String,
    num1: String,
    num2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FHSigns {
    sig_0: String,
    sig_1: String,
    address_0: String,
    address_1: String,
    fingerprint: String,
}

fn transport(msg_1: Vec<serde_json::Value>, msg_2: Vec<serde_json::Value>) {
    let mut io = IoHandler::new();
    io.add_method("say_hello", move |_: Params| {
        Ok(Value::Array(msg_1.clone()))
    });

    io.add_method("rtn_fingerprinted", move |_: Params| {
        Ok(Value::Array(msg_2.clone()))
    });

    let _server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Value("altheamesh.com".into()),
            AccessControlAllowOrigin::Null,
        ]))
        .rest_api(RestApi::Secure)
        .start_http(&"127.0.0.1:3030".parse().unwrap());

    match _server {
        Ok(server) => server.wait(),
        Err(_) => {}
    }
}

fn int256_to_padded_hex(fnm256: &Int256, snm256: &Int256) -> String {
    format!("{}{}", fnm256.to_padded_hex(), snm256.to_padded_hex())
}

//Have the rust program serialize two `num256::Int256`'s, hash them,
// sign them, then put them into the solidity which verifies the signature.
pub fn hash_and_sign(
    fnm256: &Int256,
    snm256: &Int256,
    secret: &ethkey::Secret,
    address: &ethkey::Address,
) -> Vec<serde_json::Value> {
    let bytes = int256_to_padded_hex(&fnm256, &snm256).from_hex().unwrap();
    let msg_fhash = encode(Hash::Keccak256, &bytes).unwrap();
    let msg = Message::from_slice(&msg_fhash[2..]);

    let tstr = ResSigs {
        sig: format!("{}", sign(secret, &msg).unwrap()),
        address: format!("{:?}", address),
        num1: serde_json::to_string(&fnm256).unwrap(),
        num2: serde_json::to_string(&snm256).unwrap(),
    };

    let payload = serde_json::to_string(&tstr).unwrap();
    println!("{}", &payload);
    vec![serde_json::Value::String(payload)]
}

pub fn fingerprint_hash(
    channel_id: &str,
    seqnum: &Uint256,
    bal_0: &Uint256,
    bal_1: &Uint256,
    hash_locks: &str,
) -> Vec<u8> {
    let seqnum_hex = seqnum.to_padded_hex();
    let bal_1_hex = bal_1.to_padded_hex();
    let bal_0_hex = bal_0.to_padded_hex();

    let bytes = format!(
        "{}{}{}{}{}{}",
        "updateState", channel_id, seqnum_hex, bal_0_hex, bal_1_hex, hash_locks
    );
    encode(Hash::Keccak256, &bytes.as_bytes()).unwrap()
}

pub fn fn_sha256(secret: &ethkey::Secret, address: &ethkey::Address) -> Vec<serde_json::Value> {
    let val_1 = Int256::from(234);
    let val_2 = Int256::from(333);
    hash_and_sign(&val_1, &val_2, secret, address)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val_sig_addr() -> Vec<serde_json::Value> {
        let open_secret = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";
        let secret = Secret::from_str(open_secret).unwrap();
        let keypair = KeyPair::from_secret(secret).unwrap();
        fn_sha256(&keypair.secret(), &keypair.address())
    }

    fn test_evm_signed_by_both() -> Vec<serde_json::Value> {
        let open_secret_0 = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";
        let open_secret_1 = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9387f75";
        let channel_id = "1000000000000000000000000000000000000000000000000000000000000000";
        let hash_locks = "10";

        let secret_0 = Secret::from_str(open_secret_0).unwrap();
        let keypair_0 = KeyPair::from_secret(secret_0).unwrap();
        let secret_1 = Secret::from_str(open_secret_1).unwrap();
        let keypair_1 = KeyPair::from_secret(secret_1).unwrap();

        let bal_0 = Uint256::from(5000);
        let bal_1 = Uint256::from(15000);
        let seq_num = Uint256::from(987654321);

        let fingerprint = fingerprint_hash(channel_id, &seq_num, &bal_0, &bal_1, hash_locks);
        let msg = Message::from_slice(&fingerprint[2..]);

        let fhsigns = FHSigns {
            sig_0: format!("0x{}", sign(&keypair_0.secret(), &msg).unwrap()),
            sig_1: format!("0x{}", sign(&keypair_1.secret(), &msg).unwrap()),
            address_0: format!("0x{:?}", &keypair_0.address()),
            address_1: format!("0x{:?}", &keypair_1.address()),
            fingerprint: to_hex(&fingerprint[2..]),
        };

        let payload = serde_json::to_string(&fhsigns).unwrap();
        println!("{}", &payload);
        vec![serde_json::Value::String(payload)]
    }

    #[test]
    fn mock_test() {
        transport(val_sig_addr(), test_evm_signed_by_both());
    }
}
