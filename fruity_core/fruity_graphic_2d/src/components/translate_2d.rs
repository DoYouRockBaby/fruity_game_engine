use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, Default, FruityAny)]
pub struct Translate2d {
    pub vec: Vector2d,
}
