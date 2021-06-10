//! This module contains graph-related stuff.

use zengarden_raw::ZGGraph;

use crate::context::{Context, AudioLoop, Dispatcher};
use crate::object::{ConnectionPair, Object, ObjectPosition};

/// A graph is a collection of objects and the connections between them. A [graph::Graph] is a
/// subclass of [object::Object], and thus [graph::Graph]s can contain other [graph::Graph]s (such
/// as abstraction or subgraphs).  However, this does not mean that [graph::Graph]s and
/// [object::Object]s are interchangeable in the API.  Specific functions are made available for
/// each.
#[derive(Debug)]
pub struct Graph {
    raw_ptr: *mut ZGGraph
}

impl Graph {
    /// Initialize a new empty graph.
    pub fn new_empty<D: Dispatcher, L: AudioLoop>(context: &Context<D, L>) -> Self {
        todo!()
    }

    /// Initialize a graph from a Pd file.
    pub fn from_file<D: Dispatcher, L: AudioLoop>(context: &Context<D, L>, file: &str) -> Self {
        todo!()
    }

    /// Initialize a graph from a Pd file content.
    pub fn from_str<D: Dispatcher, L: AudioLoop>(context: &Context<D, L>, string: &str) -> Self {
        todo!()
    }

    /// Create a new object with a string, e.g. “osc~ 440”, “+”, or “pack t t s", and add it to the
    /// graph. If the graph is currently attached then audio may be interrupted while the object is
    /// attached and graph reconfigured (if necessary). If the graph is unattached then no
    /// audio interruption will take place, even if reconfiguration takes place.
    ///
    /// The [object::ObjectPosition] is only relevant for input/~ and output/~ objects, otherwise
    /// `(0.0, 0.0)` may be specified.
    pub fn add_object(&self, object: &str, position: ObjectPosition) {
        todo!()
    }

    /// Add a connection between two objects, both of which are in the given graph. The new
    /// connection may cause the object graph to be reordered and cause audio dropouts. If the
    /// arguments do not define a valid connection, then this function does nothing.
    pub fn add_connection(&self, from: ConnectionPair, to: ConnectionPair) {
        todo!()
    }

    /// Remove a connection between two objects, both of which are in the given graph. If the
    /// arguments do not define a valid connection, then this function does nothing.
    pub fn remove_connection(&self, from: ConnectionPair, to: ConnectionPair) {
        todo!()
    }

    /// Returns all objects in this graph.
    pub fn objects(&self) -> Vec<Object> {
        todo!()
    }

    /// Returns the $0 argument to a graph, allowing graph-specific receivers to be addressed.
    pub fn dollar_zero(&self) -> usize {
        todo!()
    }

    /// Attaches a graph to its context.
    pub fn attach(&self) {
        todo!()
    }

    /// Unattaches a graph from its context.
    pub fn unattach(&self) {
        todo!()
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        todo!();
    }
}
