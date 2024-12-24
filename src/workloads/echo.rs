use serde::{Deserialize, Serialize};
use serde_json::Value;
/// A simple echo workload: sends a message, and expects to get that same message back.

/// Clients send echo messages to servers with an echo field containing an arbitrary payload they'd like to have sent back.
/// Servers should respond with echo_ok messages containing that same payload.
#[derive(Debug, Serialize, Deserialize)]
pub struct EchoRequest {
    /// This will always have the value "echo"
    r#type: String,
    echo: Value,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EchoResponse {
    /// This will always have the value "echo_ok"
    r#type: String,
    in_reply_to: i32,
    echo: Value,
    k: Option<i32>,
    msg_id: Option<i32>,
}
