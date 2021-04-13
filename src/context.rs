//! Pure Data context related stuff.

use zengarden_raw::PdContext;

/// Pure Data context. There might be multiple contexts.
pub struct Context {
    raw_context: *mut PdContext,
    callback: Option<Box<dyn Callback>>,
}

impl Context {
    /// [Context] initializer.
    pub fn new() -> Self {
        todo!()
    }

    /// Set the callback for different kind of events on [Context] (see [Callback]).
    pub fn set_callback(&mut self, callback: Box<dyn Callback>) {
        self.callback = Some(callback);
    }
}

/// [Context] callback. You can set it with [Context::set_callback].
pub trait Callback {
    /// Print standard message.
    fn print_std(&self, _: &'_ str) {}

    /// Print error message.
    fn print_err(&self, _: &'_ str) {}

    /// Suggestion to turn on or off context signal processing. The message is called only when the
    /// context's process function is running.
    fn switch_dsp(&mut self, _: bool) {}

    /// Called when a message for the registered with [Context::register_receiver] receiver is
    /// send.
    fn receiver_message(&mut self, _: ReceiverMessage) {}

    /// A referenced object/abstraction/external can't be found in the current context.
    fn cannot_find_obj(&mut self, _: &'_ str) {}
}

/// Message send to registered receiver.
pub struct ReceiverMessage {
    receiver_name: String,
    // message: todo!(),
}

impl ReceiverMessage {
    /// Get the receiver name.
    pub fn receiver_name(&self) -> &str {
        &self.receiver_name
    }
}
