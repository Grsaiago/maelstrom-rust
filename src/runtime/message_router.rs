use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::Message;

use super::Node;

pub type TypeFuncMap = HashMap<String, Arc<dyn Fn(Message, &Node) + Send + Sync>>;

pub struct MessageRouter {
    pub router: Arc<RwLock<Option<TypeFuncMap>>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        MessageRouter {
            router: Arc::new(RwLock::new(None)),
        }
    }

    pub fn route(
        &mut self,
        rpc_type: &str,
        handler: impl Fn(Message, &Node) + Send + Sync + 'static,
    ) {
        // box na função castando o input pra any
        let arced_handler = Arc::new(handler);
        // Insert the boxed handler into the router map
        let _ = self
            .router
            .write()
            .expect("error on write lock of message router route()")
            .get_or_insert_with(TypeFuncMap::default)
            .insert(rpc_type.to_string(), arced_handler);
    }

    pub fn get(&self, key: &str) -> Option<Arc<dyn Fn(Message, &Node) + Send + Sync>> {
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
