use glam::Vec3;
use steam_audio_sys::ffi::{self, IPLAmbisonicsEncodeEffectParams};

use crate::audio_buffer::{AudioBuffer, AudioBufferFrame};
use crate::context::Context;
use crate::error::SteamAudioError;
use crate::hrtf::AudioSettings;

pub struct AmbisonicsEncodeParams {
    pub direction: Vec3,
    pub order: u8,
}

impl Default for AmbisonicsEncodeParams {
    fn default() -> Self {
        Self {
            direction: glam::Vec3::ZERO,
            order: 1,
        }
    }
}

impl Into<ffi::IPLAmbisonicsEncodeEffectParams> for &AmbisonicsEncodeParams {
    fn into(self) -> ffi::IPLAmbisonicsEncodeEffectParams {
        ffi::IPLAmbisonicsEncodeEffectParams {
            direction: self.direction.into(),
            order: self.order as i32,
        }
    }
}

pub struct AmbisonicsEncode(ffi::IPLAmbisonicsEncodeEffect);

impl AmbisonicsEncode {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        max_order: u8,
    ) -> Result<Self, SteamAudioError> {
        let mut ambisonics_encode = Self(unsafe { std::mem::zeroed() });
        let mut effect_settings = ffi::IPLAmbisonicsEncodeEffectSettings {
            maxOrder: max_order as i32,
        };

        unsafe {
            match ffi::iplAmbisonicsEncodeEffectCreate(
                context.inner(),
                &mut audio_settings.into(),
                &mut effect_settings,
                &mut ambisonics_encode.0,
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(ambisonics_encode),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLAmbisonicsEncodeEffect {
        self.0
    }

    pub fn apply_to_buffer(
        &self,
        params: &AmbisonicsEncodeParams,
        mut frame: AudioBufferFrame,
        output_buffer: &mut AudioBuffer,
    ) -> Result<(), SteamAudioError> {
        assert_eq!(frame.channels(), 1);
        assert_eq!(
            output_buffer.channels(),
            crate::ambisonic_order_channels(params.order)
        );

        let mut output_ffi_buffer = unsafe { output_buffer.ffi_buffer_null() };
        let mut data_ptrs = unsafe { output_buffer.data_ptrs() };
        output_ffi_buffer.data = data_ptrs.as_mut_ptr();

        let mut ipl_params: IPLAmbisonicsEncodeEffectParams = params.into();

        /*
        dbg!(unsafe { frame.0.data });
        dbg!(unsafe { *frame.0.data });
        dbg!(unsafe { output_ffi_buffer.data });
        dbg!(unsafe { *output_ffi_buffer.data });
        dbg!();
        */
        unsafe {
            let _effect_state = ffi::iplAmbisonicsEncodeEffectApply(
                self.inner(),
                &mut ipl_params,
                &mut frame.0,
                &mut output_ffi_buffer,
            );
            //dbg!(_effect_state);
        }

        Ok(())
    }

    pub fn apply(
        &self,
        audio_settings: &AudioSettings,
        params: &AmbisonicsEncodeParams,
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

impl Drop for AmbisonicsEncode {
    fn drop(&mut self) {
        unsafe {
            ffi::iplAmbisonicsEncodeEffectRelease(&mut self.0);
        }
    }
}
