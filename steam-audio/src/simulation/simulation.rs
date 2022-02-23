use steam_audio_sys::ffi;

use bitflags::bitflags;

use crate::{prelude::*, Orientation};

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

impl From<ffi::IPLSimulationFlags> for SimulationFlags {
    fn from(flags: ffi::IPLSimulationFlags) -> Self {
        SimulationFlags { bits: flags.0 }
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

#[derive(Debug)]
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

impl SimulationSettings {
    pub fn from_audio_settings(audio_settings: &AudioSettings) -> Self {
        Self {
            flags: SimulationFlags::default(),
            scene_type: SceneType::default(),
            reflection_type: ReflectionEffectType::default(),
            max_num_occlusion_samples: 0,
            max_num_rays: 4096,
            num_diffuse_samples: 32,
            max_duration: 1.0,
            max_order: 1,
            max_num_sources: 256,
            num_threads: 2,
            ray_batch_size: 0,
            num_vis_samples: 0,
            sampling_rate: audio_settings.sampling_rate(),
            frame_size: audio_settings.frame_size(),
        }
    }
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

pub struct Simulator {
    inner: ffi::IPLSimulator,
    settings: ffi::IPLSimulationSettings,
}

unsafe impl Send for Simulator {}
unsafe impl Sync for Simulator {}

impl crate::SteamAudioObject for Simulator {
    type Object = ffi::IPLSimulator;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> &mut Self::Object {
        &mut self.inner
    }
}


impl Simulator {
    pub fn new(
        context: &Context,
        settings: &SimulationSettings,
    ) -> Result<Self, SteamAudioError> {
        let ipl_settings: ffi::IPLSimulationSettings = settings.into();
        let mut simulator = Self {
            inner: std::ptr::null_mut(),
            settings: ipl_settings,
        };

        unsafe {
            match ffi::iplSimulatorCreate(
                context.inner_raw(),
                &mut simulator.settings,
                simulator.inner_mut(),
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(simulator),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub fn commit(&mut self) {
        unsafe {
            ffi::iplSimulatorCommit(self.inner_raw());
        }
    }

    pub fn add_source(&self, source: &Source) {
        unsafe {
            ffi::iplSourceAdd(source.inner_raw(), self.inner_raw());
        }
    }

    pub fn run_direct(&mut self) {
        unsafe {
            ffi::iplSimulatorRunDirect(self.inner_raw());
        }
    }

    pub fn run_reflections(&mut self) {
        unsafe {
            ffi::iplSimulatorRunReflections(self.inner_raw());
        }
    }

    pub fn run_pathing(&mut self) {
        unsafe {
            ffi::iplSimulatorRunPathing(self.inner_raw());
        }
    }

    pub fn set_shared_inputs(
        &self,
        flags: SimulationFlags,
        shared_inputs: &SimulationSharedInputs,
    ) {
        // Uhhhh might need to store this somewhere to be safe?
        let mut shared_inputs: ffi::IPLSimulationSharedInputs = shared_inputs.into();

        unsafe {
            ffi::iplSimulatorSetSharedInputs(self.inner_raw(), flags.into(), &mut shared_inputs);
        }
    }

    pub fn set_scene(&self, scene: &Scene) {
        unsafe {
            ffi::iplSimulatorSetScene(self.inner_raw(), scene.inner_raw());
        }
    }
}

impl Drop for Simulator {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSimulatorRelease(self.inner_mut());
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationSharedInputs {
    pub listener: Orientation,
    pub num_rays: u32,
    pub num_bounces: u32,
    pub duration: f32,
    pub order: u8,
    pub irradiance_min_distance: f32,
}

impl Default for SimulationSharedInputs {
    fn default() -> Self {
        Self {
            listener: Orientation::default(),
            num_rays: 4096,
            num_bounces: 16,
            duration: 2.0,
            order: 1,
            irradiance_min_distance: 1.0,
        }
    }
}

impl Into<ffi::IPLSimulationSharedInputs> for &SimulationSharedInputs {
    fn into(self) -> ffi::IPLSimulationSharedInputs {
        ffi::IPLSimulationSharedInputs {
            listener: self.listener.clone().into(),
            numRays: self.num_rays as i32,
            numBounces: self.num_bounces as i32,
            duration: self.duration,
            order: self.order as i32,
            irradianceMinDistance: self.irradiance_min_distance,
        }
    }
}
