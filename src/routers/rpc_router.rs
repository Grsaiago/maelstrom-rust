use crate::routers::HandlerFunc;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio::io::{Stdin, Stdout};

type RpcMap<R, W> = HashMap<String, Arc<dyn HandlerFunc<R, W>>>;

pub struct RpcRouter<R, W> {
    pub router: Arc<RwLock<Option<RpcMap<R, W>>>>,
}

impl Default for RpcRouter<Stdin, Stdout> {
    fn default() -> Self {
        RpcRouter::new()
    }
}

impl<R, W> RpcRouter<R, W>
where
    R: AsyncRead + Send + Sync + 'static,
    W: AsyncWrite + Send + Sync + 'static,
{
    pub fn new() -> RpcRouter<R, W> {
        RpcRouter {
            router: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_capacity(size: usize) -> RpcRouter<R, W> {
        RpcRouter {
            router: Arc::new(RwLock::new(Some(RpcMap::with_capacity(size)))),
        }
    }

    pub fn route<F>(&mut self, rpc_type: &str, handler: F)
    where
        F: HandlerFunc<R, W>,
    {
        // Insert the boxed handler into the router map
        let arced_handler: Arc<dyn HandlerFunc<R, W>> = Arc::new(handler);
        let _ = self
            .router
            .write()
            .expect("error on write lock of message router route()")
            .get_or_insert_with(RpcMap::default)
            .insert(rpc_type.to_string(), arced_handler);
    }

    pub fn get(&self, key: &str) -> Option<Arc<dyn HandlerFunc<R, W>>> {
        Some(self.router.read().ok()?.as_ref()?.get(key)?.clone())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        message::{Message, MessageBody},
        node::Node,
    };

    use super::RpcRouter;

    #[test]
    fn can_set_then_get() {
        let mut router = RpcRouter::default();

        router.route("test", |_, _| {});
        assert!(router.get("test").is_some())
    }

    #[test]
    fn can_set_then_call() {
        let mut router = RpcRouter::new();
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
        callback(msg, &Node::default());
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
        let router = RpcRouter::default();

        assert!(router.get("aaaa").is_none())
    }
}
