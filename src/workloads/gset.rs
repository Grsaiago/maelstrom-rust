use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A grow-only set workload: clients add elements to a set, and read the current value of the set.

/// Requests that a server add a single element to the set.
/// Acknowledged by an add_ok message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRequest {
    /// This will always have the value "add"
    pub r#type: String,
    pub element: Value,
    pub msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResponse {
    /// This will always have the value "add_ok"
    pub r#type: String,
    pub k: Option<i32>,
    pub msg_id: Option<i32>,
    pub in_reply_to: i32,
}

/// Requests the current set of all elements.
/// Servers respond with a message containing an elements key, whose value is a JSON array of added elements.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    /// This will always have the value "read"
    pub r#type: String,
    pub msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResponse {
    /// This will always have the value "read_ok"
    pub r#type: String,
    pub value: Vec<Value>,
    pub k: Option<i32>,
    pub msg_id: Option<i32>,
    pub in_reply_to: i32,
}
