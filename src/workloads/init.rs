use serde::{Deserialize, Serialize};

/// At the start of a test, Maelstrom issues a single init message to each node.
///

#[derive(Debug, Serialize, Deserialize)]
pub struct InitRequest {
    /// This will always have the value "init"
    pub r#type: String,
    pub msg_id: i32,
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InitResponse {
    /// This will always have the value "init_ok"
    pub r#type: String,
    pub in_reply_to: i32,
}
