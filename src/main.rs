mod message_types;
mod runtime;

use message_types::*;
use runtime::Node;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut node = Node::new();
    let incoming_message = Message {
        src: "eu".to_string(),
        dst: "Voce".to_string(),
        body: MessageBody {
            r#type: "uma coisa".to_string(),
            msg_id: None,
            in_reply_to: None,
            payload: Payload::Generate(GeneratePayload { msg_id: 10 }),
        },
    };

    node.handle::<EchoPayload>(|message| println!("{:?}", message));
    node.handle::<GeneratePayload>(|_message| println!("Voce recebeu um generate"));

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