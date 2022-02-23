pub mod audio_buffer;
pub mod context;
pub mod device;
pub mod effect;
pub mod error;
pub mod hrtf;
pub mod interleave;
pub mod orientation;
pub mod raw;
pub mod simulation;


pub trait SteamAudioObject: Send + Sync {
    type Object: Clone;
    // Raw inner object.
    fn inner_raw(&self) -> Self::Object;
    // Pointer to internal object.
    // 
    // For example when we create a context we want the address
    // of where we want the context to go.
    fn inner_mut(&mut self) -> &mut Self::Object;
}


//pub use effect::ambisonics::ambisonic_order_channels;
pub use interleave::{extend_deinterleaved, interleave};
pub use orientation::Orientation;
pub use raw::{read_ogg, write_file};
pub use simulation::material::materials;

pub mod prelude {
    pub use crate::SteamAudioObject;
    pub use crate::audio_buffer::{AudioBuffer, FFIAudioBufferFrame};
    pub use crate::context::{Context, ContextSettings};
    pub use crate::effect::{
        /*
        ambisonics::{
            decode::{AmbisonicsDecode, AmbisonicsDecodeParams},
            encode::{AmbisonicsEncode, AmbisonicsEncodeParams},
        },
        */
        binaural::{BinauralEffect, BinauralParams},
        direct::{DirectEffect, DirectEffectFlags, DirectEffectParams, DirectSimulationFlags},
    };
    pub use crate::error::SteamAudioError;
    pub use crate::hrtf::{AudioSettings, HRTFInterpolation, HRTFSettings, HRTF};
    pub use crate::simulation::{
        material::Material,
        scene::{Scene, SceneSettings},
        simulation::{SimulationFlags, SimulationSettings, Simulator, SimulationSharedInputs},
        source::{
            DistanceAttenuationCallback, DistanceAttenuationModel, SimulationInputs, Source,
            SourceSettings,
        },
        static_mesh::{StaticMesh, StaticMeshSettings},
    };
}
