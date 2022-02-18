use glam::Vec3;
use steam_audio_sys::ffi::{self, IPLAmbisonicsEncodeEffectParams};

use crate::audio_buffer::{AudioBuffer, AudioBufferFrame};
use crate::context::Context;
use crate::error::SteamAudioError;
use crate::hrtf::AudioSettings;

// How many channels the higher order ambisonic has.
//
// 0 -> 1
// 1 -> 4
// 2 -> 9
// ...
pub fn ambisonic_order_channels(order: u8) -> usize {
    (order as usize + 1) * (order as usize + 1)
}

pub struct AmbisonicsDecode(ffi::IPLAmbisonicsDecodeEffect);

pub struct AmbisonicsBinaural(ffi::IPLAmbisonicsBinauralEffect);

pub struct AmbisonicsPanning(ffi::IPLAmbisonicsPanningEffect);

pub struct AmbisonicsRotation(ffi::IPLAmbisonicsRotationEffect);
