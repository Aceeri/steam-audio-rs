use std::ffi::{CStr, c_void};

use steam_audio_sys::ffi::{self, iplContextRetain};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct ContextSettings {
    version: Option<u32>,
    simd_level: Option<ffi::IPLSIMDLevel>,
}

unsafe extern "C" fn log_callback(level: ffi::IPLLogLevel, message: *const ::std::os::raw::c_char) {
    dbg!();
    let c_str: &CStr = CStr::from_ptr(message);
    let str = c_str.to_str().unwrap();
    eprintln!("{:?}: {}", level, str);
}

unsafe extern "C" fn alloc_callback(size: ffi::IPLsize, alignment: ffi::IPLsize) -> *mut c_void {
    let layout = match std::alloc::Layout::from_size_align(size as usize, alignment as usize) {
        Ok(layout) => layout,
        _ => return std::ptr::null_mut(),
    };

    std::alloc::alloc(layout) as *mut c_void
}

unsafe extern "C" fn free_callback(size: ffi::IPLsize, alignment: ffi::IPLsize) -> *mut c_void {
    let layout = match std::alloc::Layout::from_size_align(size as usize, alignment as usize) {
        Ok(layout) => layout,
        _ => return std::ptr::null_mut(),
    };

    std::alloc::alloc(layout) as *mut c_void
}

impl Into<ffi::IPLContextSettings> for ContextSettings {
    fn into(self) -> ffi::IPLContextSettings {
        ffi::IPLContextSettings {
            version: self.version.unwrap_or(ffi::STEAMAUDIO_VERSION),
            logCallback: Some(log_callback),
            //logCallback: None,
            allocateCallback: Some(alloc_callback),
            simdLevel: self
                .simd_level
                .unwrap_or(ffi::IPLSIMDLevel::IPL_SIMDLEVEL_SSE2),
            freeCallback: None,
        }
    }
}
pub struct Context {
    pub(crate) inner: ffi::IPLContext,
    settings: ffi::IPLContextSettings,
}

impl Context {
    pub fn new(settings: ContextSettings) -> Result<Self, SteamAudioError> {
        let mut ipl_settings: ffi::IPLContextSettings = settings.into();
        let mut context = Self {
            inner: unsafe { std::mem::zeroed() },
            settings: ipl_settings,
        };

        unsafe {
            match ffi::iplContextCreate(&mut context.settings, &mut context.inner) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(context),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLContext {
        self.inner
    }

    pub fn retain(&self) -> Context {
        unsafe {
            let new_context = iplContextRetain(self.inner);
            Context {
                inner: new_context,
                settings: self.settings,
            }
        }
    }

    pub fn debug(&mut self) {
        dbg!(self.inner);
        dbg!(self.settings);
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::iplContextRelease(&mut self.inner);
        }
    }
}
