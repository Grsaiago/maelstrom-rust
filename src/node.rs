use crate::Message;
use crate::message::MessageBody;
use crate::message_router::MessageRouter;
use crate::workloads::init::InitRequest;
use serde::Serialize;
use serde_json::json;
use std::{
    io::Write,
    sync::atomic::AtomicIsize,
    sync::{Arc, RwLock},
};
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
};
use tracing::{Instrument, Level, info, info_span};

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

    pub message_sender: Option<UnboundedSender<Message>>, // pub callbacks: todo!(),
}

impl Node {
    pub fn new() -> Node {
        let mut node = Node {
            id: RwLock::new(None),
            node_ids: RwLock::new(None),
            next_message_id: AtomicIsize::new(1),
            message_router: MessageRouter::new(),
            message_sender: None,
        };
        node.handle("init", Node::init_handler);
        node
    }

    pub fn with_log(self) -> Node {
        tracing_subscriber::fmt()
            .compact()
            .with_max_level(Level::INFO)
            .with_ansi(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_writer(std::io::stderr)
            .init();
        self
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
    pub fn handle<F>(&mut self, rpc_type: &str, handler: F)
    where
        F: Fn(Message, &Node) + Send + Sync + 'static,
    {
        self.message_router.route(rpc_type, handler);
    }

    pub async fn listen(sender: UnboundedSender<Message>) {
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
            let _ = sender.send(message);
        }
    }

    pub async fn serve(node: Node, mut receiver: UnboundedReceiver<Message>) {
        let node = Arc::new(node);
        while let Some(message) = receiver.recv().await {
            let shared_node = node.clone();
            if let Some(handler) = shared_node.clone().message_router.get(&message.body.ty) {
                let handler_span = info_span!("message_handler",
                    node = %node.get_id().unwrap_or("nil".to_string()),
                    msg.id = %message.body.msg_id.unwrap_or(0)
                );
                tokio::spawn(
                    async move {
                        info!(msg.type = message.body.ty, "calling handler");
                        handler(message, &shared_node);
                    }
                    .instrument(handler_span),
                );
            }
        }
    }

    pub async fn run(mut self) {
        let (message_sender, message_receiver) = mpsc::unbounded_channel::<Message>();
        self.message_sender = Some(message_sender.clone());

        let listen_handle = tokio::task::spawn(async move {
            Node::listen(message_sender.clone()).await;
        });
        let serve_handle = tokio::task::spawn(async move {
            Node::serve(self, message_receiver).await;
        });
        let _ = tokio::join!(listen_handle, serve_handle);
    }

    pub fn reply<S>(&self, message: Message, reply: S)
    where
        S: Serialize,
    {
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
