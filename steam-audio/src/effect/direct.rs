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

        const ALL = DirectSimulationFlags::DISTANCE_ATTENUATION.bits()
            | DirectSimulationFlags::AIR_ABSORPTION.bits()
            | DirectSimulationFlags::DIRECTIVITY.bits()
            | DirectSimulationFlags::OCCLUSION.bits()
            | DirectSimulationFlags::TRANSMISSION.bits();
        const DEFAULT = DirectSimulationFlags::DISTANCE_ATTENUATION.bits();
    }
}

impl Default for DirectSimulationFlags {
    fn default() -> Self {
        Self::DEFAULT
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

        const ALL = DirectEffectFlags::DISTANCE_ATTENUATION.bits()
            | DirectEffectFlags::AIR_ABSORPTION.bits()
            | DirectEffectFlags::DIRECTIVITY.bits()
            | DirectEffectFlags::OCCLUSION.bits()
            | DirectEffectFlags::TRANSMISSION.bits();
        const DEFAULT = DirectEffectFlags::DISTANCE_ATTENUATION.bits()
            | DirectEffectFlags::AIR_ABSORPTION.bits();
    }
}

impl Default for DirectEffectFlags {
    fn default() -> Self {
        Self::DEFAULT
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
        Self::FrequencyDependent
    }
}

impl Into<ffi::IPLTransmissionType> for TransmissionType {
    fn into(self) -> ffi::IPLTransmissionType {
        match self {
            Self::FrequencyIndependent => {
                ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQINDEPENDENT
            }
            Self::FrequencyDependent => {
                ffi::IPLTransmissionType::IPL_TRANSMISSIONTYPE_FREQINDEPENDENT
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
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DirectEffectParams {
    flags: DirectEffectFlags,
    transmission_type: TransmissionType,

    distance_attenuation: f32,
    directivity: f32,
    occlusion: f32,

    air_absorption: [f32; 3], // 3-band EQ coefficients
    transmission: [f32; 3],
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

pub struct DirectEffect(ffi::IPLDirectEffect);

impl DirectEffect {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        num_channels: u32,
    ) -> Result<Self, SteamAudioError> {
        let mut effect = Self(unsafe { std::mem::zeroed() });

        let mut effect_settings = ffi::IPLDirectEffectSettings {
            numChannels: num_channels as i32,
        };

        unsafe {
            match ffi::iplDirectEffectCreate(
                context.inner(),
                &mut audio_settings.into(),
                &mut effect_settings,
                &mut effect.inner(),
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(effect),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLDirectEffect {
        self.0
    }

    pub fn apply_to_buffer(
        &self,
        params: &DirectEffectParams,
        mut frame: AudioBufferFrame,
        output_buffer: &mut AudioBuffer,
    ) -> Result<(), SteamAudioError> {
        assert_eq!(frame.channels(), 1);
        assert_eq!(output_buffer.channels(), 2);

        let mut output_ffi_buffer = unsafe { output_buffer.ffi_buffer_null() };
        let mut data_ptrs = unsafe { output_buffer.data_ptrs() };
        output_ffi_buffer.data = data_ptrs.as_mut_ptr();

        let mut ipl_params: ffi::IPLDirectEffectParams = params.into();

        unsafe {
            let _effect_state = ffi::iplDirectEffectApply(
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
        params: &DirectEffectParams,
        frame: AudioBufferFrame,
    ) -> Result<AudioBuffer, SteamAudioError> {
        let mut output_buffer = AudioBuffer::frame_buffer_with_channels(audio_settings, 2);
        self.apply_to_buffer(params, frame, &mut output_buffer)?;
        Ok(output_buffer)
    }
}

impl Drop for DirectEffect {
    fn drop(&mut self) {
        unsafe {
            ffi::iplDirectEffectRelease(&mut self.inner());
        }
    }
}