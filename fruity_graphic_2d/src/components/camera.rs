use fruity_any::*;
use fruity_core::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: f32::MIN,
            far: f32::MAX,
        }
    }
}
