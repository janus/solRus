use ethkey::Message;
use ethkey::{sign, KeyPair, Secret};
use multihash::{encode, to_hex, Hash};
use rustc_hex::{FromHex, ToHex};

pub fn hash_and_sign(secret: &Secret, vec_str: &[&str]) -> String {
    let mut str_val = String::new();
    for item in vec_str {
        str_val.push_str(item);
    }

    let bytes = str_val.from_hex().unwrap();
    let fingerprint = encode(Hash::Keccak256, &bytes).unwrap();
    let msg = Message::from_slice(&fingerprint[2..]);
    format!("0x{}", sign(&secret, &msg).unwrap())
}
