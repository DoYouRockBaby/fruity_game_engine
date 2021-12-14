use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct RectCollider {
    pub bottom_left: Vector2d,
    pub top_right: Vector2d,
}

impl Default for RectCollider {
    fn default() -> Self {
        Self {
            bottom_left: Vector2d::new(-0.5, -0.5),
            top_right: Vector2d::new(0.5, 0.5),
        }
    }
}
