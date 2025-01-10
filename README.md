# maelstrom-rust

This repository serves two main purposes

1. Store my sollutions for the [fly.io gossip gloomers challenge](https://fly.io/dist-sys/)
2. Implement a library for interacting with maelstrom in Rust, heavilly inspired
by the [Go implementation](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go) of said library

## Roadmap

Define the types for the following workloads:

- [x] Broadcast
- [x] Echo
- [x] G-counter
- [x] G-set
- [ ] Kafka
- [x] Lin-kv
- [ ] Pn-counter
- [ ] Txn-list-append
- [ ] Txn-rw-register
- [ ] Unique-ids

TODOS:

- [x] Implement a way to Extract the workloads to the message just like the
[Go lib does](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go#Message).
- [ ] Implement an Error type that implement the std::Error trait.
- [x] Implement the Error codes enum.
- [ ] Read the golang's implementation of Node.Reply and Node.Send
and [this maelstrom link](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md#messages)
to understand a bit better how to implement the runtime.
Remember that you're trying to use the typesystem.
- [x] Come up with a Message type, with src, dst and a body as a serde::Value.
- [x] Make Node sync/send and implement interior mutability so it can be passed
as a parameter to the handler functions
- [ ] A way of answering based on a message (see go's node.Reply())
