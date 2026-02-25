use crate::{Message, Node};

pub trait HandlerFunc: Fn(Message, &Node) + Send + Sync + 'static {}
impl<T> HandlerFunc for T where T: Fn(Message, &Node) + Send + Sync + 'static {}
