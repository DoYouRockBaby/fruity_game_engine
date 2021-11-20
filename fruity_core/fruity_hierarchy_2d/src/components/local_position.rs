use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic_2d::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, Default, FruityAny)]
pub struct LocalPosition {
    pub pos: Vector2d,
}
