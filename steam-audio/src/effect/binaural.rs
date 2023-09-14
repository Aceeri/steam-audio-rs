use steam_audio_sys::ffi;

use crate::prelude::*;

pub struct BinauralParams {
    pub direction: [f32; 3],
    pub interpolation: HRTFInterpolation,
    pub spatial_blend: f32,
}

impl Default for BinauralParams {
    fn default() -> Self {
        Self {
            direction: [0.0; 3],
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
            peakDelays: std::ptr::null_mut(),
        }
    }
}

pub struct BinauralEffect {
    inner: ffi::IPLBinauralEffect,
    hrtf: ffi::IPLHRTF,
}

unsafe impl Send for BinauralEffect {}
unsafe impl Sync for BinauralEffect {}

impl crate::SteamAudioObject for BinauralEffect {
    type Object = ffi::IPLBinauralEffect;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> *mut Self::Object {
        std::ptr::addr_of_mut!(self.inner)
    }
}

impl BinauralEffect {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        hrtf: &HRTF,
    ) -> Result<Self, SteamAudioError> {
        let mut effect = Self {
            inner: std::ptr::null_mut(),
            hrtf: hrtf.inner_raw(),
        };

        let mut effect_settings = ffi::IPLBinauralEffectSettings {
            hrtf: hrtf.inner_raw(),
        };

        unsafe {
            match ffi::iplBinauralEffectCreate(
                context.inner_raw(),
                &mut audio_settings.into(),
                &mut effect_settings,
                effect.inner_mut(),
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(effect),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub fn apply_to_buffer(
        &self,
        params: &BinauralParams,
        frame: &mut DeinterleavedFrame,
        output_buffer: &mut DeinterleavedFrame,
    ) -> Result<(), SteamAudioError> {
        assert_eq!(frame.channels(), 1);
        assert_eq!(output_buffer.channels(), 2);

        let mut input_ffi_buffer = ffi::IPLAudioBuffer {
            numChannels: frame.channels() as i32,
            numSamples: frame.frame_size() as i32,
            data: unsafe { frame.ptrs() },
        };

        let mut output_ffi_buffer = ffi::IPLAudioBuffer {
            numChannels: output_buffer.channels() as i32,
            numSamples: output_buffer.frame_size() as i32,
            data: unsafe { output_buffer.ptrs() },
        };

        let mut ipl_params = params.merge(self.hrtf);

        unsafe {
            let _effect_state = ffi::iplBinauralEffectApply(
                self.inner_raw(),
                &mut ipl_params,
                &mut input_ffi_buffer,
                &mut output_ffi_buffer,
            );
        }

        Ok(())
    }

    pub fn apply(
        &self,
        audio_settings: &AudioSettings,
        params: &BinauralParams,
        frame: &mut DeinterleavedFrame,
    ) -> Result<DeinterleavedFrame, SteamAudioError> {
        let mut output_buffer = DeinterleavedFrame::new(
            audio_settings.frame_size() as usize,
            2,
            audio_settings.sampling_rate(),
        );
        self.apply_to_buffer(params, frame, &mut output_buffer)?;
        Ok(output_buffer)
    }
}

impl Drop for BinauralEffect {
    fn drop(&mut self) {
        unsafe {
            ffi::iplBinauralEffectRelease(self.inner_mut());
        }
    }
}
