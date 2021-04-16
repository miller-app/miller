//! This module contains [Context] and related types.

use std::ffi::{c_void, CStr};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;
use std::sync::{atomic::AtomicPtr, Arc, RwLock};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use zengarden_raw::{
    zg_context_delete, zg_context_new, PdContext, ZGCallbackFunction, ZGReceiverMessagePair,
};

/// [Context] represents a Pure Data context. There can be multiple contexts, each with its own
/// configuration (i.e. sample rate, block size, etc.) and audio loop. Contexts aren't supposed to
/// share data between each other, but there can be multiple graphs within a context, which may
/// share data between themselves.
#[derive(Debug)]
pub struct Context<C: Callback> {
    raw_context: AtomicPtr<PdContext>,
    config: Config,
    user_data: Arc<RwLock<Option<Box<dyn UserData>>>>,
    _callback: PhantomData<C>,
}

impl<C: Callback> Context<C> {
    /// [Context] initializer.
    pub fn new(config: Config, user_data: Option<Box<dyn UserData>>) -> Result<Self, Error> {
        let user_data = Arc::new(RwLock::new(user_data));
        Ok(Self {
            raw_context: Self::init_raw_context(
                &config,
                Arc::into_raw(user_data.clone()) as *mut c_void,
            )?,
            config,
            user_data,
            _callback: Default::default(),
        })
    }

    fn init_raw_context(
        config: &Config,
        user_data: *mut c_void,
    ) -> Result<AtomicPtr<PdContext>, Error> {
        let raw_context = unsafe {
            zg_context_new(
                config.input_ch_num as i32,
                config.output_ch_num as i32,
                config.blocksize as i32,
                config.sample_rate as f32,
                Some(Self::raw_callback),
                user_data,
            )
        };

        if raw_context.is_null() {
            return Err(Error::Initializing);
        }

        Ok(AtomicPtr::new(raw_context))
    }

    unsafe extern "C" fn raw_callback(
        msg_t: ZGCallbackFunction,
        udata: *mut c_void,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let ud: Arc<RwLock<Option<Box<dyn UserData>>>> =
            Arc::from_raw(udata as *mut RwLock<Option<Box<dyn UserData>>>);
        let mut data = ud.write().unwrap();
        let udata = data.as_mut();

        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD | ZGCallbackFunction::ZG_PRINT_ERR => {
                Self::print_callback(msg_t, udata, ptr)
            }
            ZGCallbackFunction::ZG_PD_DSP => Self::switch_dsp_callback(udata, ptr),
            ZGCallbackFunction::ZG_RECEIVER_MESSAGE => Self::receiver_message_callback(udata, ptr),
            ZGCallbackFunction::ZG_CANNOT_FIND_OBJECT => Self::obj_not_found_callback(udata, ptr),
        }
    }

    unsafe fn print_callback(
        msg_t: ZGCallbackFunction,
        udata: Option<&mut Box<dyn UserData>>,
        str_ptr: *mut c_void,
    ) -> *mut c_void {
        let msg: String = CStr::from_ptr(str_ptr as *const c_char)
            .to_string_lossy()
            .into();
        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD => C::print_std(msg, udata),
            ZGCallbackFunction::ZG_PRINT_ERR => C::print_err(msg, udata),
            _ => unreachable!(),
        }

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn switch_dsp_callback(
        udata: Option<&mut Box<dyn UserData>>,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let state = if ptr as i32 > 0 { true } else { false };
        C::switch_dsp(state, udata);

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn receiver_message_callback(
        udata: Option<&mut Box<dyn UserData>>,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let raw_message = ptr as *mut ZGReceiverMessagePair;
        let receiver_name: String = CStr::from_ptr((*raw_message).receiverName)
            .to_string_lossy()
            .into();
        C::receiver_message(ReceiverMessage { receiver_name }, udata);
        ptr::null::<c_void>() as *mut _
    }

    unsafe fn obj_not_found_callback(
        udata: Option<&mut Box<dyn UserData>>,
        raw_name: *mut c_void,
    ) -> *mut c_void {
        let name: String = CStr::from_ptr(raw_name as *const c_char)
            .to_string_lossy()
            .into();
        match C::cannot_find_obj(name, udata) {
            Some(path) => path.into_bytes().as_mut_slice().as_mut_ptr() as *mut c_void,
            None => ptr::null::<c_void>() as *mut _,
        }
    }
}

impl<C: Callback> Drop for Context<C> {
    fn drop(&mut self) {
        unsafe {
            zg_context_delete(*self.raw_context.get_mut());
        }
    }
}

/// User data, which is passed to callbacks and can be referenced from the context.
pub trait UserData: fmt::Debug {}

/// Callback, which you can implement to handle events from [Context].
///
/// All methods are optional.
pub trait Callback: fmt::Debug {
    /// Print standard message.
    fn print_std(_: String, _: Option<&mut Box<dyn UserData>>) {}

    /// Print error message.
    fn print_err(_: String, _: Option<&mut Box<dyn UserData>>) {}

    /// Suggestion to turn on or off context signal processing. The message is called only when the
    /// context's process function is running.
    fn switch_dsp(_: bool, _: Option<&mut Box<dyn UserData>>) {}

    /// Called when a message for the registered with [Context::register_receiver] receiver is
    /// send.
    fn receiver_message(_: ReceiverMessage, _: Option<&mut Box<dyn UserData>>) {}

    /// A referenced object, abstraction or external can't be found in the current context.
    ///
    /// The first argument is the name of the object.
    ///
    /// Optionally, you can return the path to the object definition.
    fn cannot_find_obj(_: String, _: Option<&mut Box<dyn UserData>>) -> Option<String> {
        None
    }
}

/// Message sent to registered receiver.
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
