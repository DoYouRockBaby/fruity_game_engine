use crate::Vector2d;
use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct Size {
    pub size: Vector2d,
}
