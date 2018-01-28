extern crate serde;
extern crate serde_json;
extern crate num256;
extern crate jsonrpc_core;
extern crate jsonrpc_http_server;
extern crate multihash;
extern crate ethkey;
extern crate rustc_hex;
extern crate pad;




#[macro_use]
extern crate serde_derive;

//use secp256k1::{Secp256k1, Message};
use pad::{PadStr, Alignment};
use std::str;
use multihash::{encode, Hash, to_hex};
use num256::Int256;
use serde::ser::Serialize;
use serde::{Deserialize, Deserializer, Serializer};
use ethkey::{KeyPair, Secret, sign};
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


fn transport(message: Vec<serde_json::Value>) {
    let mut io = IoHandler::new();
    io.add_method("say_hello", move |_: Params| {
        Ok(Value::Array(message.clone()))
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
	
	let num_size = 64; //the number of (assci) characters to have same signature as Web3
    //The number 64 is just the number of characters in Int256 when converted to hex

    //made the below to return hex
    //impl fmt::Display for Int256 {
    //fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //    write!(f, "{}", &self.to_str_radix(16))
    //}
    //}

	//Converts to hex and added padding to align with Web3
    let padded_fnm256_str = format!("{}", fnm256).pad(num_size, '0', Alignment::Right, true);
    let padded_snm256_str = format!("{}", snm256).pad(num_size, '0', Alignment::Right, true);
	format!("{}{}", padded_fnm256_str, padded_snm256_str)
}


//Have the rust program serialize two `num256::Int256`'s, hash them,
// sign them, then put them into the solidity which verifies the signature.
pub fn sign_and_hash(fnm256: Int256, snm256: Int256, secret: &ethkey::Secret, address: &ethkey::Address) {

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

    transport(vec![serde_json::Value::String(payload)]);
}


pub fn generate_two_int256(j_str: &str) -> TwoInt256 {
    let tmp: TwoInt256 = serde_json::from_str(j_str).unwrap();
    tmp
}

pub fn fn_sha256(secret: &ethkey::Secret, address: &ethkey::Address) {

    let val_int256s = generate_two_int256("{\"int1\":\"234\",\"int2\":\"333\"}");
    sign_and_hash(val_int256s.int1, val_int256s.int2, secret, address);

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transport() {
        let open_secret = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";
		
		let secret = Secret::from_str(open_secret).unwrap();
		let keypair = KeyPair::from_secret(secret).unwrap();
        fn_sha256(&keypair.secret(), &keypair.address() );
    }
}
