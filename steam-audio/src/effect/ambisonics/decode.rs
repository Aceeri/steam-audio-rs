use glam::Vec3;
use steam_audio_sys::ffi::{self, IPLAmbisonicsDecodeEffectParams};

use crate::Orientation;
use crate::audio_buffer::{AudioBuffer, AudioBufferFrame};
use crate::context::Context;
use crate::error::SteamAudioError;
use crate::hrtf::AudioSettings;
use crate::prelude::HRTF;

// TODO
#[derive(Debug, Clone)]
pub enum SpeakerLayout {
    Mono,
    Stereo,
    Quadraphonic,
    Surround5_1,
    Surround7_1,
    Custom {
        num_speakers: u32,
        speakers: Vec<Vec3>,
    },
}

impl Into<ffi::IPLSpeakerLayout> for SpeakerLayout {
    fn into(self) -> ffi::IPLSpeakerLayout {
        let kind = match self {
            Self::Mono => ffi::IPLSpeakerLayoutType::IPL_SPEAKERLAYOUTTYPE_MONO,
            Self::Stereo => ffi::IPLSpeakerLayoutType::IPL_SPEAKERLAYOUTTYPE_STEREO,
            Self::Quadraphonic => ffi::IPLSpeakerLayoutType::IPL_SPEAKERLAYOUTTYPE_QUADRAPHONIC,
            Self::Surround5_1 => ffi::IPLSpeakerLayoutType::IPL_SPEAKERLAYOUTTYPE_SURROUND_5_1,
            Self::Surround7_1 => ffi::IPLSpeakerLayoutType::IPL_SPEAKERLAYOUTTYPE_SURROUND_7_1,
            Self::Custom { .. } => unimplemented!(),
        };

        ffi::IPLSpeakerLayout {
            type_: kind,
            numSpeakers: 0,
            speakers: std::ptr::null_mut(),
        }
    }
}

pub struct AmbisonicsDecodeSettings {
    pub speaker_layout: SpeakerLayout,
    pub max_order: u8,
}

impl Default for AmbisonicsDecodeSettings {
    fn default() -> Self {
        Self {
            speaker_layout: SpeakerLayout::Stereo,
            max_order: 2,
        }
    }
}

impl AmbisonicsDecodeSettings {
    pub fn merge(&self, hrtf: ffi::IPLHRTF) -> ffi::IPLAmbisonicsDecodeEffectSettings {
        ffi::IPLAmbisonicsDecodeEffectSettings {
            hrtf: hrtf,
            speakerLayout: self.speaker_layout.clone().into(),
            maxOrder: self.max_order as i32,
        }
    }
}

pub struct AmbisonicsDecodeParams {
    order: u8,
    orientation: Orientation,
    binaural: bool,
}

impl Default for AmbisonicsDecodeParams {
    fn default() -> Self {
        Self {
            order: 1,
            orientation: Orientation::default(),
            binaural: true,
        }
    }
}

impl AmbisonicsDecodeParams {
    fn merge(&self, hrtf: ffi::IPLHRTF) -> ffi::IPLAmbisonicsDecodeEffectParams {
        ffi::IPLAmbisonicsDecodeEffectParams {
            order: self.order as i32,
            orientation: self.orientation.clone().into(),
            binaural: self.binaural.into(),
            hrtf: hrtf,
        }
    }
}

pub struct AmbisonicsDecode {
    inner: ffi::IPLAmbisonicsDecodeEffect,
    hrtf: ffi::IPLHRTF,
}

impl AmbisonicsDecode {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        hrtf: &HRTF,
        decode_settings: &AmbisonicsDecodeSettings,
    ) -> Result<Self, SteamAudioError> {
        let mut ambisonics_decode = Self {
            inner: unsafe { std::mem::zeroed() },
            hrtf: unsafe { hrtf.inner() },
        };

        let mut effect_settings = decode_settings.merge(unsafe { hrtf.inner() });

        unsafe {
            match ffi::iplAmbisonicsDecodeEffectCreate(
                context.inner(),
                &mut audio_settings.into(),
                &mut effect_settings,
                &mut ambisonics_decode.inner(),
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(ambisonics_decode),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLAmbisonicsDecodeEffect {
        self.inner
    }

    pub fn apply_to_buffer(
        &self,
        params: &AmbisonicsDecodeParams,
        mut frame: AudioBufferFrame,
        output_buffer: &mut AudioBuffer,
    ) -> Result<(), SteamAudioError> {
        let mut output_ffi_buffer = unsafe { output_buffer.ffi_buffer_null() };
        let mut data_ptrs = unsafe { output_buffer.data_ptrs() };
        output_ffi_buffer.data = data_ptrs.as_mut_ptr();

        let mut ipl_params: IPLAmbisonicsDecodeEffectParams = params.merge(self.hrtf);

        unsafe {
            let _effect_state = ffi::iplAmbisonicsDecodeEffectApply(
                self.inner(),
                &mut ipl_params,
                &mut frame.0,
                &mut output_ffi_buffer,
            );
        }

        Ok(())
    }

    pub fn apply(
        &self,
        audio_settings: &AudioSettings,
        params: &AmbisonicsDecodeParams,
        frame: AudioBufferFrame,
    ) -> Result<AudioBuffer, SteamAudioError> {
        let mut output_buffer = AudioBuffer::frame_buffer_with_channels(
            audio_settings,
            crate::ambisonic_order_channels(params.order),
        );
        self.apply_to_buffer(params, frame, &mut output_buffer)?;
        Ok(output_buffer)
    }
}

impl Drop for AmbisonicsDecode {
    fn drop(&mut self) {
        unsafe {
            ffi::iplAmbisonicsDecodeEffectRelease(&mut self.inner);
        }
    }
}
