#![allow(dead_code)]

use crate::models::payload_type::PayloadType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub ctid_trader_account_id: u64,
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthReq {
    pub client_msg_id: String,
    pub payload_type: u64,
    pub payload: Payload,
}

impl AccountAuthReq {
    pub fn new(cm_id: u64, ctid_trader_account_id: u64, access_token: String) -> Self {
        AccountAuthReq {
            client_msg_id: format!("cm_id_{}", cm_id),
            payload_type: PaylaodType::AccountAuthReq as u64,
            payload: Payload {
                ctid_trader_account_id,
                access_token,
            },
        }
    }
}
