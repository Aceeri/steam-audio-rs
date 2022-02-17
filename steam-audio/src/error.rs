use steam_audio_sys::ffi;

#[derive(Debug)]
pub enum SteamAudioError {
    IPLError(ffi::IPLerror),
}

impl std::fmt::Display for SteamAudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::IPLError(error) => match error {
                ffi::IPLerror::IPL_STATUS_SUCCESS => "ipl status success",
                ffi::IPLerror::IPL_STATUS_FAILURE => "ipl status failure",
                ffi::IPLerror::IPL_STATUS_OUTOFMEMORY => "ipl status out of memory",
                ffi::IPLerror::IPL_STATUS_INITIALIZATION => "ipl status initialization: An error occurred while initializing an external dependency.",
            }
        };

        write!(f, "{}", description)
    }
}

impl std::error::Error for SteamAudioError {}
