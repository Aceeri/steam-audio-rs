///```
///#![allow(non_snake_case)]
///#![allow(non_camel_case_types)]
///#![allow(non_upper_case_globals)]
///extern crate steam_audio;
///
///use steam_audio::ffi;
///
///use std::mem;
///
///fn main() {
///    let mut context: ffi::IPLhandle = unsafe { mem::zeroed() };
///
///    use ffi::IPLerror::*;
///
///    match unsafe { ffi::iplCreateContext(None, None, None, &mut context) } {
///        IPL_STATUS_SUCCESS => eprintln!("Successfully created context"),
///        err @ _ => panic!("Error creating context ({:?})", err),
///    }
///
///    unsafe {
///        ffi::iplDestroyContext(&mut context);
///    }
///}
///```
pub mod ffi {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]

    pub const STEAMAUDIO_VERSION: u32 =
        STEAMAUDIO_VERSION_MAJOR << 16 | STEAMAUDIO_VERSION_MINOR << 8 | STEAMAUDIO_VERSION_PATCH;

    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}

impl From<glam::Vec3> for ffi::IPLVector3 {
    fn from(vec: glam::Vec3) -> Self {
        ffi::IPLVector3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<&glam::Vec3> for ffi::IPLVector3 {
    fn from(vec: &glam::Vec3) -> Self {
        ffi::IPLVector3 {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl From<bool> for ffi::IPLbool {
    fn from(b: bool) -> Self {
        match b {
            true => ffi::IPLbool::IPL_TRUE,
            false => ffi::IPLbool::IPL_FALSE,
        }
    }
}

impl From<&bool> for ffi::IPLbool {
    fn from(b: &bool) -> Self {
        match b {
            true => ffi::IPLbool::IPL_TRUE,
            false => ffi::IPLbool::IPL_FALSE,
        }
    }
}
