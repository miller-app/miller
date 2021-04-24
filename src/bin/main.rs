use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    StreamConfig,
};

use miller_engine::context::{AudioLoopF32, Dispatcher, Config as ContextConfig, Context};

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

    let context_config = ContextConfig::default()
        .with_sample_rate(config.sample_rate.0)
        .with_in_ch_num(config.channels)
        .with_out_ch_num(config.channels);

    let mut context = Context::<ContextDispatcher, AudioLoopF32>::new(context_config, 0).unwrap();

    // unsafe {
        // let dir = CString::new("/Users/alestsurko/Desktop/miller/").unwrap();
        // let filename = CString::new("test.pd").unwrap();
        // let graph = zg_context_new_graph_from_file(
            // *(context.raw_context.clone().read().unwrap()),
            // dir.as_ptr(),
            // filename.as_ptr(),
        // );
        // zg_graph_attach(graph);
    // }

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut offset = 0;

                while offset < data.len() {
                    let frame = context.next_frame(&[0.0, 0.0]).unwrap();
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

#[derive(Debug)]
struct ContextDispatcher;

impl Dispatcher for ContextDispatcher {
    type UserData = i32;
}
