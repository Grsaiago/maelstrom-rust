use crate::{Message, Node};

pub type HandlerFunc<R, W> = dyn Fn(Message, &Node<R, W>) + Send + Sync;
