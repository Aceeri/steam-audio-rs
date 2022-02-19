use glam::Vec3;
use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct StaticMeshSettings {
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<[i32; 3]>,
    pub material_indices: Vec<i32>,
    pub materials: Vec<Material>,
}

impl Into<StoredStaticMeshSettings> for StaticMeshSettings {
    fn into(self) -> StoredStaticMeshSettings {
        StoredStaticMeshSettings {
            vertices: self.vertices.iter().map(|v| v.into()).collect(),
            triangles: self
                .triangles
                .iter()
                .map(|arr| ffi::IPLTriangle { indices: *arr })
                .collect(),
            material_indices: self.material_indices,
            materials: self.materials.iter().map(|m| m.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
struct StoredStaticMeshSettings {
    vertices: Vec<ffi::IPLVector3>,
    triangles: Vec<ffi::IPLTriangle>,
    material_indices: Vec<i32>,
    materials: Vec<ffi::IPLMaterial>,
}

impl Into<ffi::IPLStaticMeshSettings> for &mut StoredStaticMeshSettings {
    fn into(self) -> ffi::IPLStaticMeshSettings {
        ffi::IPLStaticMeshSettings {
            numVertices: self.vertices.len() as i32,
            numTriangles: self.triangles.len() as i32,
            numMaterials: self.materials.len() as i32,
            vertices: self.vertices.as_mut_ptr(),
            triangles: self.triangles.as_mut_ptr(),
            materialIndices: self.material_indices.as_mut_ptr(),
            materials: self.materials.as_mut_ptr(),
        }
    }
}

pub struct StaticMesh {
    inner: ffi::IPLStaticMesh,

    // Just to be safe we store these in the format that steam audio likes.
    //
    // We also need to keep this here so the pointers don't randomly die when the mesh settings get dropped.
    settings: StoredStaticMeshSettings,
}

impl StaticMesh {
    pub fn new(scene: &Scene, settings: StaticMeshSettings) -> Result<Self, SteamAudioError> {
        let mut mesh = Self {
            inner: unsafe { std::mem::zeroed() },
            settings: settings.into(),
        };

        let mut ipl_settings: ffi::IPLStaticMeshSettings = (&mut mesh.settings).into();

        unsafe {
            match ffi::iplStaticMeshCreate(scene.inner(), &mut ipl_settings, &mut mesh.inner()) {
                ffi::IPLerror::IPL_STATUS_SUCCESS => Ok(mesh),
                err => Err(SteamAudioError::IPLError(err)),
            }
        }
    }

    pub unsafe fn inner(&self) -> ffi::IPLStaticMesh {
        self.inner
    }
}

impl Drop for StaticMesh {
    fn drop(&mut self) {
        unsafe {
            ffi::iplStaticMeshRelease(&mut self.inner());
        }
    }
}
