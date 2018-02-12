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

    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
