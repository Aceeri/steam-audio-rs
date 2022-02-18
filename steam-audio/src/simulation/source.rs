/*
use steam_audio_sys::ffi;

use crate::error::SteamAudioError;

#[derive(Debug, Default)]
pub struct SourceSettings {
}

impl Into<ffi::IPLSourceSettings> for SourceSettings {
    fn into(self) -> ffi::IPLSourceSettings {
        ffi::IPLSourceSettings {
        }
    }
}

pub struct Source(ffi::IPLSource);

impl Source {
    pub fn new(settings: SourceSettings) -> Result<Self, SteamAudioError> {
        let mut source = Self(unsafe { std::mem::zeroed() });
        let mut ipl_settings: ffi::IPLSourceSettings = settings.into();

        unsafe {
            match ffi::iplSourceCreate(&simulator, &mut ipl_settings, &mut source.0) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(source),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLSource {
        self.0
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSourceRelease(&mut self.0);
        }
    }
}
*/
