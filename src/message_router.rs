use super::Node;
use crate::Message;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type HandlerFunc = dyn Fn(Message, &Node) + Send + Sync;

pub type FuncMap = HashMap<String, Arc<HandlerFunc>>;

pub struct MessageRouter {
    pub router: Arc<RwLock<Option<FuncMap>>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        MessageRouter {
            router: Arc::new(RwLock::new(None)),
        }
    }

    pub fn route<F>(&mut self, rpc_type: &str, handler: F)
    where
        F: Fn(Message, &Node) + Send + Sync + 'static,
    {
        // Insert the boxed handler into the router map
        let arced_handler: Arc<HandlerFunc> = Arc::new(handler);
        let _ = self
            .router
            .write()
            .expect("error on write lock of message router route()")
            .get_or_insert_with(FuncMap::default)
            .insert(rpc_type.to_string(), arced_handler);
    }

    pub fn get(&self, key: &str) -> Option<Arc<HandlerFunc>> {
        if let Some(ref map) = *self
            .router
            .read()
            .expect("error on read lock in MessageRouter::get")
        {
            match map.get(key) {
                Some(handler) => Some(handler.clone()),
                None => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        message::{Message, MessageBody},
        node::Node,
    };

    use super::MessageRouter;

    #[test]
    fn can_set_then_get() {
        let mut router = MessageRouter::new();

        router.route("test", |_, _| {});
        assert!(router.get("test").is_some())
    }

    #[test]
    fn can_set_then_call() {
        let mut router = MessageRouter::new();
        let msg: Message = Message {
            src: "n1".to_string(),
            dest: "n2".to_string(),
            body: MessageBody {
                ty: "test".to_string(),
                msg_id: Some(1),
                in_reply_to: None,
                payload: json!({
                    "name": "test_body",
                }),
            },
        };
        let (tx, rx) = std::sync::mpsc::channel::<Message>();
        router.route("test", move |msg, _| {
            let _ = tx.send(msg);
        });

        assert!(router.get(&msg.body.ty).is_some());
        let callback = router.get(&msg.body.ty).unwrap();
        callback(msg, &Node::new());
        let received_message = rx.recv().expect("Error on receiving message");
        assert_eq!(
            received_message
                .body
                .payload
                .get("name")
                .expect("error on get json"),
            "test_body"
        )
    }

    #[test]
    fn cannot_get_unexisting_key() {
        let router = MessageRouter::new();

        assert!(router.get("aaaa").is_none())
    }
}
