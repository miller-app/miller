use std::fmt;

use thiserror::Error;
use zengarden_raw::{zg_context_process, zg_context_process_s, PdContext};

pub(crate) trait AudioLoop: fmt::Debug {
    type SampleType;

    fn next_frame(
        &mut self,
        raw_context: *mut PdContext,
        in_frame: &[Self::SampleType],
    ) -> Result<&[Self::SampleType], Error>;
}

/// [AudioLoop] implementation for 32-bit float sampled buffer.
#[derive(Debug)]
pub(crate) struct AudioLoopF {
    frame_offset: usize,
    blocksize: usize,
    in_buf: Vec<f32>,
    out_buf: Vec<f32>,
    out_frame: Vec<f32>,
}

impl AudioLoopF {
    pub(crate) fn new(blocksize: usize, in_ch_num: usize, out_ch_num: usize) -> Self {
        Self {
            frame_offset: 0,
            blocksize,
            in_buf: vec![0.0; blocksize * in_ch_num],
            out_buf: vec![0.0; blocksize * out_ch_num],
            out_frame: vec![0.0; out_ch_num],
        }
    }

    fn process_buffers(&mut self, raw_context: *mut PdContext) {
        unsafe {
            zg_context_process(
                raw_context,
                self.in_buf.as_mut_ptr(),
                self.out_buf.as_mut_ptr(),
            );
        }
    }

    fn update_input(&mut self, in_frame: &[f32]) {
        for n in 0..self.in_buf.len() {
            let pos = n * self.blocksize + self.frame_offset;
            self.in_buf[pos] = in_frame[n];
        }
    }

    fn update_output(&mut self) {
        for n in 0..self.out_frame.len() {
            let buffer_pos = n * self.blocksize + self.frame_offset;
            self.out_frame[n] = self.out_buf[buffer_pos];
        }
    }
}

impl AudioLoop for AudioLoopF {
    type SampleType = f32;

    fn next_frame(
        &mut self,
        raw_context: *mut PdContext,
        in_frame: &[Self::SampleType],
    ) -> Result<&[Self::SampleType], Error> {
        if self.frame_offset == self.blocksize {
            self.process_buffers(raw_context);
            self.frame_offset = 0;
        }

        self.update_input(in_frame);
        self.update_output();

        self.frame_offset += 1;

        Ok(&self.out_frame)
    }
}

/// Sample type.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Sample {
    /// 32-bit float sample type.
    Float32(f32),
    /// 16-bit integer (short) type.
    Int16(i16),
}

/// Audio loop error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong input buffer size. The size should be equal to the number of input channels.")]
    WrongInBufferSize,
}
