use crate::runtime::message_router::MessageRouter;
use crate::Message;
use std::sync::atomic::AtomicI32;
use tokio::io::{Stdin, Stdout};

/// The Node struct is this lib's foundation. It helps you to avoid a lot of boilerplate, as well
/// as it exposes the methods you'll use to build your own maelstrom sollutions
pub struct Node {
    pub stdin: Stdin,
    pub stdout: Stdout,

    /// Your node's id. this will be initialized whenever your Node gets an Init message
    pub id: Option<String>,

    /// The id of all vectors in your 'cluster'
    pub node_ids: Option<Vec<String>>,

    /// Your internal message counter. This is your node's Lamport clock
    pub next_message_id: AtomicI32,

    pub message_router: MessageRouter,
    // pub callbacks: todo!(),
}

impl Node {
    pub fn new() -> Self {
        Node {
            stdin: tokio::io::stdin(),
            stdout: tokio::io::stdout(),
            id: None,
            node_ids: None,
            next_message_id: AtomicI32::new(0),
            message_router: MessageRouter::new(),
        }
    }

    // You'd call it as node.handle::<EchoPayload>(handler);
    pub fn handle(&mut self, rpc_type: &str, handler: impl Fn(Message) + 'static) {
        self.message_router.route(rpc_type, handler);
    }

    //async fn serve(&self) {
    //    self.
    //}

    // WARN:essa é interna, está como pub só pra teste
    pub fn call(&mut self, message: Message) {
        match &self.message_router.router {
            None => {
                println!("call com o map interno sendo vazio");
            }
            Some(map) => {
                if let Some(func) = map.get(&message.body.r#type) {
                    func(message)
                }
            }
        }
    }
}
