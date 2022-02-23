extern crate lewton;

use glam::Vec3;
//use steam_audio::effect::ambisonics::decode::AmbisonicsDecodeSettings;
use steam_audio::prelude::*;

use std::error::Error;
use std::path::Path;

const FILENAME: &'static str = "assets/eduardo.ogg";

fn binaural_effect(
    context: &Context,
    audio_settings: &AudioSettings,
    hrtf: &HRTF,
    audio_buffer: AudioBuffer,
) -> Result<(), Box<dyn Error>> {
    let mut output: Vec<Vec<f32>> = vec![vec![]; 2];
    let mut output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 2);
    let frame_length = audio_buffer.frames();

    let binaural_effect = BinauralEffect::new(&context, &audio_settings, &hrtf)?;
    for (frame_index, frame) in audio_buffer.into_iter().enumerate() {
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU * 5.0;

        let mut params = BinauralParams::default();
        params.interpolation = HRTFInterpolation::Bilinear;
        params.direction = Vec3::new(time.cos(), 0.0, time.sin());

        binaural_effect.apply_to_buffer(&params, frame, &mut output_buffer)?;

        steam_audio::extend_deinterleaved(&mut output, &output_buffer.data);
    }

    let filestem = file_stem(FILENAME);
    raw_to_file("binaural", filestem, output)?;

    Ok(())
}

/*
fn ambisonics_effect(
    context: &Context,
    audio_settings: &AudioSettings,
    hrtf: &HRTF,
    input_buffer: AudioBuffer,
) -> Result<(), Box<dyn Error>> {
    let encoded = ambisonics_encode_effect(context, audio_settings, hrtf, input_buffer)?;
    let output = ambisonics_decode_effect(context, audio_settings, hrtf, encoded)?;
    raw_to_file("ambisonics", file_stem(FILENAME), output.data)
}

fn ambisonics_encode_effect(
    context: &Context,
    audio_settings: &AudioSettings,
    hrtf: &HRTF,
    input_buffer: AudioBuffer,
) -> Result<AudioBuffer, Box<dyn Error>> {
    let mut output: Vec<Vec<f32>> = vec![vec![]; 9];
    let mut output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 9);
    let frame_length = input_buffer.frames();

    let encode_effect = AmbisonicsEncode::new(&context, &audio_settings, 2)?;
    for (frame_index, frame) in input_buffer.into_iter().enumerate() {
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU * 5.0;

        let mut encode_params = AmbisonicsEncodeParams::default();
        encode_params.order = 2;
        //encode_params.direction = Vec3::new(time.cos(), 0.0, time.sin());

        encode_effect.apply_to_buffer(&encode_params, frame, &mut output_buffer)?;

        steam_audio::extend_deinterleaved(&mut output, &output_buffer.data);
    }

    Ok(AudioBuffer::from_raw_pcm(audio_settings, output))
}

fn ambisonics_decode_effect(
    context: &Context,
    audio_settings: &AudioSettings,
    hrtf: &HRTF,
    input_buffer: AudioBuffer,
) -> Result<AudioBuffer, Box<dyn Error>> {
    let mut output: Vec<Vec<f32>> = vec![vec![]; 2];
    let mut output_buffer = AudioBuffer::frame_buffer_with_channels(&audio_settings, 2);
    let frame_length = input_buffer.frames();

    dbg!();
    let decode_settings = AmbisonicsDecodeSettings::default();
    let decode_effect = AmbisonicsDecode::new(&context, &audio_settings, &hrtf, &decode_settings)?;
    for (frame_index, frame) in input_buffer.into_iter().enumerate() {
        let time = (frame_index as f32 / frame_length as f32) * std::f32::consts::TAU * 5.0;

        let mut decode_params = AmbisonicsDecodeParams::default();
        decode_effect.apply_to_buffer(&decode_params, frame, &mut output_buffer)?;

        steam_audio::extend_deinterleaved(&mut output, &output_buffer.data);
    }

    Ok(AudioBuffer::from_raw_pcm(audio_settings, output))
}
*/

fn file_stem<P: AsRef<Path>>(p: P) -> String {
    p.as_ref()
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn raw_to_file(
    kind: &'static str,
    name: String,
    data: Vec<Vec<f32>>,
) -> Result<(), Box<dyn Error>> {
    let out_name = format!("assets/out/{}/{}.raw", kind, name);
    println!("outputting to `{}`", out_name);
    let interleaved = steam_audio::interleave(data);
    steam_audio::write_file(out_name, interleaved)
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new(&ContextSettings::default())?;
    let audio_settings = AudioSettings::default();
    let hrtf_settings = HRTFSettings::default();
    let hrtf = HRTF::new(&context, &audio_settings, &hrtf_settings)?;

    let audio = steam_audio::read_ogg(FILENAME)?;
    let audio_buffer = AudioBuffer::from_raw_pcm(&audio_settings, vec![audio]);
    binaural_effect(&context, &audio_settings, &hrtf, audio_buffer.clone())?;
    //ambisonics_effect(&context, &audio_settings, &hrtf, audio_buffer.clone())?;

    Ok(())
}
