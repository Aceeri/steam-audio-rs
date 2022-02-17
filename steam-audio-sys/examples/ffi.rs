extern crate lewton;
extern crate steam_audio_sys;

use lewton::inside_ogg::{read_headers, OggStreamReader};
use steam_audio_sys::ffi::*;

use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::os::raw::c_char;
use std::ptr::addr_of_mut;
use std::{ptr, slice};

unsafe extern "C" fn log_callback(level: IPLLogLevel, message: *const ::std::os::raw::c_char) {
    let c_str: &CStr = CStr::from_ptr(message);
    let str = c_str.to_str().unwrap();
    eprintln!("{:?}: {}", level, str);
}

struct IPLAudioBufferIterator {
    buffer: IPLAudioBuffer,
    frames: usize,
}

impl Iterator for IPLAudioBufferIterator {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        if self.frames > 0 {
            unsafe {
                self.buffer.data = self.buffer.data.offset(self.buffer.numSamples as isize);
            }

            self.frames -= 1;
            Some(())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct InputAudioInformation {
    data: Vec<f32>,
    outer: Vec<*mut f32>,
    frames: usize,
    frame_size: usize,
    buffer: IPLAudioBuffer,
}

impl InputAudioInformation {
    fn from_pcm_data(
        settings: IPLAudioSettings,
        mut data: Vec<f32>,
    ) -> Result<Self, Box<dyn Error>> {
        let frames = data.len() / settings.frameSize as usize;

        let buffer = IPLAudioBuffer {
            numChannels: 1,
            numSamples: settings.frameSize,
            data: ptr::null_mut(),
        };

        let mut input = Self {
            data,
            frames,
            frame_size: settings.frameSize as usize,
            buffer,
            outer: Vec::new(),
        };
        input.outer.push(input.data.as_mut_ptr());
        input.buffer.data = input.outer.as_mut_ptr();

        Ok(input)
    }
}

fn get_audio() -> Result<Vec<f32>, Box<dyn Error>> {
    let file = File::open("assets/eduardo.ogg")?;
    let mut stream_reader = OggStreamReader::new(file)?;
    assert_eq!(stream_reader.ident_hdr.audio_channels, 1);

    let mut concatted = Vec::new();
    while let Some(packet) = stream_reader.read_dec_packet_generic::<Vec<Vec<f32>>>()? {
        concatted.extend(packet[0].clone());
    }

    Ok(concatted)
}

fn vf_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

fn main() {
    let mut context = unsafe {
        let mut context = ptr::null_mut();
        let mut settings = IPLContextSettings {
            version: STEAMAUDIO_VERSION,
            logCallback: Some(log_callback),
            allocateCallback: None,
            simdLevel: IPLSIMDLevel::IPL_SIMDLEVEL_AVX512,
            freeCallback: None,
        };
        assert_eq!(
            IPLerror::IPL_STATUS_SUCCESS,
            iplContextCreate(&mut settings, &mut context)
        );
        context
    };

    println!("{:?}", context);

    let mut audio_settings = IPLAudioSettings {
        samplingRate: 44100,
        frameSize: 1024,
    };

    let file = CString::new("").unwrap();
    let mut hrtf = unsafe {
        let mut hrtf = ptr::null_mut();
        let mut hrtf_settings = IPLHRTFSettings {
            type_: IPLHRTFType::IPL_HRTFTYPE_DEFAULT,
            sofaFileName: file.as_ptr() as *const c_char,
        };

        println!("{:?}", hrtf);
        println!("{:?}", hrtf_settings);
        println!("{:?}", audio_settings);
        dbg!(context);
        dbg!(*context);
        assert_eq!(
            IPLerror::IPL_STATUS_SUCCESS,
            iplHRTFCreate(context, &mut audio_settings, &mut hrtf_settings, &mut hrtf,)
        );
        hrtf
    };

    dbg!(hrtf);

    let mut audio_buffer = get_audio().unwrap();
    let mut input = InputAudioInformation::from_pcm_data(audio_settings, audio_buffer).unwrap();

    {
        let mut effect_settings = IPLBinauralEffectSettings { hrtf: hrtf };

        let mut binaural = unsafe {
            let mut effect = ptr::null_mut();
            assert_eq!(
                IPLerror::IPL_STATUS_SUCCESS,
                iplBinauralEffectCreate(
                    context,
                    &mut audio_settings,
                    &mut effect_settings,
                    &mut effect
                )
            );
            effect
        };

        let mut output_buffer = IPLAudioBuffer {
            numChannels: 2,
            numSamples: audio_settings.frameSize,
            data: ptr::null_mut(),
        };

        unsafe {
            assert_eq!(
                IPLerror::IPL_STATUS_SUCCESS,
                iplAudioBufferAllocate(context, 2, audio_settings.frameSize, &mut output_buffer)
            );
        }

        let mut output_audio = Vec::new();

        dbg!(binaural);
        dbg!(input.frames);
        dbg!(input.data.len());

        for frame in 0..input.frames - 1 {
            let time = (frame as f32 / input.frames as f32) * std::f32::consts::TAU;

            let direction = IPLVector3 {
                x: time.cos() / 2.0,
                y: time.sin() * 4.0,
                z: (1.0 - time.cos()) / 2.0,
            };

            let mut effect_params = IPLBinauralEffectParams {
                direction: direction,
                hrtf: hrtf,
                interpolation: IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
                spatialBlend: 1.0,
            };

            println!("{:?}", frame);
            dbg!(&input.buffer);
            dbg!(&output_buffer);

            let mut output_audio_frame: Vec<f32> =
                vec![0.0; (2 * audio_settings.frameSize) as usize];

            unsafe {
                dbg!();
                let effect = iplBinauralEffectApply(
                    binaural,
                    &mut effect_params,
                    &mut input.buffer,
                    &mut output_buffer,
                );
                dbg!(effect);
                dbg!();
                iplAudioBufferInterleave(
                    context,
                    &mut output_buffer,
                    output_audio_frame.as_mut_ptr(),
                );
                dbg!();
            }

            //println!("{:?}", output_audio);

            output_audio.extend(output_audio_frame.clone());

            unsafe {
                (*input.buffer.data) =
                    (*input.buffer.data).offset(audio_settings.frameSize as isize);
            }
        }

        use std::io::Write;
        let mut file = File::create("eduardo_binaural.raw").unwrap();
        file.write(vf_to_u8(&output_audio)).unwrap();

        unsafe {
            iplAudioBufferFree(context, &mut output_buffer);
            iplBinauralEffectRelease(&mut binaural);
        }
    }

    //let effect_settings = IPLAmbisonicsEncodeEffectSettings { maxOrder: 2 };

    /*
    let mut device = unsafe {
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
    */

    unsafe {
        /*
        iplDestroyEnvironment(&mut env);
        iplDestroyScene(&mut scene);
        iplDestroyComputeDevice(&mut device);
        iplDestroyContext(&mut context);
        iplCleanup();
        */

        iplHRTFRelease(&mut hrtf);
        iplContextRelease(&mut context);
    }
}
