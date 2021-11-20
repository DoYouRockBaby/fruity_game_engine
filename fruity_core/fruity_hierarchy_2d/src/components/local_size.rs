use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic_2d::math::vector2d::Vector2d;

#[derive(Debug, Clone, Default, Component, FruityAny)]
pub struct LocalSize {
    pub size: Vector2d,
}
