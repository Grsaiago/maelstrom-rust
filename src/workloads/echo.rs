use serde::{Deserialize, Serialize};
use serde_json::Value;
/// A simple echo workload: sends a message, and expects to get that same message back.

/// Clients send echo messages to servers with an echo field containing an arbitrary payload they'd like to have sent back.
/// Servers should respond with echo_ok messages containing that same payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    /// This will always have the value "echo"
    pub echo: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    /// This will always have the value "echo_ok"
    pub r#type: String,
    pub in_reply_to: i32,
    pub echo: Value,
    pub k: Option<i32>,
    pub msg_id: Option<i32>,
}
