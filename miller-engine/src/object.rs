//! To work with objects in a graph use [Object] type.

pub mod external;

use std::ffi::CStr;

use zengarden_raw::{
    zg_object_get_canvas_position, zg_object_get_connection_type,
    zg_object_get_connections_at_inlet, zg_object_get_connections_at_outlet, zg_object_get_label,
    zg_object_get_num_inlets, zg_object_get_num_outlets, zg_object_remove, zg_object_send_message,
    zg_object_set_canvas_position, zg_object_to_string, ZGConnectionPair, ZGConnectionType,
    ZGObject,
};

use crate::message::Message;

/// Represents an object in a [Graph].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Object(pub(crate) *mut ZGObject);

impl Object {
    /// Get position on the canvas.
    pub fn position(&self) -> ObjectPosition {
        unsafe {
            let mut x = 0.0_f32;
            let mut y = 0.0_f32;
            zg_object_get_canvas_position(self.0, &mut x, &mut y);
            ObjectPosition { x, y }
        }
    }

    /// Set position on the canvas.
    pub fn set_position(&self, position: ObjectPosition) {
        unsafe {
            zg_object_set_canvas_position(self.0, position.x, position.y);
        }
    }

    /// Get outlet type.
    pub fn outlet_type(&self, outlet: usize) -> OutletType {
        unsafe { zg_object_get_connection_type(self.0, outlet as u32).into() }
    }

    /// Returns [ConnectionPair]s which indicate the objects and outlets from which the
    /// connections are comming.
    pub fn connections_at_inlet(&self, inlet: usize) -> Vec<ConnectionPair> {
        unsafe {
            let mut size = 0_u32;
            let pairs = zg_object_get_connections_at_inlet(self.0, inlet as u32, &mut size);
            let pairs = std::slice::from_raw_parts(pairs, size as usize);
            pairs.into_iter().copied().map(From::from).collect()
        }
    }

    /// Returns [ConnectionPair]s which indicate the objects and inlets to which this object outlet
    /// is connected.
    pub fn connections_at_outlet(&self, outlet: usize) -> Vec<ConnectionPair> {
        unsafe {
            let mut size = 0_u32;
            let pairs = zg_object_get_connections_at_outlet(self.0, outlet as u32, &mut size);
            let pairs = std::slice::from_raw_parts(pairs, size as usize);
            pairs.into_iter().copied().map(From::from).collect()
        }
    }

    ///	Returns the object label, e.g. “osc~” or “+”.
    pub fn label(&self) -> String {
        unsafe {
            let label = zg_object_get_label(self.0);
            let label = CStr::from_ptr(label);
            label.to_string_lossy().to_string()
        }
    }

    /// Get number of inlets.
    pub fn num_inlets(&self) -> usize {
        unsafe { zg_object_get_num_inlets(self.0) as usize }
    }

    /// Get number of outlets.
    pub fn num_outlets(&self) -> usize {
        unsafe { zg_object_get_num_outlets(self.0) as usize }
    }

    /// Removes the object from the graph and deletes it from memory. Any connections that this
    /// object may have had in the graph are also deleted. The reference to the object after this
    /// function completes is invalid.
    pub fn remove(self) {
        unsafe {
            zg_object_remove(self.0);
        }
    }

    /// Send a message directly to an object. The message will be evaluated at the beginning of the
    /// next block, before any other messages otherwise scheduled are evaluated. The timestamp of
    /// this message is ignored.
    ///
    /// If the message should be delivered at a specific time, use [Context::send_message] to send
    /// the message to a named receiver.
    pub fn send_message(&self, inlet: usize, message: Message) {
        unsafe {
            zg_object_send_message(self.0, inlet as u32, message.into_raw());
        }
    }
}

impl ToString for Object {
    fn to_string(&self) -> String {
        unsafe {
            CStr::from_ptr(zg_object_to_string(self.0))
                .to_string_lossy()
                .to_string()
        }
    }
}

impl From<*mut ZGObject> for Object {
    fn from(raw: *mut ZGObject) -> Self {
        Self(raw)
    }
}

/// Object position on canvas.
///
/// Coordinates are represented as floats and are real valued, though Pd uses only non-negative
/// values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectPosition {
    /// X position.
    pub x: f32,
    /// Y position.
    pub y: f32,
}

impl From<(f32, f32)> for ObjectPosition {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

/// Outlet type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutletType {
    /// Message (control).
    Message,
    /// DSP (audio).
    Dsp,
}

impl From<ZGConnectionType> for OutletType {
    fn from(raw: ZGConnectionType) -> Self {
        match raw {
            ZGConnectionType::ZG_CONNECTION_MESSAGE => Self::Message,
            ZGConnectionType::ZG_CONNECTION_DSP => Self::Dsp,
        }
    }
}

/// Indicates the object and the outlet/inlet index from/to which the connection are comming.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConnectionPair {
    /// Object to/from which connection comes.
    pub object: Object,
    /// Index of the inlet/outlet.
    pub index: usize,
}

impl From<ZGConnectionPair> for ConnectionPair {
    fn from(raw: ZGConnectionPair) -> Self {
        Self {
            object: Object(raw.object),
            index: raw.letIndex as usize,
        }
    }
}

impl From<(Object, usize)> for ConnectionPair {
    fn from((object, index): (Object, usize)) -> Self {
        Self { object, index }
    }
}

#[cfg(test)]
mod tests {
    use crate::context::*;
    use crate::graph::Graph;
    use crate::message::*;

    use super::*;

    #[test]
    fn position() {
        let graph = init_test_graph();
        let object = graph.add_object("osc~", None);

        assert_eq!(object.position(), (0.0, 0.0).into());

        let expected = ObjectPosition::from((10.0, 20.0));
        let object = graph.add_object("osc~", Some(expected));

        assert_eq!(object.position(), expected);

        let expected = ObjectPosition::from((20.0, 10.0));
        object.set_position(expected);
        assert_eq!(object.position(), expected);
    }

    #[test]
    fn outlet_type() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        assert_eq!(osc.outlet_type(0), OutletType::Dsp);
    }

    #[test]
    fn connections() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        let dac = graph.add_object("dac~", None);
        graph.add_connection((osc, 0).into(), (dac, 0).into());
        graph.add_connection((osc, 0).into(), (dac, 1).into());

        assert_eq!(
            osc.connections_at_outlet(0),
            vec![(dac, 0).into(), (dac, 1).into()]
        );
        assert_eq!(dac.connections_at_inlet(0), vec![(osc, 0).into()]);
        assert_eq!(dac.connections_at_inlet(1), vec![(osc, 0).into()]);
    }

    #[test]
    fn label() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        assert_eq!("obj".to_string(), osc.label());
    }

    #[test]
    fn num_io() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        assert_eq!(2, osc.num_inlets());
        assert_eq!(1, osc.num_outlets());
    }

    #[test]
    fn remove() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        assert_eq!(graph.objects(), vec![osc]);
        osc.remove();
        assert_eq!(graph.objects(), vec![]);
    }

    #[test]
    fn send_message() {
        let mut context = Context::<TestDispatcher, AudioLoopF32>::new(Config::default()).unwrap();
        let receiver_name = "connection-test-r";
        context.register_receiver(receiver_name);
        let graph = Graph::new_empty(context.clone());
        let sender = graph.add_object(&format!("send {}", receiver_name), None);
        graph.attach();

        for _ in 0..context.config().blocksize + 1 {
            context.next_frame(&[0.0, 0.0]).unwrap();
        }

        assert_eq!(*context.user_data(), 0);

        sender.send_message(0, 
            Message::builder()
                .with_element(MessageElement::Bang)
                .build());

        for _ in 0..context.config().blocksize + 1 {
            context.next_frame(&[0.0, 0.0]).unwrap();
        }

        assert_eq!(*context.user_data(), 27);
    }

    #[test]
    fn to_string() {
        let graph = init_test_graph();
        let osc = graph.add_object("osc~", None);
        assert_eq!(osc.to_string(), "osc~ 440".to_string());
    }

    fn init_test_graph() -> Graph<TestDispatcher, AudioLoopF32> {
        let context = Context::<TestDispatcher, AudioLoopF32>::new(Config::default()).unwrap();
        Graph::new_empty(context)
    }

    #[derive(Debug, Clone)]
    struct TestDispatcher;

    impl Dispatcher for TestDispatcher {
        type UserData = u64;

        fn receiver_message(_name: String, _message: Option<Message>, data: &mut Self::UserData) {
            *data = 27;
        }
    }
}
