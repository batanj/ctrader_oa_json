use crate::models::payload_type::PayloadType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccountListByAccessTokenReq {
    pub client_msg_id: String,
    pub payload_type: u64,
    pub payload: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Payload {
    pub access_token: String,
}

impl AccountListByAccessTokenReq {
    pub fn new(cm_id: u64, access_token: String) -> Self {
        AccountListByAccessTokenReq {
            client_msg_id: format!("cm_id_{}", cm_id),
            payload_type: PayloadType::AccountListByAccessTokenReq as u64,
            payload: Payload { access_token },
        }
    }
}
