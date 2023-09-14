use std::ffi::CString;

use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Copy, Clone)]
pub enum HRTFInterpolation {
    NearestNeighbor,
    Bilinear,
}

impl Into<ffi::IPLHRTFInterpolation> for HRTFInterpolation {
    fn into(self) -> ffi::IPLHRTFInterpolation {
        match self {
            Self::NearestNeighbor => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_NEAREST,
            Self::Bilinear => ffi::IPLHRTFInterpolation::IPL_HRTFINTERPOLATION_BILINEAR,
        }
    }
}

// TODO: Expose normType here.
pub enum HRTFSettings {
    Default {
        volume: f32,
    },
    SOFA {
        // TODO: Expose sofaData here.
        path: String,
        volume: f32,
    },
}

impl Default for HRTFSettings {
    fn default() -> Self {
        Self::Default { volume: 1.0 }
    }
}

impl Into<ffi::IPLHRTFSettings> for &HRTFSettings {
    fn into(self) -> ffi::IPLHRTFSettings {
        let mut settings = ffi::IPLHRTFSettings {
            type_: ffi::IPLHRTFType::IPL_HRTFTYPE_DEFAULT,
            sofaFileName: std::ptr::null_mut(),
            sofaData: std::ptr::null_mut(),
            sofaDataSize: 0,
            volume: 1.0,
            normType: ffi::IPLHRTFNormType::IPL_HRTFNORMTYPE_NONE,
        };

        match self {
            HRTFSettings::Default { volume } => {
                settings.type_ = ffi::IPLHRTFType::IPL_HRTFTYPE_DEFAULT;
                settings.volume = *volume;
            }
            HRTFSettings::SOFA { path, volume } => {
                settings.type_ = ffi::IPLHRTFType::IPL_HRTFTYPE_SOFA;
                settings.volume = *volume;

                let path = CString::new(path.clone())
                    .expect("interior nul byte in path");
                settings.sofaFileName = path.as_ptr();
            }
        };

        settings
    }
}

pub struct AudioSettings {
    sampling_rate: u32,
    frame_size: u32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            sampling_rate: 44100,
            frame_size: 1024,
        }
    }
}

impl Into<ffi::IPLAudioSettings> for &AudioSettings {
    fn into(self) -> ffi::IPLAudioSettings {
        ffi::IPLAudioSettings {
            samplingRate: self.sampling_rate as i32,
            frameSize: self.frame_size as i32,
        }
    }
}

impl AudioSettings {
    pub fn sampling_rate(&self) -> u32 {
        self.sampling_rate
    }

    pub fn frame_size(&self) -> u32 {
        self.frame_size
    }
}

pub struct HRTF {
    inner: ffi::IPLHRTF,
    hrtf_settings: ffi::IPLHRTFSettings,
    audio_settings: ffi::IPLAudioSettings,
}

unsafe impl Send for HRTF {}
unsafe impl Sync for HRTF {}

impl HRTF {
    pub fn new(
        context: &Context,
        audio_settings: &AudioSettings,
        hrtf_settings: &HRTFSettings,
    ) -> Result<Self, SteamAudioError> {
        let hrtf_ipl_settings: ffi::IPLHRTFSettings = hrtf_settings.into();
        let audio_ipl_settings: ffi::IPLAudioSettings = audio_settings.into();
        let mut hrtf = Self {
            inner: std::ptr::null_mut(),
            hrtf_settings: hrtf_ipl_settings,
            audio_settings: audio_ipl_settings,
        };

        unsafe {
            match ffi::iplHRTFCreate(
                context.inner_raw(),
                &mut hrtf.audio_settings,
                &mut hrtf.hrtf_settings,
                &mut hrtf.inner,
            ) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(hrtf),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }
}

impl crate::SteamAudioObject for HRTF {
    type Object = ffi::IPLHRTF;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> *mut Self::Object {
        std::ptr::addr_of_mut!(self.inner)
    }
}

impl Drop for HRTF {
    fn drop(&mut self) {
        unsafe {
            ffi::iplHRTFRelease(self.inner_mut());
        }
    }
}
