extern crate serde;
extern crate serde_json;
extern crate num256;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate multihash;
extern crate ethkey;




#[macro_use]
extern crate serde_derive;



//use secp256k1::{Secp256k1, Message};
use std::str;
use multihash::{encode, Hash, to_hex};
use num256::Int256;
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use ethkey::{KeyPair, Secret, sign};
use ethkey::Message;
use std::str::FromStr;


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
    address: String,
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

pub fn sign_and_hash(fnm256: Int256, snm256: Int256, open_secret: &str) {

    let fnm256_str = serde_json::to_string(&fnm256).unwrap();
    let snm256_vec = serde_json::to_vec(&snm256).unwrap();

    let secret = Secret::from_str(open_secret).unwrap();
    let keypair = KeyPair::from_secret(secret).unwrap();

    let prefix = format!(
        "{}{}{}",
        "\x19Ethereum Signed Message:\n",
        fnm256_str.len(),
        fnm256_str
    );

    let hash1 = encode(Hash::SHA3256, &prefix.as_bytes()).unwrap();
    let hash2 = encode(Hash::SHA3256, &snm256_vec).unwrap();

    let msg1 = Message::from_slice(&hash1[2..]);
    let msg2 = Message::from_slice(&hash2[2..]);

    let sign1 = sign(keypair.secret(), &msg1).unwrap();
    let sign2 = sign(keypair.secret(), &msg2).unwrap();

    let sign1_hex = format!("{}", sign1);
    let sign2_hex = format!("{}", sign2);
    let addr = format!("{:?}", keypair.address());


    let tstr = ResSigs {
        sig1: sign1_hex,
        sig2: sign2_hex,
        address: addr,
        msg_hash1: to_hex(&hash1[2..]),
        msg_hash2: to_hex(&hash2[2..]),
    };

    let payload = serde_json::to_string(&tstr).unwrap();
    println!("{}", &payload);

    transport(payload);

}

pub fn generate_two_int256(j_str: &str) -> TwoInt256 {
    let tmp: TwoInt256 = serde_json::from_str(j_str).unwrap();
    tmp
}

pub fn fn_sha256(open_secret: &str) {

    let val_int256s = generate_two_int256("{\"int1\":\"234\",\"int2\":\"333\"}");


    sign_and_hash(val_int256s.int1, val_int256s.int2, open_secret);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transport() {
        let open_secret = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";

        fn_sha256(open_secret);
    }
}
