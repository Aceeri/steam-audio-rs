use std::ffi::CString;

use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Copy, Clone)]
pub enum HRTFInterpolation {
    NearestNeighbor,
    Bilinear,
}

impl Into<ffi::IPLHRTFInterpolation> for HRTFInterpolation {
    fn into(self) -> ffi::IPLHRTFInterpolation {
        match self {
            Self::NearestNeighbor => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_NEAREST,
            Self::Bilinear => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
        }
    }
}

pub enum HRTFSettings {
    Default,
    SOFA(String),
}

impl Default for HRTFSettings {
    fn default() -> Self {
        Self::Default
    }
}

impl Into<ffi::IPLHRTFSettings> for &HRTFSettings {
    fn into(self) -> ffi::IPLHRTFSettings {
        let (type_, path) = match self {
            HRTFSettings::Default => (ffi::IPLHRTFType::IPL_HRTFTYPE_DEFAULT, String::new()),
            HRTFSettings::SOFA(path) => (ffi::IPLHRTFType::IPL_HRTFTYPE_SOFA, path.clone()),
        };

        let cstring = CString::new(path).expect("interior nul byte in path");

        ffi::IPLHRTFSettings {
            type_: type_,
            sofaFileName: cstring.as_ptr(),
        }
    }
}

pub struct AudioSettings {
    sampling_rate: u32,
    frame_size: u32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            sampling_rate: 44100,
            frame_size: 1024,
        }
    }
}

impl Into<ffi::IPLAudioSettings> for &AudioSettings {
    fn into(self) -> ffi::IPLAudioSettings {
        ffi::IPLAudioSettings {
            samplingRate: self.sampling_rate as i32,
            frameSize: self.frame_size as i32,
        }
    }
}

impl AudioSettings {
    pub fn sampling_rate(&self) -> u32 {
        self.sampling_rate
    }

    pub fn frame_size(&self) -> u32 {
        self.frame_size
    }
}

pub struct HRTF(ffi::IPLHRTF);

impl HRTF {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        hrtf_settings: &HRTFSettings,
    ) -> Result<Self, SteamAudioError> {
        let mut hrtf = Self(unsafe { std::mem::zeroed() });
        let mut audio_ipl_settings: ffi::IPLAudioSettings = audio_settings.into();
        let mut hrtf_ipl_settings: ffi::IPLHRTFSettings = hrtf_settings.into();

        unsafe {
            match ffi::iplHRTFCreate(
                context.inner(),
                &mut audio_ipl_settings,
                &mut hrtf_ipl_settings,
                &mut hrtf.0,
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(hrtf),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLHRTF {
        self.0
    }
}

impl Drop for HRTF {
    fn drop(&mut self) {
        unsafe {
            ffi::iplHRTFRelease(&mut self.0);
        }
    }
}
