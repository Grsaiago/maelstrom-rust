use std::{any::TypeId, collections::HashMap};

use crate::Message;

pub type TypeFuncMap = HashMap<TypeId, Box<dyn Fn(Message)>>;

pub struct MessageRouter {
    pub router: Option<Box<TypeFuncMap>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        MessageRouter { router: None }
    }

    pub fn route(&mut self, type_id: TypeId, handler: impl Fn(Message) + 'static) {
        let boxed_handler = Box::new(handler);

        let _ = self
            .router
            .get_or_insert_with(Box::default)
            .insert(type_id, boxed_handler);
    }
}
