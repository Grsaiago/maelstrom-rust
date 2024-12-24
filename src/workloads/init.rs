use serde::{Deserialize, Serialize};

/// At the start of a test, Maelstrom issues a single init message to each node.
///

#[derive(Debug, Serialize, Deserialize)]
pub struct InitRequest {
    /// This will always have the value "init"
    r#type: String,
    msg_id: i32,
    node_id: String,
    node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResponse {
    /// This will always have the value "init_ok"
    r#type: String,
    in_reply_to: i32,
}
