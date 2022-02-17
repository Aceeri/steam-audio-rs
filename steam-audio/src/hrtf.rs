
use std::{path::PathBuf, ffi::CString};

use crate::context::Context;

use steam_audio_sys::ffi;

use crate::{refcount::RefCounted, error::SteamAudioError};

pub enum HRTFSettings {
    Default,
    SOFA(PathBuf),
}

impl Into<ffi::IPLHRTFSettings> for HRTFSettings {
    fn into(self) -> ffi::IPLHRTFSettings {
        let (type_, path) = match self {
            HRTFSettings::Default => (ffi::IPLHRTFType::IPL_HRTFTYPE_DEFAULT, PathBuf::new()),
            HRTFSettings::SOFA(path) => (ffi::IPLHRTFType::IPL_HRTFTYPE_SOFA, path),
        };

        let cstring = CString::new(path).expect("interior nul byte in path");

        ffi::IPLHRTFSettings {
            type_: type_,
            sofaFileName: cstring.as_ptr(),
        }
    }
}

pub struct AudioSettings {

}

pub struct HRTF(ffi::IPLHRTF);

impl HRTF {
    fn new(context: Context, audio_settings: AudioSettings, hrtf_settings: HRTFSettings) -> Result<Self, SteamAudioError> {
        let mut hrtf = Self(unsafe { std::mem::zeroed() });
        let mut audio_ipl_settings: ffi::IPLAudioSettings = audio_settings.into();
        let mut hrtf_ipl_settings: ffi::IPLHRTFSettings = hrtf_settings.into();

        unsafe {
            match ffi::iplHRTFCreate(context.0, &mut audio_ipl_settings, &mut hrtf_ipl_settings, &mut hrtf.0) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(hrtf),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }
}

impl Drop for HRTF {
    fn drop(&mut self) {
        unsafe {
            ffi::iplHRTFRelease(&mut self.0);
        }
    }
}