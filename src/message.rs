use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageBody {
    #[serde(rename = "type")]
    pub ty: String,
    pub msg_id: Option<isize>,
    pub in_reply_to: Option<isize>,

    #[serde(flatten)]
    pub payload: Value,
}
