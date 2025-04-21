//! This crate provides a high-level Rust interface for interacting with
//! [Maelstrom](https://github.com/jepsen-io/maelstrom), a distributed systems
//! testing framework.
//!
//! Inspired by Fly.io's [Golang counterpart](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go),
//! maelstrom-rust simplifies writting solutions to maelstrom exercises by
//! abstracting away message creation, serialization, response sending,
//! and node lifecycle management.
//!
/// Defines all things regarding Messages
mod message;
pub use message::{Message, MessageBody};

/// Defines and exposes the Node type.
mod node;
pub use node::Node;

mod routers;
mod workloads;
