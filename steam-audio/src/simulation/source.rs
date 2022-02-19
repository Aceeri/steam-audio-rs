use crate::{prelude::*, Orientation};
use steam_audio_sys::ffi;

#[derive(Debug, Default)]
pub struct SourceSettings {
    flags: SimulationFlags,
}

impl Into<ffi::IPLSourceSettings> for &SourceSettings {
    fn into(self) -> ffi::IPLSourceSettings {
        ffi::IPLSourceSettings {
            flags: self.flags.into(),
        }
    }
}

pub struct SimulationOutputs {
    pub direct: DirectEffectParams,
    /*
    pub reflections: ReflectionEffectParams,
    pub pathing: PathEffectParams,
    */
}

impl From<ffi::IPLSimulationOutputs> for SimulationOutputs {
    fn from(other: ffi::IPLSimulationOutputs) -> Self {
        Self {
            direct: other.direct.into(),
        }
    }
}

pub struct Source(ffi::IPLSource);

impl Source {
    pub fn new(simulator: &Simulator, settings: &SourceSettings) -> Result<Self, SteamAudioError> {
        let mut source = Self(unsafe { std::mem::zeroed() });
        let mut ipl_settings: ffi::IPLSourceSettings = settings.into();

        unsafe {
            match ffi::iplSourceCreate(simulator.inner(), &mut ipl_settings, &mut source.0) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(source),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLSource {
        self.0
    }

    pub fn get_outputs(&self, flags: SimulationFlags) -> SimulationOutputs {
        unsafe {
            let mut outputs: ffi::IPLSimulationOutputs = std::mem::zeroed();
            ffi::iplSourceGetOutputs(self.inner(), flags.into(), &mut outputs);
            outputs.into()
        }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSourceRelease(&mut self.0);
        }
    }
}

pub trait DistanceAttenuationCallback {
    fn attenuation(&self, distance: f32) -> f32;
}

pub enum DistanceAttenuationModel {
    Default,
    InverseDistance {
        min_distance: f32,
    },
    Callback {
        callback: Box<dyn DistanceAttenuationCallback>,
        dirty: bool,
    },
}

impl Default for DistanceAttenuationModel {
    fn default() -> Self {
        Self::Default
    }
}

pub struct SimulationInputs {
    pub flags: SimulationFlags,
    pub direct_flags: DirectSimulationFlags,
    pub source: Orientation,
    pub distance_attenuation_model: DistanceAttenuationModel,
}
