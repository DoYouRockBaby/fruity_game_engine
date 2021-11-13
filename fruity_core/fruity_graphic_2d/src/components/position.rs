use crate::Vector2d;
use fruity_any::*;
use fruity_ecs::*;

#[derive(Debug, Clone, Component, Default, FruityAny)]
pub struct Position {
    pub pos: Vector2d,
}
