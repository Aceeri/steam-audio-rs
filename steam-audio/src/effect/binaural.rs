/*
let mut effect_params = IPLBinauralEffectParams {
    direction: direction,
    hrtf: hrtf,
    interpolation: IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
    spatialBlend: 1.0,
};

println!("{:?}", frame);
dbg!(&input.buffer);
dbg!(&output_buffer);

let mut output_audio_frame: Vec<f32> =
    vec![0.0; (2 * audio_settings.frameSize) as usize];

unsafe {
    let effect = iplBinauralEffectApply(
        binaural,
        &mut effect_params,
        &mut input.buffer,
        &mut output_buffer,
    );
    iplAudioBufferInterleave(
        context,
        &mut output_buffer,
        output_audio_frame.as_mut_ptr(),
    );
}
*/

use steam_audio_sys::ffi;

use crate::{
    error::SteamAudioError,
    prelude::{AudioBuffer, AudioSettings, Context, HRTF}, audio_buffer::{AudioBufferIterator, AudioBufferFrame},
};

#[derive(Copy, Clone)]
pub enum HRTFInterpolation {
    NearestNeighbor,
    Bilinear,
}

impl Into<ffi::IPLHRTFInterpolation> for HRTFInterpolation {
    fn into(self) -> ffi::IPLHRTFInterpolation {
        match self {
            Self::NearestNeighbor => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
            Self::Bilinear => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
        }
    }
}

pub struct BinauralParams {
    pub direction: glam::Vec3,
    pub interpolation: HRTFInterpolation,
    pub spatial_blend: f32,
}

impl Default for BinauralParams {
    fn default() -> Self {
        Self {
            direction: glam::Vec3::ZERO,
            interpolation: HRTFInterpolation::NearestNeighbor,
            spatial_blend: 1.0,
        }
    }
}

impl BinauralParams {
    pub fn merge(&self, hrtf: ffi::IPLHRTF) -> ffi::IPLBinauralEffectParams {
        ffi::IPLBinauralEffectParams {
            direction: self.direction.into(),
            hrtf: hrtf,
            interpolation: self.interpolation.into(),
            spatialBlend: self.spatial_blend,
        }
    }
}

pub struct BinauralEffect {
    inner: ffi::IPLBinauralEffect,
    hrtf: ffi::IPLHRTF,
}

impl BinauralEffect {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        hrtf: &HRTF,
    ) -> Result<Self, SteamAudioError> {
        let mut effect = Self {
            inner: unsafe { std::mem::zeroed() },
            hrtf: unsafe { hrtf.inner() },
        };

        let mut effect_settings = ffi::IPLBinauralEffectSettings {
            hrtf: unsafe { hrtf.inner() },
        };

        unsafe {
            match ffi::iplBinauralEffectCreate(
                context.inner(),
                &mut audio_settings.into(),
                &mut effect_settings,
                &mut effect.inner,
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(effect),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLBinauralEffect {
        self.inner
    }

    pub fn apply_step_with_buffer(&self, audio_settings: &AudioSettings, params: &BinauralParams, mut frame: AudioBufferFrame, output_buffer: &mut AudioBuffer) -> Result<(), SteamAudioError> {
        let mut output_ffi_buffer = unsafe { output_buffer.ffi_buffer_null() };
        let mut data_ptrs = unsafe { output_buffer.data_ptrs() };
        output_ffi_buffer.data = data_ptrs.as_mut_ptr();
        dbg!(output_ffi_buffer);

        let mut ipl_params = params.merge(self.hrtf);

        dbg!();

        unsafe {
        dbg!();
            let effect = ffi::iplBinauralEffectApply(
                self.inner,
                &mut ipl_params,
                &mut frame.0,
                &mut output_ffi_buffer,
            );
        dbg!();
        }

        Ok(())
    }

    pub fn apply_step(&self, audio_settings: &AudioSettings, params: &BinauralParams, mut frame: AudioBufferFrame) -> Result<AudioBuffer, SteamAudioError> {
        let mut output_buffer = AudioBuffer::frame_buffer_with_channels(audio_settings, 2);
        self.apply_step_with_buffer(audio_settings, params, frame, &mut output_buffer)?;
        dbg!();
        Ok(output_buffer)
    }
}

impl Drop for BinauralEffect {
    fn drop(&mut self) {
        unsafe {
            ffi::iplBinauralEffectRelease(&mut self.inner);
        }
    }
}
