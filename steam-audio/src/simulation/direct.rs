use steam_audio_sys::ffi;

use bitflags::bitflags;

bitflags! {
    pub struct DirectSimulationFlags: i32 {
        const DISTANCE_ATTENUATION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_DISTANCEATTENUATION.0;
        const AIR_ABSORPTION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_AIRABSORPTION.0;
        const DIRECTIVITY = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_DIRECTIVITY.0;
        const OCCLUSION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_OCCLUSION.0;
        const TRANSMISSION = ffi::IPLDirectSimulationFlags::IPL_DIRECTSIMULATIONFLAGS_TRANSMISSION.0;

        const ALL = DirectSimulationFlags::DISTANCE_ATTENUATION.bits()
            | DirectSimulationFlags::AIR_ABSORPTION.bits()
            | DirectSimulationFlags::DIRECTIVITY.bits()
            | DirectSimulationFlags::OCCLUSION.bits()
            | DirectSimulationFlags::TRANSMISSION.bits();
        const DEFAULT = DirectSimulationFlags::DISTANCE_ATTENUATION.bits();
    }
}

impl Default for DirectSimulationFlags {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Into<ffi::IPLDirectSimulationFlags> for DirectSimulationFlags {
    fn into(self) -> ffi::IPLDirectSimulationFlags {
        ffi::IPLDirectSimulationFlags(self.bits())
    }
}
