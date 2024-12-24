// A broadcast system.
// Essentially a test of eventually-consistent set addition, but also provides an initial topology message to the cluster with a set of neighbors for each node to use.
pub mod broadcast;

/// A simple echo workload: sends a message, and expects to get that same message back.
pub mod echo;

///An eventually-consistent grow-only counter, which supports increments.
///Validates that the final read on each node has a value which is the sum of all known (or possible) increments.
pub mod gcounter;

/// A grow-only set workload: clients add elements to a set, and read the current value of the set.
pub mod gset;

/// A workload for a linearizable key-value store.
pub mod linkv;

/// Defines an enum of maelstrom error types
pub mod errors;

// TODO: Exportar isso
// pub type Result = std::result::Result<Request<T>, MaelstromError>;
