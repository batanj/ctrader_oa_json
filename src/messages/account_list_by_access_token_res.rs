use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CtidTraderAccount {
    pub ctid_trader_account_id: u64,
    pub is_live: bool,
    pub trader_login: u64,
    pub last_closing_deal_timestamp: u64,
    pub last_balance_update_timestamp: u64,
    pub broker_title_short: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountListByAccessTokenResultPayload {
    pub access_token: String,
    pub permission_scope: u64,
    pub ctid_trader_account: Vec<CtidTraderAccount>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountListByAccessTokenRes {
    pub client_msg_id: String,
    pub payload_type: u64,
    pub payload: AccountListByAccessTokenResultPayload,
}
