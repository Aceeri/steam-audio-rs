
use steam_audio_sys::ffi;

pub enum SteamAudioError {
    IPLError(ffi::IPLerror),
}