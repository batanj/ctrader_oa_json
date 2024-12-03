use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthResPayload {
    pub ctid_trader_account_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthRes {
    pub client_msg_id: String,
    pub payload: AccountAuthResPayload,
    pub payload_type: u64,
}
