mod message_router;

use message_router::MessageRouter;
use std::{
    any::TypeId,
    io::{StdinLock, StdoutLock},
    sync::atomic::AtomicI32,
};

use crate::{Message, MessageType};

pub struct Node<'a> {
    pub stdin: StdinLock<'a>,
    pub stdout: StdoutLock<'a>,

    pub id: Option<String>,

    pub node_ids: Option<Vec<String>>,

    pub next_message_id: AtomicI32,

    pub message_router: MessageRouter,
    // pub callbacks: todo!(),
}

impl Node<'_> {
    pub fn new() -> Self {
        Node {
            stdin: std::io::stdin().lock(),
            stdout: std::io::stdout().lock(),
            id: None,
            node_ids: None,
            next_message_id: AtomicI32::new(0),
            message_router: MessageRouter::new(),
        }
    }

    // You'd call it as node.handle::<EchoPayload>(handler);
    pub fn handle<T>(&mut self, handler: impl Fn(Message) + 'static)
    where
        T: crate::message_types::MessageType + 'static,
    {
        self.message_router.route(TypeId::of::<T>(), handler);
    }

    // WARN:essa é interna, está como pub só pra teste
    pub fn call(&mut self, message: Message) {
        let message_type = message.body.payload.as_type_id();
        match &self.message_router.router {
            None => {
                println!("call com o map interno sendo vazio");
            }
            Some(map) => {
                if let Some(func) = map.get(&message_type) {
                    func(message)
                }
            }
        }
    }
}
