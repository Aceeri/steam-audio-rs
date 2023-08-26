use steam_audio_sys::ffi;

use bitflags::bitflags;

use crate::prelude::*;

bitflags! {
    pub struct DirectSimulationFlags: i32 {
        const DISTANCE_ATTENUATION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_DISTANCEATTENUATION.0;
        const AIR_ABSORPTION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_AIRABSORPTION.0;
        const DIRECTIVITY = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_DIRECTIVITY.0;
        const OCCLUSION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_OCCLUSION.0;
        const TRANSMISSION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_TRANSMISSION.0;
    }
}

impl Default for DirectSimulationFlags {
    fn default() -> Self {
        Self::DISTANCE_ATTENUATION
    }
}

impl Into<ffi::IPLDirectSimulationFlags> for DirectSimulationFlags {
    fn into(self) -> ffi::IPLDirectSimulationFlags {
        ffi::IPLDirectSimulationFlags(self.bits())
    }
}

bitflags! {
    /// What should be applied to the sound.
    pub struct DirectEffectFlags: i32 {
        const DISTANCE_ATTENUATION = ffi::IPLDirectEffectFlags::IPL_DIRECTEFFECTFLAGS_APPLYDISTANCEATTENUATION.0;
        const AIR_ABSORPTION = ffi::IPLDirectEffectFlags::IPL_DIRECTEFFECTFLAGS_APPLYAIRABSORPTION.0;
        const DIRECTIVITY = ffi::IPLDirectEffectFlags::IPL_DIRECTEFFECTFLAGS_APPLYDIRECTIVITY.0;
        const OCCLUSION = ffi::IPLDirectEffectFlags::IPL_DIRECTEFFECTFLAGS_APPLYOCCLUSION.0;
        const TRANSMISSION = ffi::IPLDirectEffectFlags::IPL_DIRECTEFFECTFLAGS_APPLYTRANSMISSION.0;
    }
}

impl Default for DirectEffectFlags {
    fn default() -> Self {
        Self::DISTANCE_ATTENUATION
    }
}

impl Into<ffi::IPLDirectEffectFlags> for DirectEffectFlags {
    fn into(self) -> ffi::IPLDirectEffectFlags {
        ffi::IPLDirectEffectFlags(self.bits())
    }
}

impl From<ffi::IPLDirectEffectFlags> for DirectEffectFlags {
    fn from(other: ffi::IPLDirectEffectFlags) -> Self {
        Self { bits: other.0 }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TransmissionType {
    FrequencyIndependent,
    FrequencyDependent,
}

impl Default for TransmissionType {
    fn default() -> Self {
        Self::FrequencyIndependent
    }
}

impl Into<ffi::IPLTransmissionType> for TransmissionType {
    fn into(self) -> ffi::IPLTransmissionType {
        match self {
            Self::FrequencyIndependent => {
                ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQINDEPENDENT
            }
            Self::FrequencyDependent => {
                ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQDEPENDENT
            }
        }
    }
}

impl From<ffi::IPLTransmissionType> for TransmissionType {
    fn from(other: ffi::IPLTransmissionType) -> Self {
        match other {
            ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQINDEPENDENT => {
                Self::FrequencyIndependent
            }
            ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQDEPENDENT => {
                Self::FrequencyDependent
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectEffectParams {
    pub flags: DirectEffectFlags,
    pub transmission_type: TransmissionType,

    pub distance_attenuation: f32,
    pub directivity: f32,
    pub occlusion: f32,

    pub air_absorption: [f32; 3], // 3-band EQ coefficients
    pub transmission: [f32; 3],
}

impl Default for DirectEffectParams {
    fn default() -> Self {
        Self {
            flags: DirectEffectFlags::default(),
            transmission_type: TransmissionType::default(),

            distance_attenuation: 1.0,
            directivity: 1.0,
            occlusion: 1.0,

            air_absorption: [1.0, 1.0, 1.0],
            transmission: [1.0, 1.0, 1.0],
        }
    }
}

impl Into<ffi::IPLDirectEffectParams> for &DirectEffectParams {
    fn into(self) -> ffi::IPLDirectEffectParams {
        ffi::IPLDirectEffectParams {
            flags: self.flags.into(),
            transmissionType: self.transmission_type.into(),

            distanceAttenuation: self.distance_attenuation,
            directivity: self.directivity,
            occlusion: self.occlusion,

            airAbsorption: self.air_absorption,
            transmission: self.transmission,
        }
    }
}

impl From<ffi::IPLDirectEffectParams> for DirectEffectParams {
    fn from(other: ffi::IPLDirectEffectParams) -> Self {
        Self {
            flags: other.flags.into(),
            transmission_type: other.transmissionType.into(),

            distance_attenuation: other.distanceAttenuation,
            directivity: other.directivity,
            occlusion: other.occlusion,

            air_absorption: other.airAbsorption,
            transmission: other.transmission,
        }
    }
}

pub struct DirectEffect {
    inner: ffi::IPLDirectEffect,
    channels: u16,
}

unsafe impl Send for DirectEffect {}
unsafe impl Sync for DirectEffect {}

impl crate::SteamAudioObject for DirectEffect {
    type Object = ffi::IPLDirectEffect;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> *mut Self::Object {
        std::ptr::addr_of_mut!(self.inner)
    }
}

impl DirectEffect {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        num_channels: u16,
    ) -> Result<Self, SteamAudioError> {
        let mut effect = Self {
            inner: std::ptr::null_mut(),
            channels: num_channels,
        };

        let mut effect_settings = ffi::IPLDirectEffectSettings {
            numChannels: num_channels as i32,
        };

        unsafe {
            match ffi::iplDirectEffectCreate(
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
        params: &DirectEffectParams,
        mut frame: DeinterleavedFrame,
        output_buffer: &mut DeinterleavedFrame,
    ) -> Result<(), SteamAudioError> {
        assert_eq!(frame.channels(), self.channels);
        assert_eq!(output_buffer.channels(), self.channels);

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

        let mut ipl_params: ffi::IPLDirectEffectParams = params.into();

        unsafe {
            let _effect_state = ffi::iplDirectEffectApply(
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
        params: &DirectEffectParams,
        frame: DeinterleavedFrame,
    ) -> Result<DeinterleavedFrame, SteamAudioError> {
        let mut output_buffer = DeinterleavedFrame::new(
            audio_settings.frame_size() as usize,
            self.channels as u16,
            audio_settings.sampling_rate(),
        );
        self.apply_to_buffer(params, frame, &mut output_buffer)?;
        Ok(output_buffer)
    }
}

impl Drop for DirectEffect {
    fn drop(&mut self) {
        unsafe {
            ffi::iplDirectEffectRelease(self.inner_mut());
        }
    }
}
