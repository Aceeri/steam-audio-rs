#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
extern crate steam_audio;
extern crate gl;
extern crate glfw;

use steam_audio::ffi::*;

use std::{ptr, slice};
use std::ffi::CString;
            
static BIN_DATA: &[u8] = include_bytes!("../assets/scene.bin");

static SIM_SETTINGS: IPLSimulationSettings = IPLSimulationSettings {
    sceneType: IPLSceneType::IPL_SCENETYPE_PHONON,
    numRays: 2048,
    numDiffuseSamples: 512,
    numBounces: 16,
    irDuration: 1.5,
    ambisonicsOrder: 2,
    maxConvolutionSources: 4,
};

static DEVICE_FILTER: IPLComputeDeviceFilter = IPLComputeDeviceFilter {
    type_: IPLComputeDeviceType::IPL_COMPUTEDEVICE_CPU,
    requiresTrueAudioNext: IPLbool::IPL_FALSE,
    minReservableCUs: 0,
    maxCUsToReserve: 32,
};

fn main() {
    let mut context = unsafe {
        let mut context = ptr::null_mut();
        assert_eq!(IPLerror::IPL_STATUS_SUCCESS, iplCreateContext(None, None, None, &mut context));
        context
    };

    let mut device: IPLhandle = unsafe {
        let mut device = ptr::null_mut();
        assert_eq!(IPLerror::IPL_STATUS_SUCCESS, iplCreateComputeDevice(context, DEVICE_FILTER, &mut device));
        device
    };

    let mut scene = unsafe {
        let mut scene: IPLhandle = ptr::null_mut();
        let mut mesh: IPLhandle = ptr::null_mut();

        const NUM_TRIS: IPLint32 = 28;
        const NUM_VERTS: IPLint32 = 48;

        assert_eq!(IPLerror::IPL_STATUS_SUCCESS, iplCreateScene(context, device, SIM_SETTINGS, 1, &mut scene));
        assert_eq!(IPLerror::IPL_STATUS_SUCCESS, iplCreateStaticMesh(scene, NUM_VERTS, NUM_TRIS, &mut mesh));

        let tris: &mut [IPLTriangle] = slice::from_raw_parts_mut(BIN_DATA.as_ptr() as _, 336);
        let vert: &mut [IPLVector3] = slice::from_raw_parts_mut(BIN_DATA.as_ptr().offset(336) as _, 576);

        iplSetStaticMeshVertices(scene, mesh, vert.as_mut_ptr());
        iplSetStaticMeshTriangles(scene, mesh, tris.as_mut_ptr());

        iplFinalizeScene(scene, None);
        scene
    };

    unsafe {
        let file = CString::new("scene/scene.obj").unwrap();
        iplDumpSceneToObjFile(scene, file.into_raw());
    }

    let mut env = unsafe {
        let mut env = ptr::null_mut();
        assert_eq!(IPLerror::IPL_STATUS_SUCCESS, iplCreateEnvironment(context, device, SIM_SETTINGS, scene, ptr::null_mut(), &mut env));
        env
    };

    eprintln!("context={:?}", context);
    eprintln!("device={:?}", device);
    eprintln!("scene={:?}", scene);
    eprintln!("env={:?}", env);
    
    unsafe {
        iplDestroyEnvironment(&mut env);
        iplDestroyScene(&mut scene);
        iplDestroyComputeDevice(&mut device);
        iplDestroyContext(&mut context);
        iplCleanup();
    }
}
