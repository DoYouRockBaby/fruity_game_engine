use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Rotate2d {
    pub angle: f32,
}
