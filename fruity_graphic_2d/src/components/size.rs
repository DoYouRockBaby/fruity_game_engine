use fruity_any::*;
use fruity_ecs::*;
use fruity_introspect::*;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}