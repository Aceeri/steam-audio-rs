extern crate lewton;
extern crate steam_audio_sys;

use glam::Vec3;
use lewton::inside_ogg::OggStreamReader;
use steam_audio::prelude::*;

use std::error::Error;
use std::fs::File;

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

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new(ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = get_audio()?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);
    let frame_length = audio_buffer.frames();

    let mut output: Vec<Vec<f32>> = vec![vec![]; 2];
    let mut output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 2);

    let binaural_effect = BinauralEffect::new(&context, &audio_settings, &hrtf)?;
    for (frame_index, frame) in audio_buffer.into_iter().enumerate() {
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU;

        let mut params = BinauralParams::default();
        params.interpolation = HRTFInterpolation::Bilinear;
        params.direction = Vec3::new(time.cos(), time.sin(), 1.0 - time.cos());

        binaural_effect.apply_step_with_buffer(&params, frame, &mut output_buffer)?;

        for (channel, output) in output_buffer.data.iter().zip(output.iter_mut()) {
            output.extend(channel);
        }
    }

    // Interleave channels
    //
    // 111111112222222233333333
    // -->
    // 123123123123123123123123
    let mut output_interleaved = Vec::new();
    for index in 0..output[0].len() {
        for channel in output.iter() {
            output_interleaved.push(channel[index]);
        }
    }

    use std::io::Write;
    let mut file = File::create("eduardo_wrapper_binaural.raw")?;
    file.write(vf_to_u8(&output_interleaved))?;

    Ok(())
}
