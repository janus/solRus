use serde::Deserialize;
use serde::ser::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewChannel {
    pub channel_id: String,
    pub sign_priv_0: String,
    pub sign_priv_1: String,
    pub balance_0: String,
    pub balance_1: String,
    pub total_balance: String,
    pub settling_period_length: String,
    pub address_0: String,
    pub address_1: String,
    pub wrong_address: String,
    pub wrong_sign: String,
    pub bogus_amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartSettlingPeriod {
    pub channel_id: String,
    pub bad_channel_id: String,
    pub sign_wrong_msg: String,
    pub sign_wrong_id: String,
    pub sign_start_settling_period: String,
    pub sign_start_settling_period_wrong_prv: String,
    pub balance_0: String,
    pub balance_1: String,
    pub total_balance: String,
    pub seq_num: String,
    pub sign_close_chnl_fast_priv_0: String,
    pub sign_close_chnl_fast_priv_1: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateStateSigns {
    pub sign_priv_0: String,
    pub sign_priv_1: String,
    pub bounty_sign: String,
    pub bounty_sign_bad_msg: String,
    pub sign_priv_0_bad_msg: String,
    pub sign_priv_1_bad_msg: String,
    pub sign_priv_0_wrong_id: String,
    pub sign_priv_1_wrong_id: String,
    pub sign_priv_0_wrong_seq_num: String,
    pub sign_priv_1_wrong_seq_num: String,
    pub sign_priv_0_wrong_balance: String,
    pub sign_priv_1_wrong_balance: String,
    pub sign_priv_0_bad_hashlocks: String,
    pub sign_priv_1_bad_hashlocks: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateStateHashLocks {
    pub channel_id: String,
    pub update_state_sign_priv_0: String,
    pub update_state_sign_priv_1: String,
    pub start_settl_period_sign_priv_0: String,
}
