extern crate lewton;
extern crate steam_audio_sys;

use glam::Vec3;
use lewton::inside_ogg::{read_headers, OggStreamReader};
use steam_audio::effect::binaural::{BinauralEffect, HRTFInterpolation, BinauralParams};
use steam_audio_sys::ffi::*;

use std::error::Error;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::os::raw::c_char;
use std::ptr::addr_of_mut;
use std::{ptr, slice};

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

use steam_audio::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new(ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = get_audio()?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);
    let frame_length = audio_buffer.frames();
    let channels = audio_buffer.channels();

    let mut output: Vec<Vec<f32>> = vec![vec![]; 2];

    let binaural_effect = BinauralEffect::new(&context, &audio_settings, &hrtf)?;
    for (frame_index, frame) in audio_buffer.into_iter().enumerate(){
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU;

        let mut params = BinauralParams::default();
        params.interpolation = HRTFInterpolation::Bilinear;
        params.direction = Vec3::new(time.cos(), time.sin(), 1.0 - time.cos());

        dbg!();
        let output_buffer = binaural_effect.apply_step(&audio_settings, &params, frame)?;
        dbg!();

        for (channel, output) in output_buffer.data.iter().zip(output.iter_mut()) {
            output.extend(channel);
        }
    }


    // Interleave
    let mut output_interleaved = Vec::new();
    for index in 0..output[0].len() {
        for channel in output.iter() {
            output_interleaved.push(channel[index]);
        }
    }

    use std::io::Write;
    let mut file = File::create("eduardo_wrapper_binaural.raw").unwrap();
    file.write(vf_to_u8(&output_interleaved)).unwrap();

    Ok(())
}
