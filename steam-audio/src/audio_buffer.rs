
use steam_audio_sys::ffi;

use crate::prelude::AudioSettings;

pub struct AudioBuffer {
    data: Vec<Vec<f32>>,
    frame_size: u32,
    inner: ffi::IPLAudioBuffer,
}

impl AudioBuffer {
    pub fn empty(settings: &AudioSettings, data: Vec<Vec<f32>>) -> Self {
        AudioBuffer {
            data: Vec::new(),
            frame_size: settings.frame_size(),
            inner: unsafe { std::mem::zeroed() }
        }
    }

    pub fn from_raw_pcm(settings: &AudioSettings, data: Vec<Vec<f32>>) -> Self {
        AudioBuffer {
            data: data,
            frame_size: settings.frame_size(),
            inner: unsafe { std::mem::zeroed() }
        }
    }
}