use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct Scale2d {
    pub vec: Vector2d,
}

impl Default for Scale2d {
    fn default() -> Self {
        Scale2d {
            vec: Vector2d::new(1.0, 1.0),
        }
    }
}
