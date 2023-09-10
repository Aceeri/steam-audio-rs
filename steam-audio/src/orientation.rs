use steam_audio_sys::ffi;

#[derive(Debug, Clone)]
pub struct Orientation {
    pub right: [f32; 3],
    pub up: [f32; 3],
    pub ahead: [f32; 3],
    pub origin: [f32; 3],
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            right: [1.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            ahead: [0.0, 0.0, -1.0],
            origin: [0.0, 0.0, 0.0],
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
