use steam_audio_sys::ffi;

use crate::prelude::*;

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

    pub fn apply_to_buffer(
        &self,
        params: &BinauralParams,
        mut frame: AudioBufferFrame,
        output_buffer: &mut AudioBuffer,
    ) -> Result<(), SteamAudioError> {
        assert_eq!(frame.channels(), 1);
        assert_eq!(output_buffer.channels(), 2);

        let mut output_ffi_buffer = unsafe { output_buffer.ffi_buffer_null() };
        let mut data_ptrs = unsafe { output_buffer.data_ptrs() };
        output_ffi_buffer.data = data_ptrs.as_mut_ptr();

        let mut ipl_params = params.merge(self.hrtf);

        unsafe {
            let _effect_state = ffi::iplBinauralEffectApply(
                self.inner,
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
        params: &BinauralParams,
        frame: AudioBufferFrame,
    ) -> Result<AudioBuffer, SteamAudioError> {
        let mut output_buffer = AudioBuffer::frame_buffer_with_channels(audio_settings, 2);
        self.apply_to_buffer(params, frame, &mut output_buffer)?;
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
