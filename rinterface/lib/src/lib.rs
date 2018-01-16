extern crate serde;
extern crate serde_json;
extern crate num256;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate multihash;
extern crate secp256k1;
extern crate rand;




#[macro_use]
extern crate serde_derive;



use secp256k1::{Secp256k1, Message};
use std::str;
use multihash::{encode, decode, Hash, to_hex};
use num256::Int256;
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use rand::{Rng, thread_rng};

use jsonrpc_core::*;
use jsonrpc_http_server::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TwoInt256 {
    int1: Int256,
    int2: Int256,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResSigs {
    sig1: String,
    sig2: String,
    pubkey: String,
    msg_hash1: String,
    msg_hash2: String,
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
        Err(_) => {}
    }
}



//Have the rust program serialize two `num256::Int256`'s, hash them,
// sign them, then put them into the solidity which verifies the signature.

pub fn sign_and_hash(fnm256: Int256, snm256: Int256) {

    let fnm256_str = serde_json::to_string(&fnm256).unwrap();
    let snm256_vec = serde_json::to_vec(&snm256).unwrap();

    let secp = Secp256k1::new();
    let (sk, pk) = secp.generate_keypair(&mut thread_rng()).unwrap();

    let prefix = format!(
        "{}{}{}",
        "\x19Ethereum Signed Message:\n",
        fnm256_str.len(),
        fnm256_str
    );
    let hash1 = encode(Hash::SHA3256, &prefix.as_bytes()).unwrap();
    let hash2 = encode(Hash::SHA3256, &snm256_vec).unwrap();

    let msg1 = Message::from_slice(&hash1[2..]).unwrap();
    let msg2 = Message::from_slice(&hash2[2..]).unwrap();

    let sig1 = secp.sign_recoverable(&msg1, &sk).unwrap();
    let sig2 = secp.sign_recoverable(&msg2, &sk).unwrap();

    let (recid1, vec1) = sig1.serialize_compact(&secp);
    let (recid2, vec2) = sig2.serialize_compact(&secp);
    //let fnm256_str = to_hex(&vectt);

    let sign1_hex = format!("{}0{}", to_hex(&vec1), recid1.to_i32());
    let sign2_hex = format!("{}0{}", to_hex(&vec2), recid2.to_i32());

    let hashed_pub_key = encode(Hash::SHA3256, &pk.serialize_uncompressed()[1..]).unwrap();

    let tstr = ResSigs {
        sig1: sign1_hex,
        sig2: sign2_hex,
        pubkey: to_hex(&hashed_pub_key[14..]),
        msg_hash1: to_hex(&hash1[2..]),
        msg_hash2: to_hex(&hash2[2..]),
    };

    let payload = serde_json::to_string(&tstr).unwrap();
    println!("{}", &payload);

    //0x135a7de83802408321b74c322f8558db1679ac20"
    transport(payload);

}


pub fn generate_two_int256(j_str: &str) -> TwoInt256 {
    let tmp: TwoInt256 = serde_json::from_str(j_str).unwrap();
    tmp
}

pub fn fn_sha256() {

    let val_int256s = generate_two_int256("{\"int1\":\"234\",\"int2\":\"333\"}");
    sign_and_hash(val_int256s.int1, val_int256s.int2);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transport() {

        fn_sha256();
    }
}
