extern crate serde;
extern crate serde_json;
extern crate edcert;
extern crate num;
extern crate sha2;
extern crate bytes;
extern crate base64;
extern crate num256;
extern crate regex;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;

#[macro_use]
extern crate serde_derive;


use sha2::Sha256;
use sha2::Digest;
use regex::Regex;
use edcert::ed25519;
use base64::{encode, decode};
use num256::Int256;
use num::bigint::{BigInt, BigUint};
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};

use jsonrpc_core::*;
use jsonrpc_http_server::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct two_Int256 {
    int1: Int256,
    int2: Int256,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct res_Sigs {
    sig1: String,
    sig2: String,
    pubkey: String,
}
fn transport(message: String) {
    let mut io = IoHandler::new();
    io.add_method("say_hello", move |_: Params| {
        Ok(Value::String(message.clone()))
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
        Err(n) => {}
    }
}


//Have the rust program serialize two `num256::Int256`'s, hash them,
// sign them, then put them into the solidity which verifies the signature.

pub fn sign_and_hash(fnm256: Int256, snm256: Int256, priv_key: &[u8; 64], pubk: &[u8]) {

    let fnm256_str = serde_json::to_string(&fnm256).unwrap();
    let snm256_str = serde_json::to_string(&snm256).unwrap();

    let mut fnm256_hash: Sha256 = Digest::new();
    let mut snm256_hash: Sha256 = Digest::new();

    fnm256_hash.input(fnm256_str.as_bytes());
    snm256_hash.input(snm256_str.as_bytes());

    let hash1 = fnm256_hash.result();
    let hash2 = snm256_hash.result();

    let sig1 = ed25519::sign(encode(&hash1).as_bytes(), priv_key);
    let sig2 = ed25519::sign(encode(&hash2).as_bytes(), priv_key);


    let tstr = res_Sigs {
        sig1: encode(&sig1).to_string(),
        sig2: encode(&sig2).to_string(),
        pubkey: encode(&pubk),
    };

    let j = serde_json::to_string(&tstr).unwrap();

    transport(j);

}

pub fn generate_two_Int256(j_str: &str) -> two_Int256 {

    let tmp: two_Int256 = serde_json::from_str(j_str).unwrap();
    tmp
}

pub fn fn_sha256() {

    let val_int256s = generate_two_Int256("{\"int1\":\"234\",\"int2\":\"333\"}");
    let (psk, msk) = ed25519::generate_keypair();

    sign_and_hash(val_int256s.int1, val_int256s.int2, &msk, &psk);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transport() {
        fn_sha256();
    }
}
