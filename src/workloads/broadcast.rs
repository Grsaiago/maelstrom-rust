use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// A broadcast system.
// Essentially a test of eventually-consistent set addition, but also provides an initial topology message to the cluster with a set of neighbors for each node to use.

/// A topology message is sent at the start of the test, after initialization, and informs the node of an optional network topology to use for broadcast.
/// The topology consists of a map of node IDs to lists of neighbor node IDs.
#[derive(Debug, Serialize, Deserialize)]
pub struct TopologyRequest {
    /// This will always have the value "topology"
    r#type: String,
    topology: HashMap<String, Vec<String>>,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopologyResponse {
    /// This will always have the value "topology_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}

/// Sends a single message into the broadcast system, and requests that it be broadcast to everyone.
/// Nodes respond with a simple acknowledgement message.
#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastRequest {
    /// This will always have the value "broadcast"
    r#type: String,
    message: Value,
    msg_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BroadcastResponse {
    /// This will always have the value "broadcast_ok"
    r#type: String,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}

/// Requests all messages present on a node.
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
    messages: Vec<Value>,
    k: Option<i32>,
    msg_id: Option<i32>,
    in_reply_to: i32,
}
