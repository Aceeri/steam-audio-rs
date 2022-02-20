use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum SceneSettings {
    Default,
    /*
    TODO: implement this stuff
    not doing it now because its probably not super necessary, Embree would be nice
    we'd need to add some more stuff for that though.
    Embree,
    RadeonRays,
    Custom,
    */
}

impl Default for SceneSettings {
    fn default() -> Self {
        Self::Default
    }
}

impl Into<ffi::IPLSceneSettings> for &SceneSettings {
    fn into(self) -> ffi::IPLSceneSettings {
        let mut model = ffi::IPLSceneSettings {
            type_: ffi::IPLSceneType::IPL_SCENETYPE_DEFAULT,
            closestHitCallback: None,
            anyHitCallback: None,
            batchedAnyHitCallback: None,
            batchedClosestHitCallback: None,
            userData: std::ptr::null_mut(),
            embreeDevice: std::ptr::null_mut(),
            radeonRaysDevice: std::ptr::null_mut(),
        };

        match self {
            SceneSettings::Default => {
                model.type_ = ffi::IPLSceneType::IPL_SCENETYPE_DEFAULT;
            } /*
              Self::Embree => { }
              Self::RadeonRays => { }
              Self::Custom => { }
              */
        }

        model
    }
}

/*
pub struct StoredSceneSettings {
    pub type_: IPLSceneType,
    pub closestHitCallback: IPLClosestHitCallback,
    pub anyHitCallback: IPLAnyHitCallback,
    pub batchedClosestHitCallback: IPLBatchedClosestHitCallback,
    pub batchedAnyHitCallback: IPLBatchedAnyHitCallback,
    pub userData: *mut ::std::os::raw::c_void,
    pub embreeDevice: IPLEmbreeDevice,
    pub radeonRaysDevice: IPLRadeonRaysDevice,
}
*/

pub struct Scene {
    pub(crate) inner: ffi::IPLScene,
    settings: ffi::IPLSceneSettings,
}

impl Scene {
    pub fn new(context: &mut Context, settings: &SceneSettings) -> Result<Self, SteamAudioError> {
        let mut ipl_settings: ffi::IPLSceneSettings = settings.into();
        let mut scene = Self {
            inner: unsafe { std::mem::zeroed() },
            settings: ipl_settings,
        };

        unsafe {
            match ffi::iplSceneCreate(context.inner, &mut scene.settings, &mut scene.inner) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(scene),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLScene {
        self.inner
    }

    pub fn commit(&mut self) {
        unsafe {
            ffi::iplSceneCommit(self.inner);
        }
    }

    pub fn add_static_mesh(&self, static_mesh: &StaticMesh) {
        unsafe {
            ffi::iplStaticMeshAdd(static_mesh.inner, self.inner);
        }
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSceneRelease(&mut self.inner);
        }
    }
}
