mod message_router;

use crate::Message;
use message_router::MessageRouter;
use std::{
    any::{Any, TypeId},
    fmt::Debug,
    io::{StdinLock, StdoutLock},
    marker::PhantomData,
    sync::atomic::AtomicI32,
};

pub trait Workload {
    fn dispatch(&self) -> TypeId;
}

pub struct Node<'a, W> {
    pub stdin: StdinLock<'a>,
    pub stdout: StdoutLock<'a>,

    pub id: Option<String>,

    pub node_ids: Option<Vec<String>>,

    pub next_message_id: AtomicI32,

    pub message_router: MessageRouter,

    _workload_type: PhantomData<W>, // caralho eu to usando phantom data vaitomanocu
                                    // pub callbacks: todo!(),
}

impl<W> Node<'_, W>
where
    W: Workload,
{
    pub fn new() -> Self {
        Node {
            stdin: std::io::stdin().lock(),
            stdout: std::io::stdout().lock(),
            id: None,
            node_ids: None,
            next_message_id: AtomicI32::new(0),
            message_router: MessageRouter::new(),
            _workload_type: PhantomData,
        }
    }

    // You'd call it as node.handle::<EchoPayload>(handler);
    pub fn handle<P>(&mut self, handler: impl Fn(Message<P>) + 'static)
    where
        P: Clone + Debug + for<'a> serde::Deserialize<'a> + 'static,
    {
        self.message_router.route(TypeId::of::<P>(), handler);
    }

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
