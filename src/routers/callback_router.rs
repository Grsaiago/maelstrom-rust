use crate::{routers::common::HandlerFunc, Message, Node};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type CallbackMap = HashMap<isize, Arc<HandlerFunc>>;

pub struct CallbackRouter {
    pub router: Arc<RwLock<Option<CallbackMap>>>,
}

impl Default for CallbackRouter {
    fn default() -> Self {
        CallbackRouter::new()
    }
}

impl CallbackRouter {
    pub fn new() -> CallbackRouter {
        CallbackRouter {
            router: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_capacity(size: usize) -> CallbackRouter {
        CallbackRouter {
            router: Arc::new(RwLock::new(Some(CallbackMap::with_capacity(size)))),
        }
    }

    pub fn register_callback<C>(&self, message_id: isize, callback: C)
    where
        C: Fn(Message, &Node) + Send + Sync + 'static,
    {
        let arced_handler: Arc<HandlerFunc> = Arc::new(callback);
        let _ = self
            .router
            .write()
            .expect("error on write lock ofcallback router")
            .get_or_insert_with(CallbackMap::default)
            .insert(message_id, arced_handler);
    }

    pub fn get(&mut self, message_id: isize) -> Option<Arc<HandlerFunc>> {
        self.router.write().ok()?.as_mut()?.remove(&message_id)
    }
}
