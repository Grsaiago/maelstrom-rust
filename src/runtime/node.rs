use tokio::io::{self, AsyncBufReadExt};
use tokio::{
    io::BufReader,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::runtime::message_router::MessageRouter;
use crate::Message;
use std::sync::atomic::AtomicI32;

/// The Node struct is this lib's foundation. It helps you to avoid a lot of boilerplate, as well
/// as it exposes the methods you'll use to build your own maelstrom sollutions
pub struct Node {
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

    pub async fn serve(&self, mut receiver: Receiver<Message>) {
        while let Some(message) = receiver.recv().await {
            let _stdout = std::io::stdout().lock();
            if let Some(router) = &self.message_router.router {
                if let Some(handler) = router.get(&message.body.ty) {
                    handler(message);
                }
            }
        }
    }

    pub async fn run(&self) {
        let (message_sender, message_receiver) = mpsc::channel::<Message>(50);
        tokio::join!(self.listen(message_sender), self.serve(message_receiver));
    }

    pub async fn listen(&self, sender: Sender<Message>) {
        let mut lines_iterator = BufReader::new(io::stdin()).lines();
        while let Some(line) = lines_iterator.next_line().await.unwrap() {
            let message: Message = match serde_json::from_str(&line) {
                Ok(val) => val,
                Err(_err) => panic!(), //TODO: RETORNO MELHOR DE ERRO
            };
            let _ = sender.send(message).await;
        }
    }

    // WARN:essa é interna, está como pub só pra teste
    pub fn call(&mut self, message: Message) {
        match &self.message_router.router {
            None => {
                println!("call com o map interno sendo vazio");
            }
            Some(map) => {
                if let Some(func) = map.get(&message.body.ty) {
                    func(message)
                }
            }
        }
    }
}
