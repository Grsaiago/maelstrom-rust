use std::{any::TypeId, collections::HashMap, fmt::Debug};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Message<T>
where
    T: Debug + Clone, // the T has to impl Deserialize too
{
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: MessageBody<T>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBody<T>
where
    T: Debug + Clone, // the T has to impl Deserialize too
{
    pub r#type: String,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: T,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EchoWorkload {
    Echo(HashMap<String, String>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct EchoRPC {
    pub echo: String,
}
