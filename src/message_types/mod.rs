use std::any::TypeId;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: MessageBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBody {
    pub r#type: String,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo(EchoPayload),
    Generate(GeneratePayload),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EchoPayload {
    pub echo: String,
}

// we have to implement MessageType for all invariants to be able to pass any
// message into the 'handle' method of the Node struct
impl MessageType for EchoPayload {
    fn as_type_id(&self) -> std::any::TypeId {
        TypeId::of::<EchoPayload>()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeneratePayload {
    pub msg_id: u32,
}

// we have to implement MessageType for all invariants to be able to pass any
// message into the 'handle' method of the Node struct
impl MessageType for GeneratePayload {
    fn as_type_id(&self) -> std::any::TypeId {
        TypeId::of::<GeneratePayload>()
    }
}

// When type_id() is called on an enum, it always returns the same value,
// regardless of the enum's current invariant.
// We need this trait in order to extract the type_id for each enum invariant at runtime
pub trait MessageType {
    fn as_type_id(&self) -> std::any::TypeId;
}

impl MessageType for Payload {
    // When type_id() is called on an enum, it always returns the same value,
    // regardless of the enum's current invariant.
    // We need this trait in order to extract the type_id for each enum invariant at runtime
    fn as_type_id(&self) -> std::any::TypeId {
        match *self {
            Self::Echo(_) => TypeId::of::<EchoPayload>(),
            Self::Generate(_) => TypeId::of::<GeneratePayload>(),
        }
    }
}
