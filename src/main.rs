mod message;
mod runtime;
mod workloads;

use message::{Message, MessageBody};
use runtime::Node;
use serde_json::json;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut node: Node = Node::new();
    let incoming_message = Message {
        src: "eu".to_string(),
        dest: "Voce".to_string(),
        body: MessageBody {
            ty: "echo".to_string(),
            msg_id: None,
            in_reply_to: None,
            payload: json!({
                "echo": "oieoie"
            }),
        },
    };

    node.handle("echo", |message| println!("{:?}", message));

    node.call(incoming_message);
    Ok(())
}

//fn main() -> Result<(), Box<dyn Error>> {
//
//    let stdin = std::io::stdin().lock();
//    let stdout = std::io::stdout().lock();
//
//    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
//
//    for input in inputs {
//        let input = input.expect("Maelstrom input from STDIN couldn't be deserialized");
//    }
//
//    Ok(())
//}
