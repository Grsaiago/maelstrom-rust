use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A workload for a linearizable key-value store.
///
/// Reads the current value of a single key.
/// Clients send a read request with the key they'd like to observe, and expect a response with the current value of that key.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    /// This will always have the value "read"
    r#type: String,
    key: Value,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResponse {
    /// This will always have the value "read_ok"
    r#type: String,
    value: Value,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}

/// Blindly overwrites the value of a key.
/// Creates keys if they do not presently exist.
/// Servers should respond with a write_ok response once the write is complete.
#[derive(Debug, Serialize, Deserialize)]
pub struct WriteRequest {
    /// This will always have the value "write"
    r#type: String,
    key: Value,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WriteResponse {
    /// This will always have the value "write_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}

/// Atomically compare-and-sets a single key: if the value of key is currently from, sets it to to.
/// Returns error 20 if the key doesn't exist, and 22 if the from value doesn't match.
pub struct CasRequest {
    /// This will always have the value "cas"
    r#type: String,
    key: Value,
    from: Value,
    to: Value,
    msg_id: i32,
}

pub struct CasResponse {
    /// This will always have the value "cas_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}
