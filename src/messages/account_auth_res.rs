use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthResPayload {
    pub ctid_trader_account_id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthRes {
    pub client_msg_id: String,
    pub payload_type: u64,
    pub payload: AccountAuthResPayload,
}
