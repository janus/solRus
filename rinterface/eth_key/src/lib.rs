extern crate ethkey;
extern crate multihash;

pub mod eth_hash_and_signature;

pub use self::eth_hash_and_signature::hash_and_sign;

#[cfg(test)]
mod tests {
    use super::*;
    use ethkey::{KeyPair, Secret};
    use std::str::FromStr;

    fn update_state_signs() -> String {
        let open_secret_0 = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        hash_and_sign(&keypair_0.secret(), "updateState".as_bytes())
    }

    #[test]
    fn test_hash_and_sign() {
        let update_state_signed_val = "0xc668d80913a092307a51da6ce1c15cdf1e1d15821c6cdd3fcfa7e176571829b02232381c31db720e5cee2333bc13cb9bd76785841eeb428dfeff371a82def39401";
        assert_eq!(update_state_signs(), update_state_signed_val);
    }
}
