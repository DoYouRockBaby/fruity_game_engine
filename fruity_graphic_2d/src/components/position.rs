use fruity_any::*;
use fruity_ecs::*;
use fruity_introspect::*;

#[derive(Debug, Clone, Component, IntrospectFields, FruityAny)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
