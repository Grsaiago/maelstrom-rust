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
