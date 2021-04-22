use std::fmt;

use thiserror::Error;
use zengarden_raw::{zg_context_process, zg_context_process_s, PdContext};

/// Audio loop.
pub trait AudioLoop: fmt::Debug + Default {
    /// Audio buffer sample type.
    type SampleType;

    /// Initialize buffers. May behave as re-initializer.
    fn init_buffers(&mut self, blocksize: usize, in_ch_num: usize, out_ch_num: usize);

    /// Returns next frame of [`Self::SampleType`].
    fn next_frame(
        &mut self,
        raw_context: *mut PdContext,
        in_frame: &[Self::SampleType],
    ) -> Result<&[Self::SampleType], Error>;
}

/// [AudioLoop] implementation for 32-bit float sampled buffer.
#[derive(Debug, Default)]
pub struct AudioLoopF32 {
    frame_offset: usize,
    in_ch_num: usize,
    blocksize: usize,
    in_buf: Vec<f32>,
    out_buf: Vec<f32>,
    out_frame: Vec<f32>,
}

impl AudioLoopF32 {
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

impl AudioLoop for AudioLoopF32 {
    type SampleType = f32;

    fn init_buffers(&mut self, blocksize: usize, in_ch_num: usize, out_ch_num: usize) {
        self.in_buf = vec![0.0; blocksize * in_ch_num];
        self.out_buf = vec![0.0; blocksize * out_ch_num];
        self.out_frame = vec![0.0; out_ch_num];
    }

    fn next_frame(
        &mut self,
        raw_context: *mut PdContext,
        in_frame: &[Self::SampleType],
    ) -> Result<&[Self::SampleType], Error> {
        if in_frame.len() != self.in_ch_num {
            return Err(Error::WrongInFrameSize);
        }

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

/// [AudioLoop] implementation for 16-bit integer sampled buffer.
#[derive(Debug, Default)]
pub struct AudioLoopI16 {
    frame_offset: usize,
    in_ch_num: usize,
    blocksize: usize,
    in_buf: Vec<i16>,
    out_buf: Vec<i16>,
    out_frame: Vec<i16>,
}

impl AudioLoopI16 {
    fn process_buffers(&mut self, raw_context: *mut PdContext) {
        unsafe {
            zg_context_process_s(
                raw_context,
                self.in_buf.as_mut_ptr(),
                self.out_buf.as_mut_ptr(),
            );
        }
    }

    fn update_input(&mut self, in_frame: &[i16]) {
        self.in_buf[self.frame_offset..].copy_from_slice(in_frame);
    }

    fn update_output(&mut self) {
        let end = self.out_frame.len();
        self.out_frame.copy_from_slice(&self.out_buf[self.frame_offset..end]);
    }
}

impl AudioLoop for AudioLoopI16 {
    type SampleType = i16;

    fn init_buffers(&mut self, blocksize: usize, in_ch_num: usize, out_ch_num: usize) {
        self.in_buf = vec![0; blocksize * in_ch_num];
        self.out_buf = vec![0; blocksize * out_ch_num];
        self.out_frame = vec![0; out_ch_num];
    }

    fn next_frame(
        &mut self,
        raw_context: *mut PdContext,
        in_frame: &[Self::SampleType],
    ) -> Result<&[Self::SampleType], Error> {
        if in_frame.len() != self.in_ch_num {
            return Err(Error::WrongInFrameSize);
        }

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

/// Audio loop error.
#[derive(Debug, Error)]
pub enum Error {
    /// The size of the input frame isn't equal to the number of the input channels.
    #[error("Wrong input frame size. The size should be equal to the number of input channels.")]
    WrongInFrameSize,
}
