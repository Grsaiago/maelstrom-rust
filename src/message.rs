use serde::Deserialize;
use serde_json::Value;
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: MessageBody,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBody {
    pub r#type: String,
    pub msg_id: Option<isize>,
    pub in_reply_to: Option<isize>,
    #[serde(flatten)]
    pub payload: Value,
}
