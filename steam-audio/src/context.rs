use std::ffi::CStr;

use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ContextSettings {
    version: Option<u32>,
    simd_level: Option<ffi::IPLSIMDLevel>,
}

unsafe extern "C" fn log_callback(level: ffi::IPLLogLevel, message: *const ::std::os::raw::c_char) {
    let c_str: &CStr = CStr::from_ptr(message);
    let str = c_str.to_str().unwrap();
    eprintln!("{:?}: {}", level, str);
}

impl Into<ffi::IPLContextSettings> for ContextSettings {
    fn into(self) -> ffi::IPLContextSettings {
        ffi::IPLContextSettings {
            version: self.version.unwrap_or(ffi::STEAMAUDIO_VERSION),
            logCallback: Some(log_callback),
            allocateCallback: None,
            simdLevel: self
                .simd_level
                .unwrap_or(ffi::IPLSIMDLevel::IPL_SIMDLEVEL_AVX512),
            freeCallback: None,
        }
    }
}
pub struct Context(ffi::IPLContext);

impl Context {
    pub fn new(settings: ContextSettings) -> Result<Self, SteamAudioError> {
        let mut context = Self(unsafe { std::mem::zeroed() });
        let mut ipl_settings: ffi::IPLContextSettings = settings.into();

        unsafe {
            match ffi::iplContextCreate(&mut ipl_settings, &mut context.0) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(context),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLContext {
        self.0
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::iplContextRelease(&mut self.0);
        }
    }
}
