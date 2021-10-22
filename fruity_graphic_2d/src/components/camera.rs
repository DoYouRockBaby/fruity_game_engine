use fruity_any::*;
use fruity_core::*;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
}
