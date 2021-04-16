use std::ffi::{c_void, CString};
use std::ptr;
use std::sync::atomic::AtomicPtr;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

use zengarden_raw::*;

struct Context(AtomicPtr<PdContext>);

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

unsafe extern "C" fn callback(
    msg: ZGCallbackFunction,
    _: *mut c_void,
    ptr: *mut c_void,
) -> *mut c_void {
    match msg {
        ZGCallbackFunction::ZG_PRINT_STD => {
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

struct Loop {
    offset: usize,
    blocksize: usize,
    in_buf: Vec<f32>,
    out_buf: Vec<f32>,
    frame: Vec<f32>,
    context: Context,
}

impl Loop {
    fn new(context: Context, blocksize: usize, ch_num: usize) -> Self {
        Self {
            offset: 0,
            blocksize,
            in_buf: vec![0.0; blocksize * ch_num],
            out_buf: vec![0.0; blocksize * ch_num],
            frame: vec![0.0; ch_num],
            context,
        }
    }

    fn next_frame(&mut self) -> &[f32] {
        if self.offset == self.blocksize {
            self.fill_buffers();
            self.offset = 0;
        }

        self.fill_frame();

        self.offset += 1;

        &self.frame
    }

    fn fill_buffers(&mut self) {
        unsafe {
            zg_context_process(
                *self.context.0.get_mut(),
                self.in_buf.as_mut_ptr(),
                self.out_buf.as_mut_ptr(),
            );
        }
    }

    fn fill_frame(&mut self) {
        for n in 0..self.frame.len() {
            let buffer_pos = n * self.blocksize + self.offset;
            self.frame[n] = self.out_buf[buffer_pos];
        }
    }
}

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let config: StreamConfig = configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate()
        .into();

    let ch_num = config.channels as i32;
    let sr = config.sample_rate.0 as f32;
    let blocksize: usize = 64;
    let mut context: Context;

    unsafe {
        context = Context(AtomicPtr::new(zg_context_new(
            ch_num,
            ch_num,
            blocksize as i32,
            sr,
            Some(callback),
            ptr::null::<c_void>() as *mut _,
        )));
        let dir = CString::new("/Users/alestsurko/Desktop/miller/").unwrap();
        let filename = CString::new("test.pd").unwrap();
        let graph = zg_context_new_graph_from_file(
            *context.0.get_mut(),
            dir.as_ptr(),
            filename.as_ptr(),
        );
        zg_graph_attach(graph);
    }

    let mut audio_loop = Loop::new(context, blocksize, ch_num as usize);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut offset = 0;

                while offset < data.len() {
                    let frame = audio_loop.next_frame();
                    let end = offset + frame.len();
                    data[offset..end].copy_from_slice(frame);
                    offset = end;
                }
            },
            move |err| {
                eprintln!("Audio I/O error: {}", err);
            },
        )
        .unwrap();

    stream.play().unwrap();

    loop {}
}
