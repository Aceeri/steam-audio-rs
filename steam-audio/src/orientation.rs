use steam_audio_sys::ffi;

use glam::Vec3;

#[derive(Debug, Clone)]
pub struct Orientation {
    pub right: Vec3,
    pub up: Vec3,
    pub ahead: Vec3,
    pub origin: Vec3,
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            right: Vec3::X,
            up: Vec3::Y,
            ahead: -Vec3::Z,
            origin: Vec3::ZERO,
        }
    }
}

impl Into<ffi::IPLCoordinateSpace3> for Orientation {
    fn into(self) -> ffi::IPLCoordinateSpace3 {
        ffi::IPLCoordinateSpace3 {
            right: self.right.into(),
            up: self.up.into(),
            ahead: self.ahead.into(),
            origin: self.origin.into(),
        }
    }
}
