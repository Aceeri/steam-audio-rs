pub mod audio_buffer;
pub mod context;
pub mod effect;
pub mod error;
pub mod hrtf;
pub mod interleave;
pub mod orientation;
pub mod raw;
pub mod simulation;

pub use effect::ambisonics::ambisonic_order_channels;
pub use interleave::{extend_deinterleaved, interleave};
pub use orientation::Orientation;
pub use raw::{read_ogg, write_file};

pub mod prelude {
    pub use crate::audio_buffer::{AudioBuffer, AudioBufferFrame};
    pub use crate::context::{Context, ContextSettings};
    pub use crate::effect::{
        ambisonics::{
            decode::{AmbisonicsDecode, AmbisonicsDecodeParams},
            encode::{AmbisonicsEncode, AmbisonicsEncodeParams},
        },
        binaural::{BinauralEffect, BinauralParams},
        direct::{DirectEffect, DirectEffectParams, DirectEffectFlags, DirectSimulationFlags},
    };
    pub use crate::error::SteamAudioError;
    pub use crate::hrtf::{AudioSettings, HRTFInterpolation, HRTFSettings, HRTF};
    pub use crate::simulation::{
        simulation::{SimulationFlags, SimulationSettings, Simulator},
        source::{Source, SourceSettings, DistanceAttenuationCallback, DistanceAttenuationModel, SimulationInputs},
    };
}
