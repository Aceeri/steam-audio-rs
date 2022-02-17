use crate::error::SteamAudioError;


pub trait RefCounted: Sized {
    type Settings;
    fn create(settings: Self::Settings) -> Result<Self, SteamAudioError>;
    fn release(self);
}
