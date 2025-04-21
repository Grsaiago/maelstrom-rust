# maelstrom-rust

A Rust abstraction layer to interact with Malestrom, heavily inspired
by the [Go implementation](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go).


- [x] Implement a way to Extract the workloads to the message just like the
[Go lib does](https://pkg.go.dev/github.com/jepsen-io/maelstrom/demo/go#Message).
- [ ] Implement an Error type that implement the std::Error trait.
- [x] Implement the Error codes enum.
- [x] Come up with a Message type, with src, dst and a body as a serde::Value.
- [x] Make Node sync/send and implement interior mutability so it can be passed
as a parameter to the handler functions
- [x] A way of answering based on a message (see go's node.Reply()). Test pending
- [x] Make the MessageRouter Type be sync to I can use tokio::join! on two
tokio::spawn handles instead of tokio::join! on both methods in Node.run().
- [ ] Revisit the CallbackRouter and see if we really need an Arc or if we can use a Box for the callbacks
- [ ] Revisit pub definitions for struct fields and methods to check if they really need to be public
