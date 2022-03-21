use crate::{prelude::*, Orientation};
use steam_audio_sys::ffi;

#[derive(Debug, Default)]
pub struct SourceSettings {
    pub flags: SimulationFlags,
}

impl Into<ffi::IPLSourceSettings> for &SourceSettings {
    fn into(self) -> ffi::IPLSourceSettings {
        ffi::IPLSourceSettings {
            flags: self.flags.into(),
        }
    }
}

#[derive(Debug, Clone)]
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

pub struct Source {
    inner: ffi::IPLSource,
}

unsafe impl Send for Source {}
unsafe impl Sync for Source {}

impl crate::SteamAudioObject for Source {
    type Object = ffi::IPLSource;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> *mut Self::Object {
        std::ptr::addr_of_mut!(self.inner)
    }
}

impl Source {
    pub fn new(simulator: &Simulator, settings: &SourceSettings) -> Result<Self, SteamAudioError> {
        let mut source = Self {
            inner: std::ptr::null_mut(),
        };

        let mut ipl_settings: ffi::IPLSourceSettings = settings.into();

        unsafe {
            match ffi::iplSourceCreate(simulator.inner_raw(), &mut ipl_settings, source.inner_mut())
            {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(source),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub fn get_outputs(&self, flags: SimulationFlags) -> SimulationOutputs {
        unsafe {
            let mut outputs: ffi::IPLSimulationOutputs = std::mem::zeroed();
            ffi::iplSourceGetOutputs(self.inner_raw(), flags.into(), &mut outputs);
            outputs.into()
        }
    }

    pub fn set_inputs(&self, flags: SimulationFlags, inputs: &SimulationInputs) {
        unsafe {
            let mut inputs: ffi::IPLSimulationInputs = inputs.into();
            ffi::iplSourceSetInputs(self.inner_raw(), flags.into(), &mut inputs);
        }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSourceRelease(self.inner_mut());
        }
    }
}

pub trait DistanceAttenuationCallback {
    fn attenuation(&self, distance: f32) -> f32;
}

#[derive(Debug, Clone)]
pub enum DistanceAttenuationModel {
    Default,
    InverseDistance { min_distance: f32 },
}

impl Default for DistanceAttenuationModel {
    fn default() -> Self {
        Self::Default
    }
}

impl Into<ffi::IPLDistanceAttenuationModel> for DistanceAttenuationModel {
    fn into(self) -> ffi::IPLDistanceAttenuationModel {
        let mut model = ffi::IPLDistanceAttenuationModel {
            type_: ffi::IPLDistanceAttenuationModelType::IPL_DISTANCEATTENUATIONTYPE_DEFAULT,
            minDistance: 1.0,
            callback: None,
            userData: std::ptr::null_mut(),
            dirty: false.into(),
        };

        match self {
            Self::Default => {
                model.type_ =
                    ffi::IPLDistanceAttenuationModelType::IPL_DISTANCEATTENUATIONTYPE_DEFAULT;
            }
            Self::InverseDistance { min_distance } => {
                model.type_ = ffi::IPLDistanceAttenuationModelType::IPL_DISTANCEATTENUATIONTYPE_INVERSEDISTANCE;
                model.minDistance = min_distance;
            }
        }

        model
    }
}

#[derive(Debug, Clone)]
pub enum AirAbsorptionModel {
    Default,
    Exponential { coefficients: [f32; 3] },
}

impl Default for AirAbsorptionModel {
    fn default() -> Self {
        Self::Default
    }
}

impl Into<ffi::IPLAirAbsorptionModel> for AirAbsorptionModel {
    fn into(self) -> ffi::IPLAirAbsorptionModel {
        let mut model = ffi::IPLAirAbsorptionModel {
            type_: ffi::IPLAirAbsorptionModelType::IPL_AIRABSORPTIONTYPE_DEFAULT,
            coefficients: [0.0, 0.0, 0.0],
            callback: None,
            userData: std::ptr::null_mut(),
            dirty: false.into(),
        };

        match self {
            Self::Default => {
                model.type_ = ffi::IPLAirAbsorptionModelType::IPL_AIRABSORPTIONTYPE_DEFAULT;
            }
            Self::Exponential { coefficients } => {
                model.type_ = ffi::IPLAirAbsorptionModelType::IPL_AIRABSORPTIONTYPE_EXPONENTIAL;
                model.coefficients = coefficients;
            }
        }

        model
    }
}

#[derive(Debug, Clone)]
pub struct Directivity {
    pub dipole_weight: f32,
    pub dipole_power: f32,
}

impl Default for Directivity {
    fn default() -> Self {
        Self {
            dipole_weight: 0.0,
            dipole_power: 0.0,
        }
    }
}

impl Into<ffi::IPLDirectivity> for Directivity {
    fn into(self) -> ffi::IPLDirectivity {
        ffi::IPLDirectivity {
            dipolePower: self.dipole_power,
            dipoleWeight: self.dipole_weight,
            callback: None,
            userData: std::ptr::null_mut(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum OcclusionType {
    Raycast,
    Volumetric {
        occlusion_radius: f32,
        num_occlusion_samples: u32,
    },
}

impl Default for OcclusionType {
    fn default() -> Self {
        Self::Raycast
    }
}

impl Into<ffi::IPLOcclusionType> for OcclusionType {
    fn into(self) -> ffi::IPLOcclusionType {
        match self {
            Self::Raycast => ffi::IPLOcclusionType::IPL_OCCLUSIONTYPE_RAYCAST,
            Self::Volumetric { .. } => ffi::IPLOcclusionType::IPL_OCCLUSIONTYPE_VOLUMETRIC,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationInputs {
    pub flags: SimulationFlags,
    pub direct_flags: DirectSimulationFlags,
    pub source: Orientation,
    pub distance_attenuation_model: DistanceAttenuationModel,
    pub air_absorption_model: AirAbsorptionModel,
    pub directivity: Directivity,
    pub occlusion_type: OcclusionType,
    pub reverb_scale: [f32; 3],
    pub hybrid_reverb_transition_time: f32,
    pub hybrid_reverb_overlap_percent: f32,
    pub baked: bool,
    //pub baked_data_identifier: bool,
    //pub pathing_probes: ProbeBatch,
    pub visible_radius: f32,
    pub visible_threshold: f32,
    pub visible_range: f32,
    pub pathing_order: u32,
    pub enabled_validation: bool,
    pub find_alternate_paths: bool,
}

impl Default for SimulationInputs {
    fn default() -> Self {
        Self {
            flags: SimulationFlags::default(),
            direct_flags: DirectSimulationFlags::default(),
            source: Orientation::default(),
            distance_attenuation_model: DistanceAttenuationModel::default(),
            air_absorption_model: AirAbsorptionModel::default(),
            directivity: Directivity::default(),
            occlusion_type: OcclusionType::default(),
            reverb_scale: [0.0, 0.0, 0.0],
            hybrid_reverb_transition_time: 0.0,
            hybrid_reverb_overlap_percent: 0.0,
            baked: false,
            visible_radius: 0.0,
            visible_threshold: 0.0,
            visible_range: 0.0,
            pathing_order: 0,
            enabled_validation: false,
            find_alternate_paths: false,
        }
    }
}

impl Into<ffi::IPLSimulationInputs> for &SimulationInputs {
    fn into(self) -> ffi::IPLSimulationInputs {
        let mut ffi_occlusion_radius = 0.0;
        let mut ffi_num_occlusion_samples = 0;
        match self.occlusion_type {
            OcclusionType::Volumetric {
                occlusion_radius,
                num_occlusion_samples,
            } => {
                ffi_occlusion_radius = occlusion_radius;
                ffi_num_occlusion_samples = num_occlusion_samples;
            }
            _ => {}
        }

        ffi::IPLSimulationInputs {
            flags: self.flags.into(),
            directFlags: self.direct_flags.into(),
            source: self.source.clone().into(),
            distanceAttenuationModel: self.distance_attenuation_model.clone().into(),
            airAbsorptionModel: self.air_absorption_model.clone().into(),
            directivity: self.directivity.clone().into(),
            occlusionType: self.occlusion_type.into(),
            occlusionRadius: ffi_occlusion_radius,
            numOcclusionSamples: ffi_num_occlusion_samples as i32,
            reverbScale: self.reverb_scale,
            hybridReverbTransitionTime: self.hybrid_reverb_transition_time,
            hybridReverbOverlapPercent: self.hybrid_reverb_overlap_percent,
            baked: self.baked.into(),
            bakedDataIdentifier: ffi::IPLBakedDataIdentifier {
                type_: ffi::IPLBakedDataType::IPL_BAKEDDATATYPE_PATHING,
                variation: ffi::IPLBakedDataVariation::IPL_BAKEDDATAVARIATION_DYNAMIC,
                endpointInfluence: ffi::IPLSphere {
                    center: glam::Vec3::ZERO.into(),
                    radius: 0.0,
                },
            },
            pathingProbes: std::ptr::null_mut(),
            visRadius: self.visible_radius,
            visThreshold: self.visible_threshold,
            visRange: self.visible_range,
            pathingOrder: self.pathing_order as i32,
            enableValidation: self.enabled_validation.into(),
            findAlternatePaths: self.find_alternate_paths.into(),
        }
    }
}
