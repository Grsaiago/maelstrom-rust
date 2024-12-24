use std::{any::TypeId, collections::HashMap, fmt::Debug};

use crate::Message;

pub type TypeFuncMap = HashMap<TypeId, Box<dyn Fn(Box<dyn std::any::Any>)>>;

pub struct MessageRouter {
    pub router: Option<Box<TypeFuncMap>>,
}

impl MessageRouter {
    pub fn new() -> Self {
        MessageRouter { router: None }
    }

    pub fn route<P>(&mut self, type_id: TypeId, handler: impl Fn(Message<P>) + 'static)
    where
        P: Clone + Debug + for<'a> serde::Deserialize<'a> + 'static,
    {
        // box na função castando o input pra any
        let boxed_handler = Box::new(move |msg: Box<dyn std::any::Any>| {
            // tenta dar downcast no valor pra oq vc colocou como handler
            if let Ok(message) = msg.downcast::<Message<P>>() {
                handler(*message);
            } else {
                // se chegou aqui é pq o sistema de tipos falhou pra caralho
                panic!("Type mismatch on router dispatch: expected Message<P>");
            }
        }) as Box<dyn Fn(Box<dyn std::any::Any>)>;

        // Insert the boxed handler into the router map
        let _ = self
            .router
            .get_or_insert_with(Box::default)
            .insert(type_id, boxed_handler);
    }
}
