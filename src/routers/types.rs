use crate::{Message, Node};

pub trait HandlerFunc<R, W>: Fn(Message, &Node<R, W>) + Send + Sync + 'static {}
impl<T, R, W> HandlerFunc<R, W> for T where T: Fn(Message, &Node<R, W>) + Send + Sync + 'static {}
