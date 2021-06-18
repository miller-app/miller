//! This module contains graph-related stuff.

use std::ffi::CString;
use std::marker::PhantomData;

use anyhow::Error as Anyhow;
use zengarden_raw::{
    zg_context_new_empty_graph, zg_context_new_graph_from_string, zg_graph_add_connection,
    zg_graph_add_new_object, zg_graph_attach, zg_graph_delete, zg_graph_get_dollar_zero,
    zg_graph_get_objects, zg_graph_remove_connection, zg_graph_unattach, ZGGraph,
};

use crate::context::{AudioLoop, Context, Dispatcher};
use crate::object::{ConnectionPair, Object, ObjectPosition};

/// A graph is a collection of objects and the connections between them. A [Graph] is a subclass of
/// [object::Object], and thus [Graph]s can contain other [Graph]s (such as abstraction or
/// subgraphs). However, this does not mean that [Graph]s and [object::Object]s are
/// interchangeable in the API. Specific functions are made available for each.
#[derive(Debug)]
pub struct Graph<'a>(*mut ZGGraph, PhantomData<&'a i64>);

impl<'a> Graph<'a> {
    /// Initialize a new empty graph.
    pub fn new_empty<D: Dispatcher, L: AudioLoop>(context: &'a Context<D, L>) -> Self {
        unsafe {
            let raw_ptr = zg_context_new_empty_graph(*context.raw_context.read().unwrap());
            Self(raw_ptr, Default::default())
        }
    }

    /// Initialize a graph from a Pd file.
    pub fn from_file<D: Dispatcher, L: AudioLoop>(
        context: Context<D, L>,
        file: &str,
    ) -> Result<Self, Anyhow> {
        unsafe {
            let contents = std::fs::read_to_string(file)?;
            let contents = CString::new(contents)?;
            let raw_ptr = zg_context_new_graph_from_string(
                *context.raw_context.read().unwrap(),
                contents.as_ptr(),
            );

            Ok(Self(raw_ptr, Default::default()))
        }
    }

    /// Initialize a graph from a Pd file content.
    pub fn from_str<D: Dispatcher, L: AudioLoop>(context: Context<D, L>, string: &str) -> Self {
        unsafe {
            let contents = CString::new(string).expect("Can't build CString from netlist");
            let raw_ptr = zg_context_new_graph_from_string(
                *context.raw_context.read().unwrap(),
                contents.as_ptr(),
            );

            Self(raw_ptr, Default::default())
        }
    }

    /// Create a new object with a string, e.g. **"osc~ 440"**, **"+"**, or **"pack t t s"**, and
    /// add it to the graph. If the graph is currently attached then audio may be interrupted while
    /// the object is attached and graph reconfigured (if necessary). If the graph is unattached
    /// then no audio interruption will take place, even if reconfiguration takes place.
    ///
    /// The [object::ObjectPosition] is only relevant for input/~ and output/~ objects, otherwise
    /// `None` may be specified.
    pub fn add_object(&self, object: &str, position: Option<ObjectPosition>) -> Object {
        unsafe {
            let object =
                CString::new(object).expect(&format!("Can't build CString from {}", object));
            let (x, y) = if let Some(pos) = position {
                (pos.x, pos.y)
            } else {
                (0.0, 0.0)
            };

            zg_graph_add_new_object(self.0, object.as_ptr(), x, y).into()
        }
    }

    /// Add a connection between two objects, both of which are in the given graph. The new
    /// connection may cause the object graph to be reordered and cause audio dropouts. If the
    /// arguments do not define a valid connection, then this function does nothing.
    pub fn add_connection(&self, from: ConnectionPair, to: ConnectionPair) {
        unsafe {
            zg_graph_add_connection(
                self.0,
                from.object.0,
                from.index as i32,
                to.object.0,
                to.index as i32,
            );
        }
    }

    /// Remove a connection between two objects, both of which are in the given graph. If the
    /// arguments do not define a valid connection, then this function does nothing.
    pub fn remove_connection(&self, from: ConnectionPair, to: ConnectionPair) {
        unsafe {
            zg_graph_remove_connection(
                self.0,
                from.object.0,
                from.index as i32,
                to.object.0,
                to.index as i32,
            );
        }
    }

    /// Returns all objects in this graph.
    pub fn objects(&self) -> Vec<Object> {
        unsafe {
            let mut n = 0;
            let objects = zg_graph_get_objects(self.0, &mut n);
            let objects = std::slice::from_raw_parts(objects, n as usize);
            objects.into_iter().copied().map(From::from).collect()
        }
    }

    /// Returns the $0 argument to a graph, allowing graph-specific receivers to be addressed.
    pub fn dollar_zero(&self) -> usize {
        unsafe { zg_graph_get_dollar_zero(self.0) as usize }
    }

    /// Attaches a graph to its context.
    pub fn attach(&self) {
        unsafe { zg_graph_attach(self.0) }
    }

    /// Detaches a graph from its context.
    pub fn detach(&self) {
        unsafe { zg_graph_unattach(self.0) }
    }
}

impl<'a> Drop for Graph<'a> {
    fn drop(&mut self) {
        unsafe {
            self.detach();
            zg_graph_delete(self.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{AudioLoopF32, Config};
    use crate::message::{Message, MessageElement};

    use super::*;

    #[test]
    fn new_empty() {
        let context = init_test_context();
        let _ = Graph::new_empty(&context);
    }

    #[test]
    fn from_file() {
        let context = init_test_context();
        let _ = Graph::from_file(context, "test/send_message.pd").unwrap();
    }

    #[test]
    fn from_string() {
        let context = init_test_context();
        let contents = std::fs::read_to_string("test/send_message.pd").unwrap();
        let _ = Graph::from_str(context, &contents);
    }

    #[test]
    fn add_object() {
        let context = init_test_context();
        let graph = Graph::new_empty(&context);
        graph.attach();

        let obj_str = "osc~ 440";
        let object = graph.add_object(obj_str, Some((10.0, 20.0).into()));

        assert_eq!(object.to_string(), obj_str.to_string());
        assert_eq!(object.position(), ObjectPosition::from((10.0, 20.0)));
    }

    #[test]
    fn connection() {
        let context = init_test_context();
        let receiver_name = "connection-test-r";
        context.register_receiver(receiver_name);
        let graph = Graph::new_empty(&context);
        let receiver = graph.add_object("receive outer-receive", None);
        let sender = graph.add_object(&format!("send {}", receiver_name), None);
        graph.attach();

        // Add connection
        send_message_and_process_block(&context);
        assert_eq!(*context.user_data(), 0);

        graph.add_connection((receiver, 0).into(), (sender, 0).into());
        send_message_and_process_block(&context);
        assert_eq!(*context.user_data(), 42);

        // Remove connection
        graph.remove_connection((receiver, 0).into(), (sender, 0).into());
        *context.user_data_mut() = 0;
        send_message_and_process_block(&context);
        assert_eq!(*context.user_data(), 0);
    }

    #[test]
    fn objects() {
        let context = init_test_context();
        let graph = Graph::new_empty(&context);
        let osc = graph.add_object("osc~ 440", None);
        let dac = graph.add_object("dac~", None);
        let expected = vec![osc, dac];
        assert_eq!(graph.objects(), expected);
    }

    #[test]
    fn dollar_zero() {
        let context = init_test_context();
        for n in 1..10 {
            let graph = Graph::new_empty(&context);
            assert_eq!(graph.dollar_zero(), n);
        }
    }

    fn init_test_context() -> Context<TestDispatcher, AudioLoopF32> {
        Context::<TestDispatcher, AudioLoopF32>::new(Config::default()).unwrap()
    }

    fn send_message_and_process_block(context: &Context<TestDispatcher, AudioLoopF32>) {
        context.send_message(
            "outer-receive",
            Message::builder()
                .with_element(MessageElement::Bang)
                .build(),
        );

        for _ in 0..context.config().blocksize + 1 {
            context.next_frame(&[0.0, 0.0]).unwrap();
        }
    }

    #[derive(Debug, Clone)]
    struct TestDispatcher;

    impl Dispatcher for TestDispatcher {
        type UserData = u64;

        fn receiver_message(_name: String, _message: Option<Message>, data: &mut Self::UserData) {
            *data = 42;
        }
    }
}
