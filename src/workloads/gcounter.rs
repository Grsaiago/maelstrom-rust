use serde::{Deserialize, Serialize};

///An eventually-consistent grow-only counter, which supports increments.
///Validates that the final read on each node has a value which is the sum of all known (or possible) increments.

/// Adds a non-negative integer, called delta, to the counter.
/// Servers should respond with an add_ok message.
#[derive(Debug, Serialize, Deserialize)]
pub struct AddRequest {
    /// This will always have the value "add"
    r#type: String,
    delta: i32,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResponse {
    /// This will always have the value "add_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}

/// Reads the current value of the counter.
/// Servers respond with a read_ok message containing a value,
/// which should be the sum of all (known) added deltas.
#[derive(Debug, Serialize, Deserialize)]
pub struct ReadRequest {
    /// This will always have the value "read"
    r#type: String,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadResponse {
    /// This will always have the value "read_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}
