#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
extern crate steam_audio;
extern crate gl;
extern crate glfw;

use steam_audio::ffi;
use steam_audio::ffi::IPLSceneType;


//use std::mem;
use std::ptr;
use std::ffi::CString;

// for reading geometry
use std::slice;
use std::mem;
            
static BIN_DATA: &[u8] = include_bytes!("../assets/scene.bin");


fn main() {
    let mut context: ffi::IPLhandle = ptr::null_mut();

    use ffi::IPLerror::*;

    unsafe {
        assert_eq!(IPL_STATUS_SUCCESS, ffi::iplCreateContext(None, None, None, &mut context));
    }

    eprintln!("context={:?}", context);

    // create compute device
    let mut device: ffi::IPLhandle = ptr::null_mut();
    let filter = ffi::IPLComputeDeviceFilter {
        type_: ffi::IPLComputeDeviceType::IPL_COMPUTEDEVICE_CPU,
        requiresTrueAudioNext: ffi::IPLbool::IPL_FALSE,
        minReservableCUs: 0,
        maxCUsToReserve: 32,
    };

    unsafe {
        assert_eq!(IPL_STATUS_SUCCESS, ffi::iplCreateComputeDevice(context, filter, &mut device));
    }

    eprintln!("device={:?}", device);

    let mut scene: ffi::IPLhandle = ptr::null_mut();
    let sim_settings = ffi::IPLSimulationSettings {
        sceneType: IPLSceneType::IPL_SCENETYPE_PHONON,
        numRays: 2048,
        numDiffuseSamples: 512,
        numBounces: 16,
        irDuration: 1.5,
        ambisonicsOrder: 2,
        maxConvolutionSources: 4,
    };

    unsafe {
        assert_eq!(IPL_STATUS_SUCCESS, ffi::iplCreateScene(context, device, sim_settings, 1, &mut scene));
    }

    let mut mesh: ffi::IPLhandle = ptr::null_mut();
    unsafe {
        const NUM_TRIS: ffi::IPLint32 = 28;
        const NUM_VERTS: ffi::IPLint32 = 48;

        assert_eq!(IPL_STATUS_SUCCESS, ffi::iplCreateStaticMesh(scene, NUM_VERTS, NUM_TRIS, &mut mesh));

        // read serialized triangles
        let tris = {
            let data = &BIN_DATA[..336];
            slice::from_raw_parts_mut(data.as_ptr() as *mut ffi::IPLTriangle, mem::size_of::<ffi::IPLTriangle>() * NUM_TRIS as usize)
        };

        let verts = {
            let data = &BIN_DATA[336..];
            slice::from_raw_parts_mut(data.as_ptr() as *mut ffi::IPLVector3, mem::size_of::<ffi::IPLVector3>() * NUM_VERTS as usize)
        };
        
        ffi::iplSetStaticMeshVertices(scene, mesh, verts.as_mut_ptr());
        ffi::iplSetStaticMeshTriangles(scene, mesh, tris.as_mut_ptr());
        ffi::iplFinalizeScene(scene, None);
    }

    // dump debug scene
    unsafe {
        let file = CString::new("scene/scene.obj").unwrap();
        ffi::iplDumpSceneToObjFile(scene, file.into_raw());
    }

    eprintln!("scene={:?}", scene);

    // environment
    let mut env: ffi::IPLhandle = ptr::null_mut();

    unsafe {
        assert_eq!(
            IPL_STATUS_SUCCESS,
            ffi::iplCreateEnvironment(context, device, sim_settings, scene, ptr::null_mut(), &mut env)
        );
    }

    eprintln!("env={:?}", env);
    
    unsafe {
        ffi::iplDestroyEnvironment(&mut env);
        ffi::iplDestroyScene(&mut scene);
        ffi::iplDestroyComputeDevice(&mut device);
        ffi::iplDestroyContext(&mut context);
        ffi::iplCleanup();
    }
}
