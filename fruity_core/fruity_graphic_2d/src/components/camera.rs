use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: -1.0,
            far: 1.0,
        }
    }
}
