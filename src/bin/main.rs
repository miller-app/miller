use std::ffi::CString;

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

use libpd_sys::{
    libpd_add_float, libpd_blocksize, libpd_finish_message, libpd_init, libpd_init_audio,
    libpd_openfile, libpd_process_float, libpd_start_message,
};

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
    let sr = config.sample_rate.0 as i32;
    let pd_blocksize: usize;

    unsafe {
        libpd_init();
        libpd_init_audio(2, ch_num, sr);

        let filename = CString::new("test.pd").unwrap();
        let dir = CString::new(".").unwrap();
        libpd_openfile(filename.as_ptr(), dir.as_ptr());

        let receiver = CString::new("pd").unwrap();
        let message = CString::new("dsp").unwrap();
        libpd_start_message(1);
        libpd_add_float(1.0);
        libpd_finish_message(receiver.as_ptr(), message.as_ptr());

        pd_blocksize = (libpd_blocksize() * ch_num) as usize;
    }

    let mut dropped = vec![0.0f32; pd_blocksize];
    let mut offset = 0;

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let input = &[];

                if offset > 0 {
                    let start = pd_blocksize - offset;
                    data[..offset].copy_from_slice(&mut dropped[start..]);
                    offset = data.len() - offset;
                } else {
                    offset = data.len();
                }

                unsafe {
                    while offset >= pd_blocksize {
                        let start = data.len() - offset;
                        libpd_process_float(1, input.as_ptr(), data[start..].as_mut_ptr());
                        offset -= pd_blocksize;
                    }

                    if offset > 0 {
                        libpd_process_float(1, input.as_ptr(), dropped.as_mut_slice().as_mut_ptr());

                        let start = data.len() - offset;
                        data[start..].copy_from_slice(&mut dropped[..offset]);
                        offset = pd_blocksize - offset;
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
