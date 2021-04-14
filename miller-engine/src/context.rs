//! Pure Data context related stuff.

use std::ffi::{c_void, CString};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ptr;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use zengarden_raw::{zg_context_delete, zg_context_new, PdContext, ZGCallbackFunction};

/// Pure Data context. There might be multiple contexts.
#[derive(Debug)]
pub struct Context<U: UserData, C: Callback<U>> {
    raw_context: Arc<Mutex<*mut PdContext>>,
    config: Config,
    user_data: U,
    _callback: PhantomData<C>,
}

impl<U: UserData, C: Callback<U>> Context<U, C> {
    /// [Context] initializer.
    pub fn new(config: Config) -> Result<Self, Error> {
        Ok(Self {
            raw_context: Self::init_raw_context(&config)?,
            config,
            user_data: U::default(),
            _callback: Default::default(),
        })
    }

    fn init_raw_context(config: &Config) -> Result<Arc<Mutex<*mut PdContext>>, Error> {
        let raw_context = unsafe {
            zg_context_new(
                config.input_ch_num as i32,
                config.output_ch_num as i32,
                config.blocksize as i32,
                config.sample_rate as f32,
                Some(Self::raw_callback),
                ptr::null::<c_void>() as *mut _,
            )
        };

        if raw_context.is_null() {
            return Err(Error::Initializing);
        }

        Ok(Arc::new(Mutex::new(raw_context)))
    }

    unsafe extern "C" fn raw_callback(
        msg: ZGCallbackFunction,
        data: *mut c_void,
        ptr: *mut c_void,
    ) -> *mut c_void {
        match msg {
            ZGCallbackFunction::ZG_PRINT_STD => {
                C::print_std("test", &mut U::default());
                println!(
                    "{}",
                    CString::from_raw(ptr as *mut i8).into_string().unwrap()
                );
            }
            ZGCallbackFunction::ZG_PRINT_ERR => {
                eprintln!(
                    "{}",
                    CString::from_raw(ptr as *mut i8).into_string().unwrap()
                );
            }
            _ => (),
        }

        ptr::null::<c_void>() as *mut _
    }
}

impl<U: UserData, C: Callback<U>> Drop for Context<U, C> {
    fn drop(&mut self) {
        unsafe {
            zg_context_delete(*(self.raw_context.lock().unwrap()));
        }
    }
}

/// User data, which is passed to callbacks and can be referenced from the context.
pub trait UserData: Default + Send + Sync {}

/// Context configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The number of input channels.
    pub input_ch_num: usize,
    /// The number of output channels.
    pub output_ch_num: usize,
    /// The computation block size.
    pub blocksize: usize,
    /// The sample rate.
    pub sample_rate: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            input_ch_num: 2,
            output_ch_num: 2,
            blocksize: 64,
            sample_rate: 44100,
        }
    }
}

impl Config {
    /// Set input channels number.
    pub fn with_in_ch_num(mut self, ch_num: usize) -> Self {
        self.input_ch_num = ch_num;
        self
    }

    /// Set output channels number.
    pub fn with_out_ch_num(mut self, ch_num: usize) -> Self {
        self.output_ch_num = ch_num;
        self
    }

    /// Set computation block size.
    pub fn with_block_size(mut self, blocksize: usize) -> Self {
        self.blocksize = blocksize;
        self
    }

    /// Set sample rate.
    pub fn with_sample_rate(mut self, sr: usize) -> Self {
        self.sample_rate = sr;
        self
    }
}

/// [Context] callback. You can set it with [Context::set_callback].
pub trait Callback<U: Default + Send + Sync>: Debug {
    /// Print standard message.
    fn print_std(_: &'_ str, _: &mut U) {}

    /// Print error message.
    fn print_err(_: &'_ str, _: &mut U) {}

    /// Suggestion to turn on or off context signal processing. The message is called only when the
    /// context's process function is running.
    fn switch_dsp(_: bool, _: &mut U) {}

    /// Called when a message for the registered with [Context::register_receiver] receiver is
    /// send.
    fn receiver_message(_: ReceiverMessage, _: &mut U) {}

    /// A referenced object/abstraction/external can't be found in the current context.
    fn cannot_find_obj(_: &'_ str, _: &mut U) {}
}

/// Message send to registered receiver.
#[derive(Clone, Debug)]
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

/// [Context] errors.
#[derive(Debug, Error)]
pub enum Error {
    /// Error initializing.
    #[error("Can't initalize context")]
    Initializing,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn config_defaults() {
        let config = Config::default();

        assert_eq!(config.input_ch_num, 2);
        assert_eq!(config.output_ch_num, 2);
        assert_eq!(config.sample_rate, 44100);
        assert_eq!(config.blocksize, 64);
    }
}
