use glam::Vec3;
use steam_audio_sys::ffi;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct StaticMeshSettings {
    pub num_vertices: u32,
    pub num_triangles: u32,
    pub num_materials: u32,
    pub vertices: Vec<Vec3>,
    pub triangles: Vec<[i32; 3]>,
    pub material_indices: Vec<i32>,
    pub materials: Vec<Material>,
}

impl Into<StoredStaticMeshSettings> for StaticMeshSettings {
    fn into(self) -> StoredStaticMeshSettings {
        StoredStaticMeshSettings {
            num_vertices: self.num_vertices as i32,
            num_triangles: self.num_vertices as i32,
            num_materials: self.num_materials as i32,
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
pub struct StoredStaticMeshSettings {
    pub num_vertices: i32,
    pub num_triangles: i32,
    pub num_materials: i32,
    pub vertices: Vec<ffi::IPLVector3>,
    pub triangles: Vec<ffi::IPLTriangle>,
    pub material_indices: Vec<i32>,
    pub materials: Vec<ffi::IPLMaterial>,
}

impl Into<ffi::IPLStaticMeshSettings> for &mut StoredStaticMeshSettings {
    fn into(self) -> ffi::IPLStaticMeshSettings {
        ffi::IPLStaticMeshSettings {
            numVertices: self.num_vertices as i32,
            numTriangles: self.num_vertices as i32,
            numMaterials: self.num_materials as i32,
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
