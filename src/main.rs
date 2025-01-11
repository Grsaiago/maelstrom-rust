mod message;
mod runtime;
mod workloads;

use message::Message;
use runtime::Node;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut node: Node = Node::new();

    node.handle("echo", |message, node| {
        let body = message
            .body
            .payload
            .get("echo")
            .expect("no echo key")
            .clone();

        node.reply(message, json!({"echo": body}));
    });

    node.run().await;
    Ok(())
}
