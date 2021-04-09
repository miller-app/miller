use std::ffi::{c_void, CString};
use std::ptr;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

use zengarden_raw::*;

struct Context(*mut PdContext);

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
    let context: Context;

    unsafe {
        context = Context(zg_context_new(
            ch_num,
            ch_num,
            blocksize as i32,
            sr * ch_num as f32,
            Some(callback),
            ptr::null::<c_void>() as *mut _,
        ));
        let dir = CString::new("/Users/alestsurko/Desktop/miller/").unwrap();
        let filename = CString::new("test.pd").unwrap();
        let graph = zg_context_new_graph_from_file(context.0, dir.as_ptr(), filename.as_ptr());
        zg_graph_attach(graph);
    }

    let mut dropped = vec![0.0f32; blocksize];
    let mut offset = 0;

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let input = &mut [];

                if offset > 0 {
                    let start = blocksize - offset;
                    data[..offset].copy_from_slice(&mut dropped[start..]);
                    offset = data.len() - offset;
                } else {
                    offset = data.len();
                }

                unsafe {
                    while offset >= blocksize {
                        let start = data.len() - offset;
                        zg_context_process(
                            context.0,
                            input.as_mut_ptr(),
                            data[start..].as_mut_ptr(),
                        );
                        offset -= blocksize;
                    }

                    if offset > 0 {
                        zg_context_process(
                            context.0,
                            input.as_mut_ptr(),
                            dropped.as_mut_slice().as_mut_ptr(),
                        );

                        let start = data.len() - offset;
                        data[start..].copy_from_slice(&mut dropped[..offset]);
                        offset = blocksize - offset;
                    }
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
