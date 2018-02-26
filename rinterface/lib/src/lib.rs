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
pub struct FHSigns {
    chl_id: String,
    sig_0: String,
    sig_1: String,
    bal_0: String,
    bal_1: String,
    total_bal: String,
    set_period_ln: String,
    addr_0: String,
    addr_1: String,
    bogus_addr: String,
    bogus_sign: String,
    bogus_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UPSigns {
    chl_id: String,
    chl_id_wg: String,
    sig_0: String,
    sig_1: String,
    sig_bogus_msg: String,
    sig_wrong_id: String,
    sig_start_stl_p: String,
    sig_start_stl_wg_prv: String,
    bal_0: String,
    bal_1: String,
    total_bal: String,
    seq_num: String,
    sig_0_cl: String,
    sig_1_cl: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SIGNBlock {
    sign_1: String,
    sign_2: String,
    sign_bt: String,
    sign_bt_bd: String,
    sig_st_derp_1: String,
    sig_st_derp_2: String,
    sig_st_id_1: String,
    sig_st_id_2: String,
    sig_st_sqn_1: String,
    sig_st_sqn_2: String,
    sig_st_bl_1: String,
    sig_st_bl_2: String,
    sig_st_hl_1: String,
    sig_st_hl_2: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UPSp {
    chl_id: String,
    sig_0: String,
    sig_1: String,
    sig_start_stl_p: String,
}

pub fn transport(
    msg_1: Vec<serde_json::Value>,
    msg_2: Vec<serde_json::Value>,
    msg_3: Vec<serde_json::Value>,
    msg_4: Vec<serde_json::Value>,
) {
    let mut io = IoHandler::new();
    io.add_method("rpc_update", move |_: Params| {
        Ok(Value::Array(msg_1.clone()))
    });

    io.add_method("rpc_update_sp", move |_: Params| {
        Ok(Value::Array(msg_4.clone()))
    });

    io.add_method("rpc_data", move |_: Params| Ok(Value::Array(msg_2.clone())));

    io.add_method("rpc_signs", move |_: Params| {
        Ok(Value::Array(msg_3.clone()))
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

pub fn hash_and_sign(secret: &ethkey::Secret, vec_str: &[&str]) -> String {
    let mut str_vl = String::new();
    for em in vec_str {
        str_vl.push_str(em);
    }

    let bytes = str_vl.from_hex().unwrap();
    let fingerprint = encode(Hash::Keccak256, &bytes).unwrap();
    let msg = Message::from_slice(&fingerprint[2..]);
    format!("0x{}", sign(&secret, &msg).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_evm_signed_by_both(
        chl_id: &str,
        open_secret_0: &str,
        open_secret_1: &str,
    ) -> serde_json::Value {
        let keypair_0 = KeyPair::from_secret(Secret::from_str(open_secret_0).unwrap()).unwrap();
        let keypair_1 = KeyPair::from_secret(Secret::from_str(open_secret_1).unwrap()).unwrap();

        let bal_0 = Uint256::from(15000);
        let bal_1 = Uint256::from(15000);
        let total_bal = bal_1.clone() + bal_0.clone();
        let set_period_ln = format!("0x{}", Uint256::from(2).to_padded_hex());
        let bogus_amount = format!("0x{}", Uint256::from(150000).to_padded_hex());
        let bal_0 = format!("0x{}", bal_0.to_padded_hex());
        let bal_1 = format!("0x{}", bal_1.to_padded_hex());
        let total_bal = format!("0x{}", total_bal.to_padded_hex());

        let bogus_addr = "0xd68ff82bd0f8afeee459e6cbbf18d753576a8fff".to_owned();
        let bogus_sign = "0x77ffcdde8818c7f851fd5ac41fe5243684b649d5fccd4209a13f643b8c01aeb76501c656b297bcc618c274646072fe0fe3abf22af710563455b0f0f32252520400".to_owned();

        let addr_0 = format!("0x{:?}", &keypair_0.address());
        let addr_1 = format!("0x{:?}", &keypair_1.address());
        let new_chl = to_hex(&"newChannel".as_bytes());

        let fhsigns = FHSigns {
            chl_id: chl_id.clone().to_string(),
            sig_0: hash_and_sign(
                &keypair_0.secret(),
                &[
                    &new_chl,
                    &chl_id,
                    &addr_0[2..],
                    &addr_1[2..],
                    &bal_0[2..],
                    &bal_1[2..],
                    &set_period_ln[2..],
                ],
            ),
            sig_1: hash_and_sign(
                &keypair_1.secret(),
                &[
                    &new_chl,
                    &chl_id,
                    &addr_0[2..],
                    &addr_1[2..],
                    &bal_0[2..],
                    &bal_1[2..],
                    &set_period_ln[2..],
                ],
            ),
            bal_0,
            bal_1,
            total_bal,
            set_period_ln,
            addr_0,
            addr_1,
            bogus_addr,
            bogus_sign,
            bogus_amount,
        };

        let payload = serde_json::to_string(&fhsigns).unwrap();
        serde_json::Value::String(payload)
    }

    fn test_evm_update(
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

        let update = to_hex(&"updateState".as_bytes());
        let wg_start_pd = to_hex(&"startSettlingPeriod derp".as_bytes());
        let start_pd = to_hex(&"startSettlingPeriod".as_bytes());
        let close_chl = to_hex(&"closeChannelFast".as_bytes());

        let up_signs = UPSigns {
            chl_id: chl_id.clone().to_string(),
            chl_id_wg: chl_id_wg.clone().to_string(),
            sig_0: hash_and_sign(
                &keypair_0.secret(),
                &[&update, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
            sig_1: hash_and_sign(
                &keypair_1.secret(),
                &[&update, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
            sig_bogus_msg: hash_and_sign(&keypair_0.secret(), &[&wg_start_pd, &chl_id]),
            sig_wrong_id: hash_and_sign(&keypair_0.secret(), &[&start_pd, &chl_id_wg]),
            sig_start_stl_p: hash_and_sign(&keypair_0.secret(), &[&start_pd, &chl_id]),
            sig_start_stl_wg_prv: hash_and_sign(&keypair_1.secret(), &[&start_pd, &chl_id]),

            bal_0: bal_0.clone(),
            bal_1: bal_1.clone(),
            total_bal: total_bal.clone(),
            seq_num: seq_num.clone(),
            sig_0_cl: hash_and_sign(
                &keypair_0.secret(),
                &[&close_chl, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
            sig_1_cl: hash_and_sign(
                &keypair_1.secret(),
                &[&close_chl, &chl_id, &seq_num[2..], &bal_0[2..], &bal_1[2..]],
            ),
        };

        let payload = serde_json::to_string(&up_signs).unwrap();
        serde_json::Value::String(payload)
    }

    fn test_evm_update_sp(
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

        let up_signs = UPSp {
            chl_id: chl_id.clone().to_string(),
            sig_0: hash_and_sign(
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
            sig_1: hash_and_sign(
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
            sig_start_stl_p: hash_and_sign(
                &keypair_0.secret(),
                &[&to_hex(&"startSettlingPeriod".as_bytes()), &chl_id],
            ),
        };

        let payload = serde_json::to_string(&up_signs).unwrap();
        serde_json::Value::String(payload)
    }

    fn test_sign_list(
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

        let sign_1 = hash_and_sign(
            &keypair_0.secret(),
            &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1],
        );
        let sign_2 = hash_and_sign(
            &keypair_1.secret(),
            &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1],
        );
        let sign_bt = hash_and_sign(
            &keypair_0.secret(),
            &[
                &update_state_wt_bnty,
                &chl_id,
                &seq_num,
                &bal_0,
                &bal_1,
                &sign_1[2..],
                &sign_2[2..],
                &bounty_amount,
            ],
        );
        let sign_bt_bd = hash_and_sign(
            &keypair_0.secret(),
            &[
                &update_state_wt_bnty_wg,
                chl_id,
                &seq_num,
                &bal_0,
                &bal_1,
                &sign_1[2..],
                &sign_2[2..],
                &bounty_amount,
            ],
        );

        let signs = SIGNBlock {
            sign_1,
            sign_2,
            sign_bt,
            sign_bt_bd,
            sig_st_derp_1: hash_and_sign(
                &keypair_0.secret(),
                &[&bd_update_state, &chl_id, &seq_num, &bal_0, &bal_1],
            ),
            sig_st_derp_2: hash_and_sign(
                &keypair_1.secret(),
                &[&bd_update_state, &chl_id, &seq_num, &bal_0, &bal_1],
            ),
            sig_st_id_1: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id_wg, &seq_num, &bal_0, &bal_1],
            ),
            sig_st_id_2: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id_wg, &seq_num, &bal_0, &bal_1],
            ),
            sig_st_sqn_1: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &bal_0, &bal_1],
            ),
            sig_st_sqn_2: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &bal_0, &bal_1],
            ),
            sig_st_bl_1: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num, &total_bal, &bal_1],
            ),
            sig_st_bl_2: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num_bg, &total_bal, &bal_1],
            ),
            sig_st_hl_1: hash_and_sign(
                &keypair_0.secret(),
                &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1, "01"],
            ),
            sig_st_hl_2: hash_and_sign(
                &keypair_1.secret(),
                &[&update_state, &chl_id, &seq_num, &bal_0, &bal_1, "01"],
            ),
        };

        let payload = serde_json::to_string(&signs).unwrap();
        serde_json::Value::String(payload)
    }

    #[test]
    fn mock_test() {
        let open_secret_0 = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9383f65";
        let open_secret_01 = "11ac864e2ea71dfcee703a812b3cede2f13e0ff3ebf1a1557fec5c433cd8bc11";
        let open_secret_1 = "a100df7a048e50ed308ea696dc600215098141cb391e9527329df289f9387f75";
        let open_secret_11 = "7bb6d789e06add997292e1164ec8274a96ed6acba9e05c05e60c61d4b755810e";

        let open_secret_18 = "193c5c799570443f817293a3bc68e9deee88a8fba24a2fd7425da1d0574911fd";
        let open_secret_181 = "f6bec5473c3c5fbcc77f62595566feb9071a11b04b30abe1cd27af38a6f910dd";
        let open_secret_19 = "14b0f52bf6a235c8db20e7bd4e739b445d7dc0860be6b56d8b7a4c41cbd00579";
        let open_secret_191 = "d59be801fb2f9341bd7fbb51d9e4cb8fd18126fb3f5267ff5cf68dfe8b3240e6";

        let chl_id_0 = "1000000000000000000000000000000000000000000000000000000000000000";
        let chl_id_1 = "2000000000000000000000000000000000000000000000000000000000000000";
        let chl_id_18 = "2000000000000000000000000000000000000000000000000000000000000000";
        let chl_id_19 = "2100000000000000000000000000000000000000000000000000000000000000";
        let chl_id_lt = "7600000000000000000000000000000000000000000000000000000000000000";

        let hash_lock_sp_1 = "b378812c6f48641f0e297db1e5f8d9254631795df40a86803323bb9994bdd174fffff\
    fffffffffffffffffffffffffffffffffffffffffffffffffffffffd8ee7b1cffef36bc044d4d86b29ca4bdd91e42c6e81\
        93372ca09a109da9b44281e320000000000000000000000000000000000000000000000000000000000002711e71bb8\
        17e6d260f83cc84a5a113594013b4eeb456ce3a172d0d7d83bce7ed1060000000000000000000000000000000000000\
        000000000000000000000000002";

        let hash_lock_sp_2 = "290decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e563000000000000000000000000000000000\
        0000000000000000000000000000001b10e2d527612073b26eecdfd717e6a320cf44b4afac2b0732d9fcbe2b7fa0cf6fffffffffffffffffffffffffff\
        fffffffffffffffffffffffffffffffffffff405787fa12a823e0f2b7631cc41b3ba8828b3321ca811111fa75cd3aa3bb5ace000000000000000000000\
        0000000000000000000000000000000000000000001c2575a0e9e593c00f959f8c92f12db2869c3395a3b0502d05e2516446f71f85bfffffffffffffff\
        fffffffffffffffffffffffffffffffffffffffffffffffff8a35acfbc15ff81a39ae7d344fd709f28e8600b4aa8c65c6b64bfe7fe36bd19b000000000\
        0000000000000000000000000000000000000000000000000000001036b6384b5eca791c62761152d0c79bb0604c104a5fb6f4eb0703f3154bb3db0fff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff652222313e28459528d920b65115c16c04f3efc82aaedc97be59f3f377c0\
        d3f0000000000000000000000000000000000000000000000000000000000000001a66cc928b5edb82af9bd49922954155ab7b0942694bea4ce44661d9\
        a8736c688fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff3f7a9fe364faab93b216da50a3214154f22a0a2b415b23a8\
        4c8169e8b636ee300000000000000000000000000000000000000000000000000000000000000016e1540171b6c0c960b71a7020d9f60077f6af931a8b\
        bf590da0223dacf75c7afffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc65a7bb8d6351c1cf70c95a316cc6a92839c9\
        86682d98bc35f958f4883f9d2a800000000000000000000000000000000000000000000000000000000000000010175b7a638427703f0dbe7bb9bbf987\
        a2551717b34e79f33b5b1008d1fa01db9ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffdf6966c971051c3d54ec59162\
        606531493a51404a002842f56009d7e5cf4a8c70000000000000000000000000000000000000000000000000000000000000001d7b6990105719101dab\
        eb77144f2a3385c8033acd3af97e9423a695e81ad1eb5ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbb7b4a454dc349\
        3923482f07822329ed19e8244eff582cc204f8554c3620c3fd00000000000000000000000000000000000000000000000000000000000000018d1108e10\
        bcb7c27dddfc02ed9d693a074039d026cf4ea4240b40f7d581ac802ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1b68\
        47dc741a1b0cd08d278845f9d819d87b734759afb55fe2de5cb82a9ae672000000000000000000000000000000000000000000000000000000000000000\
        131ecc21a745e3968a04e9570e4425bc18fa8019c68028196b546d1669c200c68ffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffbb8a6a4669ba250d26cd7a459eca9d215f8307e33aebe50379bc5a3617ec344400000000000000000000000000000000000000000000000000000\
        0000000000166de8ffda797e3de9c05e8fc57b3bf0ec28a930d40b0d285d93c06501cf6a090ffffffffffffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffce6d7b5282bd9a3661ae061feed1dbda4e52ab073b1f9285be6e155d9c38d4ec0000000000000000000000000000000000000000000\
        00000000000000000000155f448fdea98c4d29eb340757ef0a66cd03dbb9538908a6a81d96026b71ec475ffffffffffffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffd833147d7dc355ba459fc788f669e58cfaf9dc25ddcd0702e87d69c7b5124289000000000000000000000000000000000\
        0000000000000000000000000000001c624b66cc0138b8fabc209247f72d758e1cf3343756d543badbf24212bed8c15ffffffffffffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffb13d2d76d1f4b7be834882e410b3e3a8afaf69f83600ae24db354391d2378d2e00000000000000000000000\
        00000000000000000000000000000000000000001944998273e477b495144fb8794c914197f3ccb46be2900f4698fd0ef743c9695ffffffffffffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffff057c384a7d1c54f3a1b2e5e67b2617b8224fdfd1ea7234eea573a6ff665ff63e0000000000000\
        0000000000000000000000000000000000000000000000000013ad8aa4f87544323a9d1e5dd902f40c356527a7955687113db5f9a85ad579dc1ffffffff\
        ffffffffffffffffffffffffffffffffffffffffffffffffffffffff0e4562a10381dec21b205ed72637e6b1b523bdd0e4d4d50af5cd23dd4500a211000\
        00000000000000000000000000000000000000000000000000000000000016d4407e7be21f808e6509aa9fa9143369579dd7d760fe20a2c09680fc146134\
        fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff50bb669a95c7b50b7e8a6f09454034b2b14cf2b85c730dca9a539ca82cb\
        6e3500000000000000000000000000000000000000000000000000000000000000001a03837a25210ee280c2113ff4b77ca23440b19d4866cca721c8012\
        78fd08d807ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc97bfaf2f8ee708c303a06d134f5ecd8389ae0432af62dc13\
        2a24118292866bb00000000000000000000000000000000000000000000000000000000000000013a6357012c1a3ae0a17d304c9920310382d968ebcc4b1\
        771f41c6b304205b570ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff61035b26e3e9eee00e0d72fd1ee8ddca6894550dc\
        a6916ea2ac6baa90d11e5100000000000000000000000000000000000000000000000000000000000000001d57b2b5166478fd4318d2acc6cc2c70458431\
        2bdd8781b32d5d06abda57f4230ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7cd332d19b93bcabe3cce7ca0c18a052f\
        57e5fd03b4758a09f30f5ddc4b22ec40000000000000000000000000000000000000000000000000000000000000001401968ff42a154441da5f6c4c935a\
        c46b8671f0e062baaa62a7545ba53bb6e4cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff744a2cf8fd7008e3d53b67916e\
        73460df9fa5214e3ef23dd4259ca09493a3594000000000000000000000000000000000000000000000000000000000000000198a476f1687bc3d60a2da2\
        adbcba2c46958e61fa2fb4042cd7bc5816a710195bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe16da923a2d88192e5\
        070f37b4571d58682c0d66212ec634d495f33de3f77ab50000000000000000000000000000000000000000000000000000000000000001cb7c14ce178f56e\
        2e8d86ab33ebc0ae081ba8556a00cd122038841867181caacffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffbeced0952104\
        7d05b8960b7e7bcc1d1292cf3e4b2a6b63f48335cbde5f7545d2000000000000000000000000000000000000000000000000000000000000000111c44e487\
        5b74d31ff9fd779bf2566af7bd15b87fc985d01f5094b89e3669e4fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7416c94\
        3b4a09859521022fd2e90eac0dd9026dad28fa317782a135f28a8609100000000000000000000000000000000000000000000000000000000000000014a2cc9\
        1ee622da3bc833a54c37ffcb6f3ec23b7793efc5eaf5e71b7b406c5c06ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff37fa1\
        66cbdbfbb1561ccd9ea985ec0218b5e68502e230525f544285b2bdf3d7e0000000000000000000000000000000000000000000000000000000000000001a813\
        484aef6fb598f9f753daf162068ff39ccea4075cb95e1a30f86995b5b7eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6ff\
        97a59c90d62cc7236ba3a37cd85351bf564556780cf8c1157a220f31f0cbb0000000000000000000000000000000000000000000000000000000000000001c5\
        4045fa7c6ec765e825df7f9e9bf9dec12c5cef146f93a5eee56772ee647fbcffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff1\
        1df491316f14931039edfd4f8964c9a443b862f02d4c7611d18c2bc4e6ff6970000000000000000000000000000000000000000000000000000000000000001\
        82a75bdeeae8604d839476ae9efd8b0e15aa447e21bfd7f41283bb54e22c9a82fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        f46bddb1178e94d7f2892ff5f366840eb658911794f2c3a44c450aa2c505186c1000000000000000000000000000000000000000000000000000000000000000\
        1cfa4bec1d3298408bb5afcfcd9c430549c5b31f8aa5c5848151c0a55f473c34dfffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
        f4a11f94e20a93c79f6ec743a1954ec4fc2c08429ae2122118bf234b2185c81b8000000000000000000000000000000000000000000000000000000000000000\
    142a7b7dd785cd69714a189dffb3fd7d7174edc9ece837694ce50f7078f7c31aeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
    38395c5dceade9603479b177b68959049485df8aa97b39f3533039af5f4561990000000000000000000000000000000000000000000000000000000000000001d\
    c16fef70f8d5ddbc01ee3d903d1e69c18a3c7be080eb86a81e0578814ee58d3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa2\
    999d817b6757290b50e8ecf3fa939673403dd35c97de392fdb343b4015ce9e0000000000000000000000000000000000000000000000000000000000000001bbe\
    3212124853f8b0084a66a2d057c2966e251e132af3691db153ab65f0d1a4dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc6bb0\
    6cb7f92603de181bf256cd16846b93b752a170ff24824098b31aa008a7e0000000000000000000000000000000000000000000000000000000000000001ece66cf\
    dbd22e3f37d348a3d8e19074452862cd65fd4b9a11f0336d1ac6d1dc3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8d800d661\
    4d35eed73733ee453164a3b48076eb3138f466adeeb9dec7bb31f700000000000000000000000000000000000000000000000000000000000000001c03004e3ce07\
    84bf68186394306849f9b7b1200073105cd9aeb554a1802b58fdffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff352feee0eea125f1\
    1f791c1b77524172e9bc20f1b719b6cef0fc24f64db8e15e00000000000000000000000000000000000000000000000000000000000000017c9785e8241615bc804\
    15d89775984a1337d15dc1bf4ce50f41988b2a2b336a7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff38dfe4635b27babeca8be3\
    8d3b448cb5161a639b899a14825ba9c8d7892eb8c300000000000000000000000000000000000000000000000000000000000000019690ad99d6ce244efa8a0f6c2\
    d04036d3b33a9474db32a71b71135c695102793ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9b22d3d61959b4d3528b1d8ba932\
    c96fbe302b36a1aad1d95cab54f9e0a135ea0000000000000000000000000000000000000000000000000000000000000001a80a8fcc11760162f08bb091d2c938\
    9d07f2b73d0e996161dfac6f1043b5fc0bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff128667f541fed74a8429f9d592c26c2c\
    6a4beb9ae5ead9912c98b2595c8423100000000000000000000000000000000000000000000000000000000000000001c43c1e24e1884c4e28a16bbd9506f60b5c\
    a9f18fc90635e729d3cfe13abcf001ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff15040156076f78057c0a886f6dbac29221fa\
    3c2646adbc8effedab98152ff32b000000000000000000000000000000000000000000000000000000000000000137e472f504e93744df80d87316862f9a8fd41a\
    7bc266c723bf77df7866d75f55fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffcc5ba1a98fc477b8948a04d08c6f4a76181fe75\
    021370ab5e6abd22b1792a2a000000000000000000000000000000000000000000000000000000000000000117b0af156a929edf60c351f3df2d53ed643fdd750a\
    ef9eda90dc7c8759a104a8ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff42859d4f253f4d4a28ee9a59f9c9683a9404da2c5d32\
    9c733ab84f150db798a800000000000000000000000000000000000000000000000000000000000000011b524e1c8b5382bb913d0a2aae8ad83bb92a45fcb47761\
    fa4a12f5b6316c2b20ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9b65e484ce3d961a557081a44c6c68a0a27eca0b88fce820b\
    dd99c3dc223dcc70000000000000000000000000000000000000000000000000000000000000001a2e8f972dc9f7d0b76177bb8be102e6bec069ee42c61080745e88\
    25470e80c6cffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff5529612556959ef813dbe8d0ed29336ab75e80a9b7855030760b2917b\
01e568a0000000000000000000000000000000000000000000000000000000000000001994a4b4eddb300691ee19901712848b1114bad8a1a4ae195e5abe0ec38021b\
94ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffa9144a5e7efd259b8b0d55467f4696ed47ec83317d61501b76366dbcca65ce73000\
00000000000000000000000000000000000000000000000000000000000014c83efb3982afbd500ab7c66d02b996df5fdc3d20660e61600390aad6d5f7f1effffffff\
fffffffffffffffffffffffffffffffffffffffffffffffffffffffff0d642dbc7517672e217238a2f008f4f8cdad0586d8ce5113e9e09dcc686061900000000000000\
0000000000000000000000000000000000000000000000000171beda120aafdd3bb922b360a066d10b7ce81d7ac2ad9874daac46e2282f6b45ffffffffffffffffffff\
ffffffffffffffffffffffffffffffffffffffffffffea7419f5ae821e7204864e6a0871433ba612011908963bb42a64f42d65ad2f7200000000000000000000000000\
00000000000000000000000000000000000001e8e5595d268aaa85b36c3557e9d96c14a4fffaee9f45bcae0c407968a7109630fffffffffffffffffffffffffffffff\
fffffffffffffffffffffffffffffffff657000d47e971dcfb21375bcfa3496f47a2a2f0f12c8aeb78a008ace6ae55ca5000000000000000000000000000000000000\
0000000000000000000000000001d73956b9e00d8f8bc5e44f7184df1387cdd652e7726b8ccda3db4859e02f31bffffffffffffffffffffffffffffffffffffffffff\
fffffffffffffffffffffffe8c3abd4193a84ec8a3fff3eeb3ecbcbd0979e0c977ac1dee06c6e01a60aca1b0000000000000000000000000000000000000000000000\
000000000000000001fcebc02dd307dc58cd01b156d63c6948b8f3422055fac1d836349b01722e9c52fffffffffffffffffffffffffffffffffffffffffffffffffff\
fffffffffffffec0b854938343f85eb39a6648b9e449c2e4aee4dc9b4e96ab592f9f497d0513800000000000000000000000000000000000000000000000000000000\
000000012619ec68b255542e3da68c054bfe0d7d0f27b7fdbefc8bbccdd23188fc71fe7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
ffff34d3c319f536deb74ed8f1f3205d9aefef7487c819e77d3351630820dbff11180000000000000000000000000000000000000000000000000000000000000001\
cc7ee599e5d59fee88c83157bd897847c5911dc7d317b3175e0b085198349973ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff41c7\
ae758795765c6664a5d39bf63841c71ff191e9189522bad8ebff5d4eca980000000000000000000000000000000000000000000000000000000000000001f0ecb75d\
d1820844c57b6762233d4e26853b3a7b8157bbd9f41f280a0f1cee9bffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb912c5eb6319\
a4a6a83580b9611610bedb31614179330261bfd87a41347cae1c0000000000000000000000000000000000000000000000000000000000000001d86d8a3f7c82c89\
ed8e04140017aa108a0a1469249f92c8f022b9dbafa87b883ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

        let rst_data = vec![
            test_evm_signed_by_both(chl_id_0, open_secret_0, open_secret_01),
            test_evm_signed_by_both(chl_id_1, open_secret_1, open_secret_11),
            test_evm_signed_by_both(chl_id_18, open_secret_18, open_secret_181),
            test_evm_signed_by_both(chl_id_19, open_secret_19, open_secret_191),
        ];

        let rst_update = vec![
            test_evm_update(chl_id_0, chl_id_lt, open_secret_0, open_secret_01),
            test_evm_update(chl_id_1, chl_id_lt, open_secret_1, open_secret_11),
            test_evm_update(chl_id_18, chl_id_lt, open_secret_18, open_secret_181),
            test_evm_update(chl_id_19, chl_id_lt, open_secret_19, open_secret_191),
        ];

        let rst_signs = vec![
            test_sign_list(chl_id_0, chl_id_lt, open_secret_0, open_secret_01),
            test_sign_list(chl_id_1, chl_id_lt, open_secret_1, open_secret_11),
            test_sign_list(chl_id_18, chl_id_lt, open_secret_18, open_secret_181),
            test_sign_list(chl_id_19, chl_id_lt, open_secret_19, open_secret_191),
        ];

        let rst_update_sp = vec![
            test_evm_update_sp(chl_id_18, open_secret_18, open_secret_181, hash_lock_sp_1),
            test_evm_update_sp(chl_id_19, open_secret_19, open_secret_191, hash_lock_sp_2),
        ];

        transport(rst_update, rst_data, rst_signs, rst_update_sp);
    }
}
