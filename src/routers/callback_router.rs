use crate::routers::types::HandlerFunc;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tokio::io::{AsyncRead, AsyncWrite, Stdin, Stdout};

type CallbackMap<R, W> = HashMap<isize, Arc<dyn HandlerFunc<R, W>>>;

pub struct CallbackRouter<R, W> {
    pub router: Arc<RwLock<Option<CallbackMap<R, W>>>>,
}

impl Default for CallbackRouter<Stdin, Stdout> {
    fn default() -> Self {
        CallbackRouter::new()
    }
}

impl<R, W> CallbackRouter<R, W>
where
    R: AsyncRead + Send + Sync + 'static,
    W: AsyncWrite + Send + Sync + 'static,
{
    pub fn new() -> CallbackRouter<R, W> {
        CallbackRouter {
            router: Arc::new(RwLock::new(None)),
        }
    }

    pub fn with_capacity(size: usize) -> CallbackRouter<R, W> {
        CallbackRouter {
            router: Arc::new(RwLock::new(Some(CallbackMap::with_capacity(size)))),
        }
    }

    pub fn register_callback<C>(&self, message_id: isize, callback: C)
    where
        C: HandlerFunc<R, W>,
    {
        let arced_handler: Arc<dyn HandlerFunc<R, W>> = Arc::new(callback);
        let _ = self
            .router
            .write()
            .expect("error on write lock ofcallback router")
            .get_or_insert_with(CallbackMap::default)
            .insert(message_id, arced_handler);
    }

    pub fn get(&mut self, message_id: isize) -> Option<Arc<dyn HandlerFunc<R, W>>> {
        self.router.write().ok()?.as_mut()?.remove(&message_id)
    }
}
