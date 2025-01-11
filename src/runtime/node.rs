use serde::Serialize;
use serde_json::json;
use tokio::io::{self, AsyncBufReadExt};
use tokio::{
    io::BufReader,
    sync::mpsc::{self, Receiver, Sender},
};

use crate::message::MessageBody;
use crate::runtime::message_router::MessageRouter;
use crate::workloads::init::InitRequest;
use crate::Message;
use std::io::Write;
use std::sync::atomic::AtomicIsize;
use std::sync::RwLock;

/// The Node struct is this lib's foundation. It helps you to avoid a lot of boilerplate, as well
/// as it exposes the methods you'll use to build your own maelstrom sollutions
pub struct Node {
    /// Your node's id. this will be initialized whenever your Node gets an Init message
    pub id: RwLock<Option<String>>,

    /// The id of all vectors in your 'cluster'
    pub node_ids: RwLock<Option<Vec<String>>>,

    /// Your internal message counter. This is your node's Lamport clock
    pub next_message_id: AtomicIsize,

    pub message_router: MessageRouter,

    pub message_sender: Option<Sender<Message>>, // pub callbacks: todo!(),
}

impl Node {
    pub fn new() -> Self {
        let mut node = Node {
            id: RwLock::new(None),
            node_ids: RwLock::new(None),
            next_message_id: AtomicIsize::new(1),
            message_router: MessageRouter::new(),
            message_sender: None,
        };
        node.handle("init", Self::init_handler);
        node
    }

    pub fn get_id(&self) -> Option<String> {
        self.id.read().expect("Error on node_id read lock").clone()
    }

    pub fn set_id(&self, new_id: Option<String>) {
        let mut lock = self.id.write().expect("Error on node_id write lock");
        *lock = new_id;
    }

    pub fn get_ids(&self) -> Option<Vec<String>> {
        self.node_ids
            .read()
            .expect("Error on get_node_ids read lock")
            .clone()
    }

    pub fn set_ids(&self, new_ids: Option<Vec<String>>) {
        let mut lock = self
            .node_ids
            .write()
            .expect("Error on get_node_ids read lock");
        *lock = new_ids;
    }

    fn init_handler(message: Message, node: &Node) {
        let body = match serde_json::from_value::<InitRequest>(message.body.payload.clone()) {
            Ok(body) => body,
            Err(err) => panic!("{err:?}"),
        };
        node.set_id(Some(body.node_id));
        node.set_ids(Some(body.node_ids));
        #[cfg(debug_assertions)]
        println!(
            "Node info!\nNodeId: {:?}\nNodeIds: {:?}",
            node.get_id(),
            node.get_ids()
        );
        node.reply(message, json!({}));
    }

    // You'd call it as node.handle::<EchoPayload>(handler);
    pub fn handle(
        &mut self,
        rpc_type: &str,
        handler: impl Fn(Message, &Node) + Send + Sync + 'static,
    ) {
        self.message_router.route(rpc_type, handler);
    }

    pub async fn listen(sender: Sender<Message>) {
        let mut lines_iterator = BufReader::new(io::stdin()).lines();
        while let Some(line) = lines_iterator.next_line().await.unwrap() {
            let message: Message = match serde_json::from_str(&line) {
                Ok(val) => val,
                Err(_err) => panic!(), //TODO: RETORNO MELHOR DE ERRO
            };
            #[cfg(debug_assertions)]
            println!(
                "A mensagem recebida foi: {}",
                serde_json::to_string_pretty(&message).expect("Error desserializing it")
            );
            let _ = sender.send(message).await;
        }
    }

    pub async fn serve(node: Node, mut receiver: Receiver<Message>) {
        while let Some(message) = receiver.recv().await {
            if let Some(handler) = node.message_router.get(&message.body.ty) {
                handler(message, &node);
            }
        }
    }

    pub async fn run(mut self) {
        let (message_sender, message_receiver) = mpsc::channel::<Message>(50);
        self.message_sender = Some(message_sender.clone());

        let listen_handle = tokio::task::spawn(async move {
            Node::listen(message_sender.clone()).await;
        });
        let serve_handle = tokio::task::spawn(async move {
            Node::serve(self, message_receiver).await;
        });
        let _ = tokio::join!(listen_handle, serve_handle);
    }

    pub fn reply(&self, message: Message, reply: impl Serialize) {
        let Some(src) = self.get_id() else {
            panic!("self.id value not yet initialized in call to reply")
        };

        let message_reply = Message {
            src,
            dest: message.src.clone(),
            body: MessageBody {
                ty: format!("{}_ok", &message.body.ty),
                msg_id: Some(
                    self.next_message_id
                        .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
                ),
                in_reply_to: message.body.msg_id,
                payload: serde_json::to_value(reply)
                    .expect("error converting reply into payload in Node::reply"),
            },
        };

        match serde_json::to_string(&message_reply) {
            Ok(reply_string) => {
                let mut stdout_lock = std::io::stdout().lock();
                if let Err(err) = writeln!(stdout_lock, "{}", reply_string) {
                    panic!("error on writting to stdout in reply method: {err:?}");
                }
            }
            Err(err) => {
                panic!("error on serializing message_reply: {err:?}");
            }
        }
    }
}
