//! This module contains [Context] and related types.

mod audioloop;

use std::ffi::{c_void, CStr, CString};
use std::fmt;
use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;
use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use thiserror::Error;
#[allow(unused_imports)]
use zengarden_raw::{
    zg_context_delete, zg_context_get_userinfo, zg_context_new, zg_context_process,
    zg_context_unregister_receiver, PdContext, ZGCallbackFunction, ZGMessage,
    ZGReceiverMessagePair,
};
use zengarden_raw::{zg_context_register_receiver, zg_context_send_message};

use crate::message::Message;

pub use audioloop::{AudioLoop, AudioLoopF32, AudioLoopI16, Error as AudioLoopError};

/// [Context] represents a Pure Data context. There can be multiple contexts, each with its own
/// configuration (i.e. sample rate, block size, etc.) and audio loop. Contexts aren't supposed to
/// share data between each other, but there can be multiple graphs within a context, which may
/// share data between themselves.
#[derive(Debug)]
pub struct Context<D: Dispatcher, L: AudioLoop> {
    raw_context: RwLock<*mut PdContext>,
    config: Config,
    audio_loop: L,
    _dispatcher: PhantomData<D>,
}

impl<D: Dispatcher, L: AudioLoop> Context<D, L> {
    /// [Context] initializer.
    pub fn new(config: Config, user_data: D::UserData) -> Result<Self, Error> {
        // Thread-safity for the user data is implemented on the ZenGarden's side. But in case of
        // threading issues this should be the first place to look.
        let mut result = Self {
            raw_context: Self::init_raw_context(
                &config,
                Box::into_raw(Box::new(user_data)) as *mut c_void,
            )?,
            audio_loop: Default::default(),
            config: config.clone(),
            _dispatcher: Default::default(),
        };

        result.init_buffers(config.blocksize, config.input_ch_num, config.output_ch_num);

        Ok(result)
    }

    fn init_raw_context(
        config: &Config,
        user_data: *mut c_void,
    ) -> Result<RwLock<*mut PdContext>, Error> {
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

        Ok(RwLock::new(raw_context))
    }

    unsafe extern "C" fn raw_callback(
        msg_t: ZGCallbackFunction,
        udata: *mut c_void,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let data = (udata as *mut D::UserData).as_mut().unwrap();

        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD | ZGCallbackFunction::ZG_PRINT_ERR => {
                Self::print_dispatcher(msg_t, data, ptr)
            }
            ZGCallbackFunction::ZG_PD_DSP => Self::switch_dsp_dispatcher(data, ptr),
            ZGCallbackFunction::ZG_RECEIVER_MESSAGE => Self::receiver_message_dispatcher(data, ptr),
            ZGCallbackFunction::ZG_CANNOT_FIND_OBJECT => Self::obj_not_found_dispatcher(data, ptr),
        }
    }

    unsafe fn print_dispatcher(
        msg_t: ZGCallbackFunction,
        udata: &mut D::UserData,
        str_ptr: *mut c_void,
    ) -> *mut c_void {
        let msg: String = CString::from_raw(str_ptr as *mut c_char)
            .to_string_lossy()
            .into();
        match msg_t {
            ZGCallbackFunction::ZG_PRINT_STD => D::print_std(msg, udata),
            ZGCallbackFunction::ZG_PRINT_ERR => D::print_err(msg, udata),
            _ => unreachable!(),
        }

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn switch_dsp_dispatcher(udata: &mut D::UserData, ptr: *mut c_void) -> *mut c_void {
        let state = if ptr as i32 > 0 { true } else { false };
        D::switch_dsp(state, udata);

        ptr::null::<c_void>() as *mut _
    }

    unsafe fn receiver_message_dispatcher(
        udata: &mut D::UserData,
        ptr: *mut c_void,
    ) -> *mut c_void {
        let raw_receiver_message = ptr as *mut ZGReceiverMessagePair;
        let receiver_name: String = CStr::from_ptr((*raw_receiver_message).receiverName)
            .to_string_lossy()
            .into();
        let message = Message::from_raw_message((*raw_receiver_message).message);
        D::receiver_message(receiver_name, message, udata);
        ptr::null::<c_void>() as *mut _
    }

    unsafe fn obj_not_found_dispatcher(
        udata: &mut D::UserData,
        raw_name: *mut c_void,
    ) -> *mut c_void {
        let name: String = CString::from_raw(raw_name as *mut c_char)
            .to_string_lossy()
            .into();
        match D::cannot_find_obj(name, udata) {
            Some(path) => CString::new(path.as_str())
                .expect(&format!("Can't initialize CString from {}", path))
                .into_raw() as *mut c_void,
            None => ptr::null::<c_void>() as *mut _,
        }
    }

    fn init_buffers(&mut self, blocksize: u16, in_ch_num: u16, out_ch_num: u16) {
        self.audio_loop
            .init_buffers(blocksize, in_ch_num, out_ch_num);
    }

    /// Borrow user data.
    pub fn user_data(&self) -> &'_ D::UserData {
        unsafe {
            let raw = zg_context_get_userinfo(*(self.raw_context.read().unwrap()));
            (raw as *mut D::UserData).as_ref().unwrap()
        }
    }

    /// Borrow mutable user data.
    pub fn user_data_mut(&self) -> &'_ mut D::UserData {
        unsafe {
            let raw = zg_context_get_userinfo(*(self.raw_context.write().unwrap()));
            (raw as *mut D::UserData).as_mut().unwrap()
        }
    }

    /// Get next frame of interleaved audio 32-bit floating point samples.
    ///
    /// The `in_frame` argument is an input stream frame of interleaved 32-bit floating point
    /// samples. It should be equal in size to the number of input channels.
    pub fn next_frame(
        &mut self,
        in_frame: &[L::SampleType],
    ) -> Result<&[L::SampleType], AudioLoopError> {
        let raw_context = self.raw_context.read().unwrap();
        self.audio_loop.next_frame(*raw_context, in_frame)
    }

    /// Send a message to a receiver.
    pub fn send_message(&mut self, receiver: &str, message: Message) {
        unsafe {
            let raw_name = CString::new(receiver)
                .expect(&format!("Can't initialize CString from {}", receiver));
            let raw_message = message.into_raw();
            zg_context_send_message(
                *self.raw_context.read().unwrap(),
                raw_name.as_ptr(),
                raw_message,
            );
        }
    }

    /// Register a receiver for this context.
    pub fn register_receiver(&self, receiver: &str) {
        unsafe {
            let raw_name = CString::new(receiver)
                .expect(&format!("Can't initialize CString from {}", receiver));
            zg_context_register_receiver(*self.raw_context.read().unwrap(), raw_name.as_ptr());
        }
    }

    /// Unregister a receiver for this context.
    pub fn unregister_receiver(&self, receiver: &str) {
        unsafe {
            let raw_name = CString::new(receiver)
                .expect(&format!("Can't initialize CString from {}", receiver));
            zg_context_unregister_receiver(*self.raw_context.read().unwrap(), raw_name.as_ptr());
        }
    }
}

impl<D: Dispatcher, L: AudioLoop> Drop for Context<D, L> {
    fn drop(&mut self) {
        unsafe {
            zg_context_delete(*(self.raw_context.write().unwrap()));
        }
    }
}

/// Dispatcher, which you can implement to handle events from [Context].
///
/// All methods are optional.
pub trait Dispatcher: fmt::Debug {
    /// The user data type, which will be passed to the dispatcher's methods.
    type UserData: Default;

    /// Print standard message.
    fn print_std(_: String, _: &mut Self::UserData) {}

    /// Print error message.
    fn print_err(_: String, _: &mut Self::UserData) {}

    /// Suggestion to turn on or off context signal processing. The message is called only when the
    /// context's process function is running.
    fn switch_dsp(_: bool, _: &mut Self::UserData) {}

    /// Called when a message for the registered with [Context::register_receiver] receiver is
    /// send.
    fn receiver_message(_name: String, _message: Option<Message>, _: &mut Self::UserData) {}

    /// A referenced object, abstraction or external can't be found in the current context.
    ///
    /// The first argument is the name of the object.
    ///
    /// Optionally, you can return the path to the object definition.
    fn cannot_find_obj(_: String, _: &mut Self::UserData) -> Option<String> {
        None
    }
}

/// Context configuration.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The number of input channels.
    pub input_ch_num: u16,
    /// The number of output channels.
    pub output_ch_num: u16,
    /// The computation block size.
    pub blocksize: u16,
    /// The sample rate.
    pub sample_rate: u32,
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
    pub fn with_in_ch_num(mut self, ch_num: u16) -> Self {
        self.input_ch_num = ch_num;
        self
    }

    /// Set output channels number.
    pub fn with_out_ch_num(mut self, ch_num: u16) -> Self {
        self.output_ch_num = ch_num;
        self
    }

    /// Set computation block size.
    pub fn with_block_size(mut self, blocksize: u16) -> Self {
        self.blocksize = blocksize;
        self
    }

    /// Set sample rate.
    pub fn with_sample_rate(mut self, sr: u32) -> Self {
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

    use std::fs;

    use zengarden_raw::{zg_context_new_graph_from_file, zg_graph_attach};

    use crate::message::MessageElement;

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
        let expected = 42;
        let context =
            Context::<DummyDispatcher, AudioLoopF32>::new(Config::default(), expected).unwrap();
        assert_eq!(expected, *context.user_data());

        let data = context.user_data_mut();
        *data = 27;

        assert_eq!(27, *context.user_data());
    }

    #[test]
    fn context_next_frame_f32() {
        let mut context = init_test_context::<DummyDispatcher, AudioLoopF32>("loop_with_input.pd");

        let input = 0..context.config.blocksize * context.config.input_ch_num * 2;

        let expected: Vec<f32> = input
            .clone()
            .enumerate()
            .map(|(n, val)| {
                let mul = [2_f32, 3.0][n % context.config.input_ch_num as usize];
                val as f32 * mul
            })
            .collect();

        let result: Vec<f32> = input
            .map(|n| n as f32)
            .collect::<Vec<f32>>()
            .chunks(context.config.input_ch_num as usize)
            .map(|val| context.next_frame(val).unwrap().to_owned())
            .flatten()
            .collect();

        // there's a one block delay, so we compare slices of a single block only
        let actual_blocksize = (context.config.blocksize * context.config.input_ch_num) as usize;
        assert_eq!(expected[..actual_blocksize], result[actual_blocksize..]);
    }

    #[test]
    fn context_next_frame_i16() {
        let mut context = init_test_context::<DummyDispatcher, AudioLoopI16>("loop_with_input.pd");

        let input = 0_..(context.config.blocksize * context.config.input_ch_num * 2) as i16;

        let expected: Vec<i16> = input
            .clone()
            .enumerate()
            .map(|(n, val)| {
                let mul = [2_i16, 3][n % context.config.input_ch_num as usize];
                val * mul
            })
            .collect();

        let result: Vec<i16> = input
            .collect::<Vec<i16>>()
            .chunks(context.config.input_ch_num as usize)
            .map(|val| context.next_frame(val).unwrap().to_owned())
            .flatten()
            .collect();

        // there's a one block delay, so we compare slices of a single block only
        let actual_blocksize = (context.config.blocksize * context.config.input_ch_num) as usize;
        assert_eq!(expected[..actual_blocksize], result[actual_blocksize..]);
    }

    #[test]
    fn context_send_message() {
        let mut context = init_test_context::<TestDispatcher, AudioLoopF32>("send_message.pd");
        let message = Message::default()
            .with_element(MessageElement::Symbol("baz".to_string()))
            .build();
        let receiver = "test-send-message-s";
        context.register_receiver(receiver);
        context.send_message("test-send-message-r", message);

        // as all messages are scheduled in ZenGarden, we should process an audio block
        for _ in 0..context.config.blocksize + 1 {
            context.next_frame(&[0.0, 0.0]).unwrap();
        }

        assert_eq!(context.user_data_mut().0, format!("{}.{}", receiver, "baz"));
    }

    fn init_test_context<D: Dispatcher, L: AudioLoop>(file: &str) -> Context<D, L> {
        let context = Context::<D, L>::new(Config::default(), D::UserData::default()).unwrap();
        let patch_dir_path = fs::canonicalize("./test/").unwrap();
        let patch_dir_str = patch_dir_path.to_str().unwrap();

        unsafe {
            let dir = CString::new(patch_dir_str).unwrap();
            let filename = CString::new(format!("/{}", file)).unwrap();
            let graph = zg_context_new_graph_from_file(
                *(context.raw_context.read().unwrap()),
                dir.as_ptr(),
                filename.as_ptr(),
            );
            zg_graph_attach(graph);
        }

        context
    }

    #[test]
    fn dispatcher() {
        let context = Context::<TestDispatcher, AudioLoopF32>::new(
            Config::default(),
            TestUserData(String::new()),
        )
        .unwrap();
        let data_ptr: *mut TestUserData = context.user_data_mut();
        let data_raw = data_ptr as *mut c_void;

        unsafe {
            test_print_std(&context, data_raw);
            test_print_err(&context, data_raw);
            test_switch_dsp(&context, data_raw);
            test_receiver_msg(&context, data_raw);
            test_obj_not_found(&context, data_raw);
        }
    }

    unsafe fn test_print_std(
        context: &'_ Context<TestDispatcher, AudioLoopF32>,
        data: *mut c_void,
    ) {
        let expected = "foo";
        let msg = CString::new(expected).unwrap().into_raw() as *mut c_void;
        let result = Context::<TestDispatcher, AudioLoopF32>::raw_callback(
            ZGCallbackFunction::ZG_PRINT_STD,
            data,
            msg,
        );

        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_print_err(
        context: &'_ Context<TestDispatcher, AudioLoopF32>,
        data: *mut c_void,
    ) {
        let expected = "bar";
        let msg = CString::new(expected).unwrap().into_raw() as *mut c_void;
        let result = Context::<TestDispatcher, AudioLoopF32>::raw_callback(
            ZGCallbackFunction::ZG_PRINT_ERR,
            data,
            msg,
        );
        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_switch_dsp(
        context: &'_ Context<TestDispatcher, AudioLoopF32>,
        data: *mut c_void,
    ) {
        let expected = "true";
        let msg = true as i32 as *mut c_void;
        let result = Context::<TestDispatcher, AudioLoopF32>::raw_callback(
            ZGCallbackFunction::ZG_PD_DSP,
            data,
            msg,
        );
        assert!(result.is_null());
        assert_eq!(expected, context.user_data().0);
    }

    unsafe fn test_receiver_msg(
        context: &'_ Context<TestDispatcher, AudioLoopF32>,
        data: *mut c_void,
    ) {
        let expected = String::from("receiver_name");
        let name = CString::new(expected.as_str()).unwrap().into_raw();
        let msg = Box::into_raw(Box::new(ZGReceiverMessagePair {
            receiverName: name,
            message: ptr::null::<ZGMessage>() as *mut ZGMessage, //TODO
        })) as *mut c_void;
        let result = Context::<TestDispatcher, AudioLoopF32>::raw_callback(
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

    unsafe fn test_obj_not_found(
        context: &'_ Context<TestDispatcher, AudioLoopF32>,
        data: *mut c_void,
    ) {
        let expected = String::from("object_name");
        let name = CString::new(expected.as_str()).unwrap().into_raw() as *mut c_void;
        let result = Context::<TestDispatcher, AudioLoopF32>::raw_callback(
            ZGCallbackFunction::ZG_CANNOT_FIND_OBJECT,
            data,
            name,
        );

        let result_str = CString::from_raw(result as *mut c_char);

        assert_eq!(result_str.to_string_lossy(), expected.clone());
        assert_eq!(expected, context.user_data().0);
    }

    #[derive(Debug)]
    struct TestDispatcher;

    impl Dispatcher for TestDispatcher {
        type UserData = TestUserData;

        fn print_std(message: String, data: &mut Self::UserData) {
            data.0 = message;
        }

        fn print_err(message: String, data: &mut Self::UserData) {
            TestDispatcher::print_std(message, data);
        }

        fn switch_dsp(state: bool, data: &mut Self::UserData) {
            if state {
                data.0 = String::from("true");
            }
        }

        fn receiver_message(name: String, msg: Option<Message>, data: &mut Self::UserData) {
            if let Some(message) = msg {
                let val = match message.element_at(0) {
                    MessageElement::Float(val) => val.to_string(),
                    MessageElement::Symbol(val) => val.to_owned(),
                    _ => "bang".to_string(),
                };
                data.0 = format!("{}.{}", name, val);
            } else {
                data.0 = name;
            }
        }

        fn cannot_find_obj(name: String, data: &mut Self::UserData) -> Option<String> {
            data.0 = name;
            Some(data.0.clone())
        }
    }

    #[derive(Debug, Default)]
    struct TestUserData(String);

    #[derive(Debug)]
    struct DummyDispatcher;

    impl Dispatcher for DummyDispatcher {
        type UserData = u32;
    }
}
