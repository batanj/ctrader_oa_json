use crate::models::payload_type::PayloadType;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApplicationAuthReq {
    pub client_msg_id: String,
    pub payload_type: u64,
    pub payload: Payload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub client_id: String,
    pub client_secret: String,
}

impl ApplicationAuthReq {
    pub fn new(cm_id: u64, client_id: String, client_secret: String) -> Self {
        ApplicationAuthReq {
            client_msg_id: format!("cm_id_{}", cm_id),
            payload_type: PayloadType::ApplicationAuthReq as u64,
            payload: Payload {
                client_id,
                client_secret,
            },
        }
    }
}
