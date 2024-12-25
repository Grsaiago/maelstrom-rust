use std::collections::HashMap;

use crate::Message;

pub type TypeFuncMap = HashMap<String, Box<dyn Fn(Message)>>;

pub struct MessageRouter {
    pub router: Option<Box<TypeFuncMap>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        MessageRouter { router: None }
    }

    pub fn route(&mut self, rpc_type: &str, handler: impl Fn(Message) + 'static) {
        // box na função castando o input pra any
        let boxed_handler = Box::new(handler);
        // Insert the boxed handler into the router map
        let _ = self
            .router
            .get_or_insert_with(Box::default)
            .insert(rpc_type.to_string(), boxed_handler);
    }
}
