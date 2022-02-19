use steam_audio_sys::ffi;

use crate::prelude::*;

/// Acoustic properties of a surface.
#[derive(Debug, Clone)]
pub struct Material {
    // Specified in 3 frequency bands of 400 Hz, 2.5KHz, and 15 KHz.
    pub absorption: [f32; 3],
    pub scattering: f32,
    pub transmission: [f32; 3],
}

impl Into<ffi::IPLMaterial> for &Material {
    fn into(self) -> ffi::IPLMaterial {
        ffi::IPLMaterial {
            absorption: self.absorption,
            scattering: self.scattering,
            transmission: self.transmission,
        }
    }
}

pub mod materials {
    use super::Material;

    pub const GENERIC: Material = Material {
        absorption: [0.10, 0.20, 0.30],
        scattering: 0.05,
        transmission: [0.100, 0.050, 0.030],
    };
    pub const BRICK: Material = Material {
        absorption: [0.03, 0.04, 0.07],
        scattering: 0.05,
        transmission: [0.015, 0.015, 0.015],
    };
    pub const CONCRETE: Material = Material {
        absorption: [0.05, 0.07, 0.08],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    };
    pub const CERAMIC: Material = Material {
        absorption: [0.01, 0.02, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    };
    pub const GRAVEL: Material = Material {
        absorption: [0.60, 0.70, 0.80],
        scattering: 0.05,
        transmission: [0.031, 0.012, 0.008],
    };
    pub const CARPET: Material = Material {
        absorption: [0.24, 0.69, 0.73],
        scattering: 0.05,
        transmission: [0.020, 0.005, 0.003],
    };
    pub const GLASS: Material = Material {
        absorption: [0.06, 0.03, 0.02],
        scattering: 0.05,
        transmission: [0.060, 0.044, 0.011],
    };
    pub const PLASTER: Material = Material {
        absorption: [0.12, 0.06, 0.04],
        scattering: 0.05,
        transmission: [0.056, 0.056, 0.004],
    };
    pub const WOOD: Material = Material {
        absorption: [0.11, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.070, 0.014, 0.005],
    };
    pub const METAL: Material = Material {
        absorption: [0.20, 0.07, 0.06],
        scattering: 0.05,
        transmission: [0.200, 0.025, 0.010],
    };
    pub const ROCK: Material = Material {
        absorption: [0.13, 0.20, 0.24],
        scattering: 0.05,
        transmission: [0.015, 0.002, 0.001],
    };
}
