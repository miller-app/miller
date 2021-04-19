//! This module contains [Context] and related types.

use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use zengarden_raw::{
    zg_context_delete, zg_context_get_userinfo, zg_context_new, PdContext, ZGCallbackFunction,
    ZGReceiverMessagePair, ZGMessage
};

/// [Context] represents a Pure Data context. There can be multiple contexts, each with its own
/// configuration (i.e. sample rate, block size, etc.) and audio loop. Contexts aren't supposed to
/// share data between each other, but there can be multiple graphs within a context, which may
/// share data between themselves.
#[derive(Debug)]
pub struct Context<C: Callback> {
    raw_context: Arc<RwLock<*mut PdContext>>,
    config: Config,
    _callback: PhantomData<C>,
}

impl<C: Callback> Context<C> {
    /// [Context] initializer.
    pub fn new(config: Config, user_data: C::UserData) -> Result<Self, Error> {
        // Thread-safity for the user data is implemented on the ZenGarden's side. But in case of
        // threading issues this should be the first place to look.
        Ok(Self {
            raw_context: Self::init_raw_context(
                &config,
                Box::into_raw(Box::new(user_data)) as *mut c_void,
            )?,
            config,
            _callback: Default::default(),
        })
    }

    fn init_raw_context(
        config: &Config,
        user_data: *mut c_void,
    ) -> Result<Arc<RwLock<*mut PdContext>>, Error> {
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

        Ok(Arc::new(RwLock::new(raw_context)))
    }

    unsafe extern "C" fn raw_callback(
        msg_t: ZGCallbackFunction,
        udata: *mut c_void,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let data = (udata as *mut C::UserData).as_mut().unwrap();

        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD | ZGCallbackFunction::ZG_PRINT_ERR => {
                Self::print_callback(msg_t, data, ptr)
            }
            ZGCallbackFunction::ZG_PD_DSP => Self::switch_dsp_callback(data, ptr),
            ZGCallbackFunction::ZG_RECEIVER_MESSAGE => Self::receiver_message_callback(data, ptr),
            ZGCallbackFunction::ZG_CANNOT_FIND_OBJECT => Self::obj_not_found_callback(data, ptr),
        }
    }

    unsafe fn print_callback(
        msg_t: ZGCallbackFunction,
        udata: &mut C::UserData,
        str_ptr: *mut c_void,
    ) -> *mut c_void {
        let msg: String = CString::from_raw(str_ptr as *mut c_char)
            .to_string_lossy()
            .into();
        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD => C::print_std(msg, udata),
            ZGCallbackFunction::ZG_PRINT_ERR => C::print_err(msg, udata),
            _ => unreachable!(),
        }

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn switch_dsp_callback(udata: &mut C::UserData, ptr: *mut c_void) -> *mut c_void {
        let state = if ptr as i32 > 0 { true } else { false };
        C::switch_dsp(state, udata);

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn receiver_message_callback(udata: &mut C::UserData, ptr: *mut c_void) -> *mut c_void {
        let raw_message = ptr as *mut ZGReceiverMessagePair;
        let receiver_name: String = CStr::from_ptr((*raw_message).receiverName)
            .to_string_lossy()
            .into();
        C::receiver_message(ReceiverMessage { receiver_name }, udata);
        ptr::null::<c_void>() as *mut _
    }

    unsafe fn obj_not_found_callback(
        udata: &mut C::UserData,
        raw_name: *mut c_void,
    ) -> *mut c_void {
        let name: String = CString::from_raw(raw_name as *mut c_char)
            .to_string_lossy()
            .into();
        match C::cannot_find_obj(name, udata) {
            Some(path) => CString::new(path.as_str())
                .expect(&format!("Can't initialize CString from {}", path))
                .into_raw() as *mut c_void,
            None => ptr::null::<c_void>() as *mut _,
        }
    }

    /// Borrow user data.
    pub fn user_data(&self) -> &'_ C::UserData {
        unsafe {
            let raw = zg_context_get_userinfo(*(self.raw_context.clone().read().unwrap()));
            (raw as *mut C::UserData).as_ref().unwrap()
        }
    }

    /// Borrow mutable user data.
    pub fn user_data_mut(&self) -> &'_ mut C::UserData {
        unsafe {
            let raw = zg_context_get_userinfo(*(self.raw_context.clone().write().unwrap()));
            (raw as *mut C::UserData).as_mut().unwrap()
        }
    }
}

impl<C: Callback> Drop for Context<C> {
    fn drop(&mut self) {
        unsafe {
            zg_context_delete(*(self.raw_context.read().unwrap()));
        }
    }
}

/// Callback, which you can implement to handle events from [Context].
///
/// All methods are optional.
pub trait Callback: fmt::Debug {
    /// The user data type, which will be passed to the callbacks.
    type UserData;

    /// Print standard message.
    fn print_std(_: String, _: &mut Self::UserData) {}

    /// Print error message.
    fn print_err(_: String, _: &mut Self::UserData) {}

    /// Suggestion to turn on or off context signal processing. The message is called only when the
    /// context's process function is running.
    fn switch_dsp(_: bool, _: &mut Self::UserData) {}

    /// Called when a message for the registered with [Context::register_receiver] receiver is
    /// send.
    fn receiver_message(_: ReceiverMessage, _: &mut Self::UserData) {}

    /// A referenced object, abstraction or external can't be found in the current context.
    ///
    /// The first argument is the name of the object.
    ///
    /// Optionally, you can return the path to the object definition.
    fn cannot_find_obj(_: String, _: &mut Self::UserData) -> Option<String> {
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

    #[test]
    fn context_user_data() {
        #[derive(Debug)]
        struct TestCallback;

        impl Callback for TestCallback {
            type UserData = u32;
        }

        let expected = 42;
        let context = Context::<TestCallback>::new(Config::default(), expected).unwrap();
        assert_eq!(expected, *context.user_data());

        let data = context.user_data_mut();
        *data = 27;

        assert_eq!(27, *context.user_data());
    }

    #[test]
    fn callback() {
        let context =
            Context::<TestCallback>::new(Config::default(), TestUserData(String::new())).unwrap();
        let data_ptr: *mut TestUserData = context.user_data_mut();
        let data_raw = data_ptr as *mut c_void;

        unsafe {
            test_print_std(&context, data_raw);
            test_print_err(&context, data_raw);
            test_switch_dsp(&context, data_raw);
            test_receiver_msg(&context, data_raw);
        }
    }

    unsafe fn test_print_std(context: &'_ Context<TestCallback>, data: *mut c_void) {
        let expected = "foo";
        let msg = CString::new(expected).unwrap().into_raw() as *mut c_void;
        let result =
            Context::<TestCallback>::raw_callback(ZGCallbackFunction::ZG_PRINT_STD, data, msg);

        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_print_err(context: &'_ Context<TestCallback>, data: *mut c_void) {
        let expected = "bar";
        let msg = CString::new(expected).unwrap().into_raw() as *mut c_void;
        let result =
            Context::<TestCallback>::raw_callback(ZGCallbackFunction::ZG_PRINT_ERR, data, msg);
        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_switch_dsp(context: &'_ Context<TestCallback>, data: *mut c_void) {
        let expected = "true";
        let msg = true as i32 as *mut c_void;
        let result =
            Context::<TestCallback>::raw_callback(ZGCallbackFunction::ZG_PD_DSP, data, msg);
        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_receiver_msg(context: &'_ Context<TestCallback>, data: *mut c_void) {
        let expected = String::from("receiver_name");
        let name = CString::new(expected.as_str()).unwrap().into_raw();
        let msg = Box::into_raw(Box::new(ZGReceiverMessagePair {
            receiverName: name,
            message: ptr::null::<ZGMessage>() as *mut ZGMessage, //TODO
        })) as *mut c_void;
        let result = Context::<TestCallback>::raw_callback(
            ZGCallbackFunction::ZG_RECEIVER_MESSAGE,
            data,
            msg,
        );
        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);

        // make the memory be managed back by rust
        let _ = CString::from_raw(name);
        let _ = Box::from_raw(msg as *mut ZGReceiverMessagePair);
    }

    #[derive(Debug)]
    struct TestCallback;

    impl Callback for TestCallback {
        type UserData = TestUserData;

        fn print_std(message: String, data: &mut Self::UserData) {
            data.0 = message;
        }

        fn print_err(message: String, data: &mut Self::UserData) {
            TestCallback::print_std(message, data);
        }

        fn switch_dsp(state: bool, data: &mut Self::UserData) {
            if state {
                data.0 = String::from("true");
            }
        }

        fn receiver_message(msg: ReceiverMessage, data: &mut Self::UserData) {
            data.0 = msg.receiver_name.clone();
        }
    }

    #[derive(Debug)]
    struct TestUserData(String);
}
