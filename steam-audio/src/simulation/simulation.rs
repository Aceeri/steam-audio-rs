use steam_audio_sys::ffi;

use bitflags::bitflags;

use crate::context::Context;
use crate::error::SteamAudioError;

bitflags! {
    pub struct SimulationFlags: i32 {
        const DIRECT = ffi::IPLSimulationFlags::IPL_SIMULATIONFLAGS_DIRECT.0;
        const REFLECTIONS = ffi::IPLSimulationFlags::IPL_SIMULATIONFLAGS_REFLECTIONS.0;
        const PATHING = ffi::IPLSimulationFlags::IPL_SIMULATIONFLAGS_PATHING.0;
        const ALL = SimulationFlags::DIRECT.bits() | SimulationFlags::REFLECTIONS.bits() | SimulationFlags::PATHING.bits();
        const DEFAULT = SimulationFlags::DIRECT.bits();
    }
}

impl Default for SimulationFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Into<ffi::IPLSimulationFlags> for SimulationFlags {
    fn into(self) -> ffi::IPLSimulationFlags {
        ffi::IPLSimulationFlags(self.bits())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SceneType {
    Default,
    Embree,
    RadeonRays,
    Custom,
}

impl Default for SceneType {
    fn default() -> Self {
        Self::Default
    }
}

impl Into<ffi::IPLSceneType> for SceneType {
    fn into(self) -> ffi::IPLSceneType {
        match self {
            SceneType::Default => ffi::IPLSceneType::IPL_SCENETYPE_DEFAULT,
            SceneType::Embree => ffi::IPLSceneType::IPL_SCENETYPE_EMBREE,
            SceneType::RadeonRays => ffi::IPLSceneType::IPL_SCENETYPE_RADEONRAYS,
            SceneType::Custom => ffi::IPLSceneType::IPL_SCENETYPE_CUSTOM,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ReflectionEffectType {
    Convolution,
    Parametric,
    Hybrid,
    Tan,
}

impl Default for ReflectionEffectType {
    fn default() -> Self {
        Self::Convolution
    }
}

impl Into<ffi::IPLReflectionEffectType> for ReflectionEffectType {
    fn into(self) -> ffi::IPLReflectionEffectType {
        match self {
            ReflectionEffectType::Convolution => {
                ffi::IPLReflectionEffectType::IPL_REFLECTIONEFFECTTYPE_CONVOLUTION
            }
            ReflectionEffectType::Parametric => {
                ffi::IPLReflectionEffectType::IPL_REFLECTIONEFFECTTYPE_PARAMETRIC
            }
            ReflectionEffectType::Hybrid => {
                ffi::IPLReflectionEffectType::IPL_REFLECTIONEFFECTTYPE_HYBRID
            }
            ReflectionEffectType::Tan => ffi::IPLReflectionEffectType::IPL_REFLECTIONEFFECTTYPE_TAN,
        }
    }
}

#[derive(Debug, Default)]
pub struct SimulationSettings {
    pub flags: SimulationFlags,
    pub scene_type: SceneType,
    pub reflection_type: ReflectionEffectType,
    pub max_num_occlusion_samples: u32,
    pub max_num_rays: u32,
    pub num_diffuse_samples: u32,
    pub max_duration: f32,
    pub max_order: u8,
    pub max_num_sources: u32,
    pub num_threads: u32,
    pub ray_batch_size: u32,
    pub num_vis_samples: u32,
    pub sampling_rate: u32,
    pub frame_size: u32,
    // TODO:
    //pub opencl_device: IPLOpenCLDevice,
    //pub radeon_rays_device: IPLRadeonRaysDevice,
    //pub tan_device: IPLTrueAudioNextDevice,
}

impl Into<ffi::IPLSimulationSettings> for &SimulationSettings {
    fn into(self) -> ffi::IPLSimulationSettings {
        ffi::IPLSimulationSettings {
            flags: self.flags.into(),
            sceneType: self.scene_type.into(),
            reflectionType: self.reflection_type.into(),
            maxNumOcclusionSamples: self.max_num_occlusion_samples as i32,
            maxNumRays: self.max_num_rays as i32,
            numDiffuseSamples: self.num_diffuse_samples as i32,
            maxDuration: self.max_duration,
            maxOrder: self.max_order as i32,
            maxNumSources: self.max_num_sources as i32,
            numThreads: self.num_threads as i32,
            rayBatchSize: self.ray_batch_size as i32,
            numVisSamples: self.num_vis_samples as i32,
            samplingRate: self.sampling_rate as i32,
            frameSize: self.frame_size as i32,
            openCLDevice: std::ptr::null_mut(),
            radeonRaysDevice: std::ptr::null_mut(),
            tanDevice: std::ptr::null_mut(),
        }
    }
}

pub struct Simulator(ffi::IPLSimulator);

impl Simulator {
    pub fn new(context: &Context, settings: &SimulationSettings) -> Result<Self, SteamAudioError> {
        let mut simulator = Self(unsafe { std::mem::zeroed() });
        let mut ipl_settings: ffi::IPLSimulationSettings = settings.into();

        unsafe {
            match ffi::iplSimulatorCreate(context.inner(), &mut ipl_settings, &mut simulator.0) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(simulator),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLSimulator {
        self.0
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSimulatorRelease(&mut self.0);
        }
    }
}