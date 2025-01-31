mod message;
mod runtime;
mod workloads;

use message::Message;
use runtime::Node;
use serde_json::json;
use std::error::Error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut node: Node = Node::new().with_log();

    node.handle("echo", |message, node| {
        let body = message
            .body
            .payload
            .get("echo")
            .expect("no echo key")
            .clone();

        info!("Oie de dentro da função handler que eu, o user, escrevi");

        node.reply(message, json!({"echo": body}));
    });

    node.run().await;
    Ok(())
}
