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
    inner: ffi::IPLScene,
    settings: ffi::IPLSceneSettings,
}

unsafe impl Send for Scene {}
unsafe impl Sync for Scene {}

impl crate::SteamAudioObject for Scene {
    type Object = ffi::IPLScene;
    fn inner_raw(&self) -> Self::Object {
        assert!(!self.inner.is_null());
        self.inner
    }
    fn inner_mut(&mut self) -> &mut Self::Object {
        &mut self.inner
    }
}

impl Scene {
    pub fn new(context: &mut Context, settings: &SceneSettings) -> Result<Self, SteamAudioError> {
        let mut ipl_settings: ffi::IPLSceneSettings = settings.into();
        let mut scene = Self {
            inner: std::ptr::null_mut(),
            settings: ipl_settings,
        };

        unsafe {
            match ffi::iplSceneCreate(context.inner_raw(), &mut scene.settings, scene.inner_mut()) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(scene),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub fn commit(&mut self) {
        unsafe {
            ffi::iplSceneCommit(self.inner_raw());
        }
    }

    pub fn add_static_mesh(&self, static_mesh: &StaticMesh) {
        unsafe {
            ffi::iplStaticMeshAdd(static_mesh.inner_raw(), self.inner_raw());
        }
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            ffi::iplSceneRelease(self.inner_mut());
        }
    }
}
