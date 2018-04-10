use ethkey::{sign, Message, Secret};
use multihash::{encode, Hash};

pub fn hash_and_sign(secret: &Secret, bytes: &[u8]) -> String {
    let fingerprint = encode(Hash::Keccak256, &bytes).unwrap();
    let msg = Message::from_slice(&fingerprint[2..]);
    format!("0x{}", sign(&secret, &msg).unwrap())
}
