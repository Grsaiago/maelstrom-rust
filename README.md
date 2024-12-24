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

Implement the following

- [ ] Extract the workloads to the message just like the [Go lib does](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go#Message)
- [ ] An Error type that implement the std::Error trait
- [x] Error codes enum
