use std::ffi::{c_void, CStr};

use steam_audio_sys::ffi;

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

/*
unsafe extern "C" fn free_callback(size: ffi::IPLsize, alignment: ffi::IPLsize) -> *mut c_void {
    let layout = match std::alloc::Layout::from_size_align(size as usize, alignment as usize) {
        Ok(layout) => layout,
        _ => return std::ptr::null_mut(),
    };

    std::alloc::dealloc(layout) as *mut c_void
}
*/

impl Into<ffi::IPLContextSettings> for &ContextSettings {
    fn into(self) -> ffi::IPLContextSettings {
        ffi::IPLContextSettings {
            version: self.version.unwrap_or(ffi::STEAMAUDIO_VERSION),
            logCallback: Some(log_callback),
            allocateCallback: Some(alloc_callback),
            simdLevel: self
                .simd_level
                .unwrap_or(ffi::IPLSIMDLevel::IPL_SIMDLEVEL_SSE2),
            freeCallback: None,
        }
    }
}
pub struct Context {
    inner: ffi::IPLContext,

    // I'm not sure if this is necessary as I don't think the settings
    // are used after creation, but better to be safe here and to ensure
    // it is still alive while the context is alive.
    settings: ffi::IPLContextSettings,
}

// This is supposedly safe as IPLContext is allowed to be used from multiple threads
// according to the documentation of steam audio.
unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl crate::SteamAudioObject for Context {
    type Object = ffi::IPLContext;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> &mut Self::Object {
        &mut self.inner
    }
}

impl Context {
    pub fn new(settings: &ContextSettings) -> Result<Self, SteamAudioError> {
        let ipl_settings: ffi::IPLContextSettings = settings.into();
        let mut context = Self {
            inner: std::ptr::null_mut(),
            settings: ipl_settings,
        };

        unsafe {
            match ffi::iplContextCreate(&mut context.settings, context.inner_mut()) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => {},
                err => return Err(SteamAudioError::IPLError(err)),
            };
            
            Ok(context)
        }

    }

    pub fn retain(&self) -> Context {
        unsafe {
            let new_context = ffi::iplContextRetain(self.inner_raw());
            Context {
                inner: new_context,
                settings: self.settings,
            }
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            ffi::iplContextRelease(self.inner_mut());
        }
    }
}
