//! Contains stuff for writing externals.
//!
//! There are objects which only process messages, the [MessageObject], and those objects which
//! process messages and audio, the [DspObject].

use super::{ConnectionPair, ObjectPosition, OutletType};
use crate::context::{AudioLoop, Dispatcher};
use crate::graph::Graph;
use crate::message::Message;

/// The message object.
pub trait MessageObject<D: Dispatcher, L: AudioLoop>: ToString {
    /// The generic entrypoint of a message to an object. This function usually either passes the
    /// message directly to [MessageObject::process_message] in the case of an object which only
    /// processes messages, or queues the message for later processing.
    fn receive_message(&mut self, _inlet: usize, _message: Message) {}

    /// The message logic of an object.
    fn process_message(&mut self, inlet: usize, message: Message);

    /// Sends the given message to all connected objects at the given outlet index. This function
    /// can be overridden in order to take some other action, such as additionally scheduling a
    /// new message as in the case of `MessageMetro`.
    fn send_message(&mut self, _outlet: usize, _message: Message) {}

    /// Get the [object::OutletType] of the given outlet.
    fn outlet_type(&self, outlet: usize) -> OutletType;

    /// Get connections at inlet.
    fn connections_at_inlet(&self, _inlet: usize) -> Vec<ConnectionPair> {
        Vec::new()
    }

    /// Get connections at outlet.
    fn connections_at_outlet(&self, _outlet: usize) -> Vec<ConnectionPair> {
        Vec::new()
    }

    /// Establish a connection from another object to this object.
    fn connect_to_inlet(&self, _inlet: usize, _connection: ConnectionPair) {}

    /// Establish a connection to another object from this object.
    fn connect_to_outlet(&self, _outlet: usize, _connection: ConnectionPair) {}

    /// Remove a connection to another object from this object. This function does not remove the
    /// connection reference at the connecting object. It must be removed separately.
    fn disconnect_inlet(&self, _inlet: usize, _connection: ConnectionPair) {}

    /// Remove a connection to another object from this object. This function does not remove the
    /// connection reference at the connecting object. It must be removed separately.
    fn disconnect_outlet(&self, _outlet: usize, _connection: ConnectionPair) {}

    /// The destination inlet of an outgoing message connection can change if an `[inlet]` object
    /// in a graph is moved (and the inlet ordering changes). The connection index change has no
    /// effect on the graph ordering and thus it is not necessary to remove and readd a connection.
    /// However, the connection must be updated such that message will still be addressed to the
    /// correct inlet.
    fn update_outlet_connection(
        &self,
        _outlet: usize,
        _old_connection: ConnectionPair,
        _new_inlet: usize,
    ) {
    }

    /// Same as [MessageObject::update_outlet_connection], but for inlets.
    fn update_inlet_connection(
        &self,
        _inlet: usize,
        _old_connection: ConnectionPair,
        _new_outlet: usize,
    ) {
    }

    /// Get object label.
    fn label(&self) -> String {
        "obj".to_string()
    }

    /// Get object type.
    fn object_type(&self) -> ObjectType;

    /// Returns `true` if this object processes audio, `false` otherwise.
    fn does_process_audio(&self) -> bool;

    /// Returns `true` if this object should distribute the elements of the incoming message across
    /// the inlets. A message is otherwise only distributed if the message arrives on the left-most
    /// inlet and has more than one inlet. This function returns `true` by default and should be
    /// overridden to return `false` if this behaviour is not desired (e.g., as in the case of the
    /// `line` object). This behaviour is set to `false` for all `DspObject` objects.
    fn should_distribute_message_to_inlets(&self) -> bool {
        true
    }

    /// Returns `true` if this object is a leaf in the Pd tree. `false` otherwise. This function is
    /// used only while computing the process order of objects. For this reason it also returns
    /// true in the cases when the object is `send`, `send~`, or `throw~`.
    fn is_leaf_node(&self) -> bool;

    /// Returns an ordered list of all parent objects of this object.
    fn process_order(&self) -> Vec<Box<dyn DspObject<D, L>>>;

    /// Reset the `is_ordered` flag to `false`. This is necessary in order to recompute the process
    /// order.
    fn reset_ordered_flag(&self) {}

    /// Get number of inlets.
    fn num_inlets(&self) -> usize;

    /// Get number of outlets.
    fn num_outlets(&self) -> usize;

    /// Get graph in which this object exists.
    fn graph(&self) -> Graph<D, L>;

    /// Get position on the canvas.
    fn position(&self) -> ObjectPosition;

    /// Set position on the canvas.
    fn set_position(&self, position: ObjectPosition);
}

#[allow(missing_docs)]
pub enum ObjectType {
    DspAdc,
    DspAdd,
    DspBandpassFilter,
    DspCatch,
    DspClip,
    DspCosine,
    DspDac,
    DspTablePlay,
    DspDelayRead,
    DspDelayWrite,
    DspInlet,
    DspOutlet,
    DspReceive,
    DspSend,
    DspTableRead,
    DspTableRead4,
    DspTableWrite,
    DspThrow,
    DspVariableDelay,
    MessageInlet,
    MessageNotein,
    MessageOutlet,
    MessageReceive,
    MessageSend,
    MessageTable,
    MessageTableRead,
    MessageTableWrite,
    ObjectPd,
    ObjectUnknown,
}

/// The DSP object.
pub trait DspObject<D: Dispatcher, L: AudioLoop>: MessageObject<D, L> {}
