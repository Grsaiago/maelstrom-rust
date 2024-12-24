mod message_types;
mod node;
mod workloads;

use message_types::*;
use node::Node;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut node: Node = Node::new();
    let incoming_message = Message {
        src: "eu".to_string(),
        dst: "Voce".to_string(),
        body: MessageBody::<EchoRPC> {
            r#type: "uma coisa".to_string(),
            msg_id: None,
            in_reply_to: None,
            payload: EchoRPC {
                echo: "oie".to_string(),
            },
        },
    };

    node.handle::<EchoRPC>(|message| println!("{:?}", message));

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
