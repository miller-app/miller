//! An audio engine and Pure Data programming environment runtime.
//!
//! Engine consists of four basic object types. These are the context ([context::Context]), graph
//! ([graph::Graph]), object ([object::Object]), and message ([message::Message]). The first three
//! have to do with how the signal graph is organised. The later represents discrete messages which
//! are sent into, processed by, and out of the graph.
//! 
//! A context represents a unique and independent instance of Pure Data. Think of it as Pure Data's
//! console window. A context is defined by its block size, sample rate, and the number of input
//! and output channels. Contexts are entirely independent and messages and objects cannot be
//! exchanged between them.
//! 
//! A graph is a collection of objects and the connections between them. A [graph::Graph] is a
//! subclass of [object::Object], and thus [graph::Graph]s can contain other [graph::Graph]s (such
//! as abstraction or subgraphs).  However, this does not mean that [graph::Graph]s and
//! [object::Object]s are interchangeable in the API.  Specific functions are made available for
//! each.
//! 
//! Messages represent any Pd message, be it a bang or a list of assorted float, symbols, or bangs.
//! Messages are timestamped and contain at least one element, and may otherwise contain any number
//! and any combination of primitives. The engine messages do not distinguish between lists or
//! arrays or singleton elementary types as in Pd. The messages are always lists of typed
//! elementary types.
//!
//! # Graph Attachement
//!
//! Whenever any change in the signal graph takes place in Pd, the audio thread must wait until the
//! reconfiguration is finished. For minor changes such as removing a connection this can be very
//! fast and no audio underrun will occur.  For larger changes, such as adding an object requiring
//! significant initialisation, or many changes at once, such as adding a complex abstraction,
//! audio underruns are almost guaranteed.  
//!
//! The engine solves this problem by allowing a new object or graph to be created on another
//! thread, and then attached to a context at a convenient time.  As the graph has already been
//! instantiated, the attachement process is a relatively quick one and can thus be accomplished
//! without causing any audio dropouts. Graph attachement generally involves registering global
//! senders and receivers and ensuring that existing objects are aware of the new ones. Similarly,
//! a graph can be unattached from a context, leaving it in memory yet inert.

#![deny(
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts
)]
#![warn(
    deprecated_in_future,
    missing_docs,
    unused_import_braces,
    unused_labels,
    unused_lifetimes,
    unused_qualifications,
    unreachable_pub
)]

pub mod context;
pub mod graph;
pub mod message;
pub mod object;
