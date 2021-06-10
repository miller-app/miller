//! This module contains the message-related stuff.

use std::{
    ffi::{CStr, CString},
    string::ToString,
    sync::RwLock,
};

use thiserror::Error;
use zengarden_raw::{
    zg_message_delete, zg_message_get_element_type, zg_message_get_float,
    zg_message_get_num_elements, zg_message_get_symbol, zg_message_get_timestamp, zg_message_new,
    zg_message_new_from_string, zg_message_set_bang, zg_message_set_float, zg_message_set_symbol,
    zg_message_to_string, ZGMessage, ZGMessageElementType,
};

/// Messages can be sent to a context and corresponding receivers will get them.
///
/// You can also receive messages for registered receivers (i.e. you can send a message from a
/// patch to the engine).
#[derive(Debug)]
pub struct Message {
    timestamp: f64,
    elements: Vec<MessageElement>,
    raw_message: RwLock<*mut ZGMessage>,
}

impl Message {
    /// Initialize [MessageBuilder].
    ///
    /// Use this to build messages.
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }

    /// Get number of elements for this message.
    pub fn num_elements(&self) -> usize {
        self.elements.len()
    }

    /// Get element at index.
    pub fn element_at(&self, index: usize) -> Option<&MessageElement> {
        self.elements.get(index)
    }

    /// Initialize a message from string.
    pub fn from_str(timestamp: f64, message: &str) -> Result<Self, Error> {
        unsafe {
            let raw_str =
                CString::new(message).expect(&format!("Cannot build raw string from {}", message));
            let raw_message = zg_message_new_from_string(timestamp, raw_str.as_ptr());

            if raw_message.is_null() {
                return Err(Error::Parse);
            }

            Self::from_raw(raw_message).ok_or(Error::RawMessageIsNull)
        }
    }

    pub(crate) unsafe fn from_raw(raw_message: *mut ZGMessage) -> Option<Self> {
        if raw_message.is_null() {
            return None;
        }
        //TODO the better way would be copying memory pointed by raw_message and initializing the
        //Message using it, but I didn't manage core::ptr::copy typing for that
        let mut builder =
            MessageBuilder::default().with_timestamp(zg_message_get_timestamp(raw_message));
        builder.collect_elements_to_message(raw_message);
        let mut message = builder.build();
        message.collect_elements_from_raw_message(raw_message);
        Some(message)
    }

    unsafe fn collect_elements_from_raw_message(&mut self, raw_message: *mut ZGMessage) {
        let num_elements = zg_message_get_num_elements(raw_message);

        for n in 0..num_elements {
            let element = match zg_message_get_element_type(raw_message, n) {
                ZGMessageElementType::ZG_MESSAGE_ELEMENT_FLOAT => {
                    MessageElement::Float(zg_message_get_float(raw_message, n) as f64)
                }
                ZGMessageElementType::ZG_MESSAGE_ELEMENT_SYMBOL => {
                    let str_ptr = zg_message_get_symbol(raw_message, n);
                    let raw_str = CStr::from_ptr(str_ptr);
                    MessageElement::Symbol(raw_str.to_string_lossy().to_string())
                }
                ZGMessageElementType::ZG_MESSAGE_ELEMENT_BANG => MessageElement::Bang,
            };
            self.elements.push(element);
        }
    }

    pub(crate) unsafe fn into_raw(self) -> *mut ZGMessage {
        let null_ptr = std::ptr::null::<ZGMessage>() as *mut ZGMessage;
        let raw = std::mem::replace(&mut *self.raw_message.write().unwrap(), null_ptr);
        raw
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        let raw = *self.raw_message.write().unwrap();
        if !raw.is_null() {
            unsafe {
                zg_message_delete(raw);
            }
        }
    }
}

impl ToString for Message {
    fn to_string(&self) -> String {
        unsafe {
            let raw = *self.raw_message.write().unwrap();
            if !raw.is_null() {
                let raw_str = CStr::from_ptr(zg_message_to_string(raw));
                return raw_str.to_string_lossy().to_string();
            }

            String::new()
        }
    }
}

/// Message builder. Initialize it using [Message::builder].
#[derive(Debug, Default)]
pub struct MessageBuilder {
    timestamp: f64,
    elements: Vec<MessageElement>,
}

impl MessageBuilder {
    /// Append element to the message chain.
    pub fn with_element(mut self, element: MessageElement) -> Self {
        self.elements.push(element);
        self
    }

    /// Set message timestamp.
    pub fn with_timestamp(mut self, timestamp: f64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// Build the [Message].
    ///
    /// Should be called after you append all the elements to the message.
    pub fn build(mut self) -> Message {
        unsafe {
            let raw_message = zg_message_new(self.timestamp, self.elements.len() as u32);
            self.collect_elements_to_message(raw_message);
            let raw_message = RwLock::new(raw_message);
            Message {
                timestamp: self.timestamp,
                elements: self.elements,
                raw_message,
            }
        }
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
}

/// Message element type.
#[derive(Debug, Clone, PartialEq)]
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

/// [Message] errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Error parsing message from string.
    #[error("Can't parse message.")]
    Parse,
    /// Raw message is null.
    #[error("Raw message is null.")]
    RawMessageIsNull,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_build() {
        let message = Message::builder()
            .with_timestamp(12.345)
            .with_element(MessageElement::Float(1.2))
            .with_element(MessageElement::Symbol("foo".to_string()))
            .with_element(MessageElement::Symbol("bar".to_string()))
            .with_element(MessageElement::Bang)
            .build();

        assert!(!message.raw_message.write().unwrap().is_null());
        assert_eq!(message.timestamp, 12.345);
        assert_eq!(message.num_elements(), 4);
        assert_eq!(message.element_at(0), Some(&MessageElement::Float(1.2)));
        assert_eq!(
            message.element_at(1),
            Some(&MessageElement::Symbol("foo".to_string()))
        );
        assert_eq!(
            message.element_at(2),
            Some(&MessageElement::Symbol("bar".to_string()))
        );
        assert_eq!(message.element_at(3), Some(&MessageElement::Bang));
    }

    #[test]
    fn message_to_string() {
        let message = Message::builder()
            .with_timestamp(12.345)
            .with_element(MessageElement::Float(1.2))
            .with_element(MessageElement::Symbol("foo".to_string()))
            .with_element(MessageElement::Symbol("bar".to_string()))
            .with_element(MessageElement::Bang)
            .build();

        assert_eq!("1.2 foo bar bang".to_string(), message.to_string());
    }

    #[test]
    fn message_from_string() {
        let message = Message::from_str(12.345, "1.0 foo bar bang").unwrap();
        let expected = Message::builder()
            .with_timestamp(12.345)
            .with_element(MessageElement::Float(1.0))
            .with_element(MessageElement::Symbol("foo".to_string()))
            .with_element(MessageElement::Symbol("bar".to_string()))
            .with_element(MessageElement::Bang)
            .build();

        assert_eq!(message.timestamp, expected.timestamp);

        for n in 0..message.num_elements() {
            assert_eq!(message.element_at(n), expected.element_at(n));
        }
    }
}
