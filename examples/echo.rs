use maelstrom_rust::Node;
use serde_json::json;
use std::error::Error;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut node = Node::default().with_log();

    node.handle("echo", |message, node| {
        let body = message
            .body
            .payload
            .get("echo")
            .expect("no echo key")
            .clone();

        info!("Hello from withing the function I wrote myself");

        node.reply(message, json!({"echo": body}));
    });

    node.run().await;
    Ok(())
}
