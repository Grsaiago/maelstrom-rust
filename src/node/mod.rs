mod message_router;

use crate::Message;
use message_router::MessageRouter;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    sync::atomic::AtomicI32,
};
use tokio::{
    io::{Stdin, Stdout},
    task,
};

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
    pub fn handle<P>(&mut self, handler: impl Fn(Message<P>) + 'static)
    where
        P: Clone + Debug + for<'a> serde::Deserialize<'a> + 'static,
    {
        self.message_router.route(TypeId::of::<P>(), handler);
    }

    pub async fn run(&self) {
        task::spawn(self.listen);
    }

    async fn listen(&self) {
        let stdin = std::io::stdin();

        let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<_>>();

        for input in inputs {
            let input = input.expect("Maelstrom input from STDIN couldn't be deserialized");
        }
    }

    //async fn serve(&self) {
    //    self.
    //}

    // WARN:essa é interna, está como pub só pra teste
    pub fn call<P>(&mut self, message: Message<P>)
    where
        P: Clone + Debug + for<'a> serde::Deserialize<'a> + 'static,
    {
        let message_type = message.body.payload.type_id();
        match &self.message_router.router {
            None => {
                println!("call com o map interno sendo vazio");
            }
            Some(map) => {
                if let Some(func) = map.get(&message_type) {
                    func(Box::new(message))
                }
            }
        }
    }
}
