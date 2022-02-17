use steam_audio_sys::ffi;

use crate::prelude::AudioSettings;

pub struct AudioBuffer {
    data: Vec<Vec<f32>>,
    frame_size: usize,
}

impl AudioBuffer {
    pub fn empty(settings: &AudioSettings) -> Self {
        AudioBuffer {
            data: Vec::new(),
            frame_size: settings.frame_size() as usize,
        }
    }

    pub fn from_raw_pcm(settings: &AudioSettings, mut data: Vec<Vec<f32>>) -> Self {
        AudioBuffer {
            data: data,
            frame_size: settings.frame_size() as usize,
        }
    }

    pub fn frames(&self) -> usize {
        self.data.get(0).unwrap_or(&Vec::new()).len() / self.frame_size
    }

    pub fn channels(&self) -> usize {
        self.data.len()
    }

    pub unsafe fn data_ptrs(&self) -> Vec<*mut f32> {
        self.data.iter().map(|v| v.as_ptr() as *mut _).collect()
    }

    pub unsafe fn ffi_buffer(&self) -> ffi::IPLAudioBuffer {
        ffi::IPLAudioBuffer {
            numChannels: self.channels() as i32,
            numSamples: self.frame_size as i32,
            data: std::ptr::null_mut(),
        }
    }
}

impl<'a> IntoIterator for &'a AudioBuffer {
    type Item = usize;
    type IntoIter = AudioBufferIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

pub struct AudioBufferIterator<'a> {
    buffer: &'a AudioBuffer,
    ptrs: Vec<*mut f32>,
    current: Option<usize>,
    frames: usize,
    pub(crate) inner: ffi::IPLAudioBuffer,
}

impl<'a> AudioBufferIterator<'a> {
    pub fn new(buffer: &'a AudioBuffer) -> Self {
        let mut ipl_buffer = unsafe { buffer.ffi_buffer() };
        let mut ptrs = unsafe { buffer.data_ptrs() };
        ipl_buffer.data = ptrs.as_mut_ptr();

        Self {
            buffer: buffer,
            ptrs: ptrs,
            current: None,
            frames: buffer.frames(),
            inner: ipl_buffer,
        }
    }
}

impl<'a> Iterator for AudioBufferIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.current {
            if index < self.frames {
                // Move the pointers ahead 1 frame size.
                unsafe {
                    for ptr in &mut self.ptrs {
                        *ptr = ptr.offset(self.buffer.frame_size as isize);
                    }
                }

                self.current = Some(index + 1);
                self.current
            } else {
                None
            }
        } else {
            self.current = Some(0);
            self.current
        }
    }
}
