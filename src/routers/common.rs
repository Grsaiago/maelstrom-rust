use crate::{Message, Node};

pub type HandlerFunc = dyn Fn(Message, &Node) + Send + Sync;
