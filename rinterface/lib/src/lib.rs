extern crate eth_key;
extern crate ethkey;
extern crate multihash;
extern crate num256;
extern crate rpc_server;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod channel_types;
pub mod constants;

#[cfg(test)]
mod tests {
    use super::*;
    use channel_types::*;
    use constants::*;
    use multihash::to_hex;
    use num256::{PaddedHex, Uint256};
    use serde::ser::Serialize;
    use serde::{Deserialize, Deserializer, Serializer};
    use std::str;
    use std::str::FromStr;

    use eth_key::hash_and_sign;
    use ethkey::{KeyPair, Secret};
    use rpc_server::{transport, ChannelData};

    fn channel(chl_id: &str, open_secret_0: &str, open_secret_1: &str) -> serde_json::Value {
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        let keypair_1 = KeyPair::from_secret(Secret::from_str(open_secret_1).unwrap()).unwrap();

        let bal_0 = Uint256::from(15000);
        let bal_1 = Uint256::from(15000);
        let total_bal = bal_1.clone() + bal_0.clone();
        let settling_period_length = format!("0x{}", Uint256::from(2).to_padded_hex());
        let bogus_amount = format!("0x{}", Uint256::from(150000).to_padded_hex());
        let balance_0 = format!("0x{}", bal_0.to_padded_hex());
        let balance_1 = format!("0x{}", bal_1.to_padded_hex());
        let total_balance = format!("0x{}", total_bal.to_padded_hex());

        let wrong_address = "0xd68ff82bd0f8afeee459e6cbbf18d753576a8fff".to_owned();
        let wrong_sign = "0x77ffcdde8818c7f851fd5ac41fe5243684b649d5fccd4209a13f643b8c01aeb76501c656b297bcc618c274646072fe0fe3abf22af710563455b0f0f32252520400".to_owned();

        let address_0 = format!("0x{:?}", &keypair_0.address());
        let address_1 = format!("0x{:?}", &keypair_1.address());
        let new_chl = to_hex(&"newChannel".as_bytes());

        let channel = NewChannel {
            channel_id: chl_id.clone().to_string(),
            sign_priv_0: hash_and_sign(
                &keypair_0.secret(),
                &[
                    &new_chl,
                    &chl_id,
                    &address_0[2..],
                    &address_1[2..],
                    &balance_0[2..],
                    &balance_1[2..],
                    &settling_period_length[2..],
                ],
            ),
            sign_priv_1: hash_and_sign(
                &keypair_1.secret(),
                &[
                    &new_chl,
                    &chl_id,
                    &address_0[2..],
                    &address_1[2..],
                    &balance_0[2..],
                    &balance_1[2..],
                    &settling_period_length[2..],
                ],
            ),
            balance_0,
            balance_1,
            total_balance,
            settling_period_length,
            address_0,
            address_1,
            wrong_address,
            wrong_sign,
            bogus_amount,
        };

        let payload = serde_json::to_string(&channel).unwrap();
        serde_json::Value::String(payload)
    }

    fn start_settling_period(
        chl_id: &str,
        chl_id_wg: &str,
        open_secret_0: &str,
        open_secret_1: &str,
    ) -> serde_json::Value {
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        let keypair_1 = KeyPair::from_secret(Secret::from_str(open_secret_1).unwrap()).unwrap();

        let bal_0 = Uint256::from(17000);
        let bal_1 = Uint256::from(13000);
        let total_bal = bal_1.clone() + bal_0.clone();
        let seq_num = format!("0x{}", Uint256::from(1).to_padded_hex());
        let bal_0 = format!("0x{}", bal_0.to_padded_hex());
        let bal_1 = format!("0x{}", bal_1.to_padded_hex());
        let total_bal = format!("0x{}", total_bal.to_padded_hex());

        let wg_start_pd = to_hex(&"startSettlingPeriod derp".as_bytes());
        let start_pd = to_hex(&"startSettlingPeriod".as_bytes());
        let close_chl = to_hex(&"closeChannelFast".as_bytes());

        let start_settling = StartSettlingPeriod {
            channel_id: chl_id.clone().to_string(),
            bad_channel_id: chl_id_wg.clone().to_string(),
            sign_wrong_msg: hash_and_sign(&keypair_0.secret(), &[&wg_start_pd, &chl_id]),
            sign_wrong_id: hash_and_sign(&keypair_0.secret(), &[&start_pd, &chl_id_wg]),
            sign_start_settling_period: hash_and_sign(&keypair_0.secret(), &[&start_pd, &chl_id]),
            sign_start_settling_period_wrong_prv: hash_and_sign(
                &keypair_1.secret(),
                &[&start_pd, &chl_id],
            ),

            balance_0: bal_0.clone(),
            balance_1: bal_1.clone(),
            total_balance: total_bal.clone(),
            seq_num: seq_num.clone(),
            sign_close_chnl_fast_priv_0: hash_and_sign(
                &keypair_0.secret(),
                &[&close_chl, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
            sign_close_chnl_fast_priv_1: hash_and_sign(
                &keypair_1.secret(),
                &[&close_chl, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
        };

        let payload = serde_json::to_string(&start_settling).unwrap();
        serde_json::Value::String(payload)
    }

    fn update_state_hashlocks(
        chl_id: &str,
        open_secret_0: &str,
        open_secret_1: &str,
        hash_locks: &str,
    ) -> serde_json::Value {
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        let keypair_1 = KeyPair::from_secret(Secret::from_str(open_secret_1).unwrap()).unwrap();

        let seq_num = Uint256::from(1).to_padded_hex();
        let bal_0 = Uint256::from(17000).to_padded_hex();
        let bal_1 = Uint256::from(13000).to_padded_hex();

        let update_state = to_hex(&"updateState".as_bytes());

        let up_signs = UpdateStateHashLocks {
            channel_id: chl_id.clone().to_string(),
            update_state_sign_priv_0: hash_and_sign(
                &keypair_0.secret(),
                &[
                    &update_state,
                    &chl_id,
                    &seq_num,
                    &bal_0,
                    &bal_1,
                    &hash_locks,
                ],
            ),
            update_state_sign_priv_1: hash_and_sign(
                &keypair_1.secret(),
                &[
                    &update_state,
                    &chl_id,
                    &seq_num,
                    &bal_0,
                    &bal_1,
                    &hash_locks,
                ],
            ),
            start_settl_period_sign_priv_0: hash_and_sign(
                &keypair_0.secret(),
                &[&to_hex(&"startSettlingPeriod".as_bytes()), &chl_id],
            ),
        };

        let payload = serde_json::to_string(&up_signs).unwrap();
        serde_json::Value::String(payload)
    }

    fn update_state_signs(
        chl_id: &str,
        chl_id_wg: &str,
        open_secret_0: &str,
        open_secret_1: &str,
    ) -> serde_json::Value {
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        let keypair_1 = KeyPair::from_secret(Secret::from_str(open_secret_1).unwrap()).unwrap();

        let bal_0 = Uint256::from(17000);
        let bal_1 = Uint256::from(13000);
        let total_bal = bal_1.clone() + bal_0.clone();
        let seq_num = Uint256::from(1).to_padded_hex();
        let seq_num_bg = Uint256::from(2).to_padded_hex();
        let bounty_amount = Uint256::from(2).to_padded_hex();
        let bal_0 = bal_0.to_padded_hex();
        let bal_1 = bal_1.to_padded_hex();
        let total_bal = total_bal.to_padded_hex();

        let update_state = to_hex(&"updateState".as_bytes());
        let update_state_wt_bnty = to_hex(&"updateStateWithBounty".as_bytes());
        let update_state_wt_bnty_wg = to_hex(&"updateStateWithBounty derp".as_bytes());
        let bd_update_state = to_hex(&"updateState derp".as_bytes());

        let sign_priv_0 = hash_and_sign(
            &keypair_0.secret(),
            &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1],
        );
        let sign_priv_1 = hash_and_sign(
            &keypair_1.secret(),
            &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1],
        );
        let bounty_sign = hash_and_sign(
            &keypair_0.secret(),
            &[
                &update_state_wt_bnty,
                &chl_id,
                &seq_num,
                &bal_0,
                &bal_1,
                &sign_priv_0[2..],
                &sign_priv_1[2..],
                &bounty_amount,
            ],
        );
        let bounty_sign_bad_msg = hash_and_sign(
            &keypair_0.secret(),
            &[
                &update_state_wt_bnty_wg,
                chl_id,
                &seq_num,
                &bal_0,
                &bal_1,
                &sign_priv_0[2..],
                &sign_priv_1[2..],
                &bounty_amount,
            ],
        );

        let signs = UpdateStateSigns {
            sign_priv_0,
            sign_priv_1,
            bounty_sign,
            bounty_sign_bad_msg,
            sign_priv_0_bad_msg: hash_and_sign(
                &keypair_0.secret(),
                &[&bd_update_state, &chl_id, &seq_num, &bal_0, &bal_1],
            ),
            sign_priv_1_bad_msg: hash_and_sign(
                &keypair_1.secret(),
                &[&bd_update_state, &chl_id, &seq_num, &bal_0, &bal_1],
            ),
            sign_priv_0_wrong_id: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id_wg, &seq_num, &bal_0, &bal_1],
            ),
            sign_priv_1_wrong_id: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id_wg, &seq_num, &bal_0, &bal_1],
            ),
            sign_priv_0_wrong_seq_num: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &bal_0, &bal_1],
            ),
            sign_priv_1_wrong_seq_num: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &bal_0, &bal_1],
            ),
            sign_priv_0_wrong_balance: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num, &total_bal, &bal_1],
            ),
            sign_priv_1_wrong_balance: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &total_bal, &bal_1],
            ),
            sign_priv_0_bad_hashlocks: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1, "01"],
            ),
            sign_priv_1_bad_hashlocks: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1, "01"],
            ),
        };

        let payload = serde_json::to_string(&signs).unwrap();
        serde_json::Value::String(payload)
    }

    #[test]
    fn mock_test() {
        let data_1 = ChannelData {
            group: "rpc_data".to_string(),
            payload: vec![
                channel(CHL_ID_0, OPEN_SECRET_0, OPEN_SECRET_01),
                channel(CHL_ID_1, OPEN_SECRET_1, OPEN_SECRET_11),
                channel(CHL_ID_18, OPEN_SECRET_18, OPEN_SECRET_181),
                channel(CHL_ID_19, OPEN_SECRET_19, OPEN_SECRET_191),
            ],
        };

        let data_2 = ChannelData {
            group: "rpc_update".to_string(),
            payload: vec![
                start_settling_period(CHL_ID_0, CHL_ID_LT, OPEN_SECRET_0, OPEN_SECRET_01),
                start_settling_period(CHL_ID_1, CHL_ID_LT, OPEN_SECRET_1, OPEN_SECRET_11),
                start_settling_period(CHL_ID_18, CHL_ID_LT, OPEN_SECRET_18, OPEN_SECRET_181),
                start_settling_period(CHL_ID_19, CHL_ID_LT, OPEN_SECRET_19, OPEN_SECRET_191),
            ],
        };

        let data_3 = ChannelData {
            group: "rpc_signs".to_string(),
            payload: vec![
                update_state_signs(CHL_ID_0, CHL_ID_LT, OPEN_SECRET_0, OPEN_SECRET_01),
                update_state_signs(CHL_ID_1, CHL_ID_LT, OPEN_SECRET_1, OPEN_SECRET_11),
                update_state_signs(CHL_ID_18, CHL_ID_LT, OPEN_SECRET_18, OPEN_SECRET_181),
                update_state_signs(CHL_ID_19, CHL_ID_LT, OPEN_SECRET_19, OPEN_SECRET_191),
            ],
        };

        let data_4 = ChannelData {
            group: "rpc_update_sp".to_string(),
            payload: vec![
                update_state_hashlocks(CHL_ID_18, OPEN_SECRET_18, OPEN_SECRET_181, HASH_LOCK_SP_1),
                update_state_hashlocks(CHL_ID_19, OPEN_SECRET_19, OPEN_SECRET_191, HASH_LOCK_SP_2),
            ],
        };

        println!("Starting RPC_JSON server");
        let vec_data = vec![data_1, data_2, data_3, data_4];
        transport(vec_data);
    }
}
