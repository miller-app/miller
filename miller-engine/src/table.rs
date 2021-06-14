//! Pd table/array.

use zengarden_raw::{zg_table_get_buffer, zg_table_set_buffer};

use crate::object::Object;

/// Pd table/array.
#[derive(Debug, Clone)]
pub struct Table(Object);

impl Table {
    /// Clone table buffer for the given lentgh.
    pub fn buffer(&self, length: usize) -> &[f32] {
        unsafe {
            let raw = zg_table_get_buffer(self.0 .0, &mut (length as u32));
            std::slice::from_raw_parts(raw, length)
        }
    }

    /// The table’s buffer is resized and copied from the given buffer. This set operation is
    /// thread-safe especially with regards to [context::Context::process].
    pub fn set_buffer(&self, buffer: &mut [f32]) {
        unsafe {
            zg_table_set_buffer(self.0 .0, buffer.as_mut_ptr(), buffer.len() as u32);
        }
    }
}
