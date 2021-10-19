use fruity_any::*;
use fruity_ecs::*;
use fruity_introspect::*;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Camera {
    pub near: f32,
    pub far: f32,
}
