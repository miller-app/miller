//! This module contains the message builder.

use std::{ffi::CString, sync::RwLock};

use zengarden_raw::{
    zg_message_delete, zg_message_new, zg_message_set_bang, zg_message_set_float,
    zg_message_set_symbol, ZGMessage,
};

/// Messages can be sent to a context and corresponding receivers will get them.
#[derive(Default, Debug)]
pub struct Message {
    is_built: bool,
    timestamp: f64,
    elements: Vec<MessageElement>,
    pub(crate) raw_message: Option<RwLock<*mut ZGMessage>>,
}

impl Message {
    /// Append element to the message chain.
    ///
    /// You can't append elements to a built message.
    pub fn with_element(mut self, element: MessageElement) -> Self {
        if !self.is_built {
            self.elements.push(element);
        }
        self
    }

    /// Set message timestamp.
    ///
    /// You can't change it after message is built.
    pub fn with_timestamp(mut self, timestamp: f64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Build the message.
    ///
    /// Should be called after you append all the elements to the message.
    pub fn build(mut self) -> Self {
        if !self.is_built {
            unsafe {
                let raw_message = zg_message_new(self.timestamp, self.elements.len() as u32);
                self.collect_elements_to_message(raw_message);
                self.raw_message = Some(RwLock::new(raw_message));
            }
            self.is_built = true;
        }
        self
    }

    unsafe fn collect_elements_to_message(&mut self, raw_message: *mut ZGMessage) {
        for (n, element) in self.elements.iter().enumerate() {
            let index = n as u32;
            match element {
                MessageElement::Float(value) => {
                    zg_message_set_float(raw_message, index, *value as f32)
                }
                MessageElement::Symbol(value) => {
                    let symbol =
                        CString::new(value.to_owned()).expect("Cannot build symbol from string");
                    zg_message_set_symbol(raw_message, index, symbol.as_ptr())
                }
                MessageElement::Bang => zg_message_set_bang(raw_message, index),
            }
        }
    }

    /// Get number of elements for this message.
    pub fn num_elements(&self) -> usize {
        self.elements.len()
    }

    /// Get element at index.
    pub fn element_at(&self, index: usize) -> &MessageElement {
        &self.elements[index]
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        if let Some(ref message) = self.raw_message {
            unsafe {
                zg_message_delete(*message.write().unwrap());
            }
        }
    }
}

/// Message element type.
#[derive(Debug)]
pub enum MessageElement {
    /// Float.
    Float(f64),
    /// Symbol.
    Symbol(String),
    /// Bang.
    Bang,
}

impl Default for MessageElement {
    fn default() -> Self {
        MessageElement::Bang
    }
}
