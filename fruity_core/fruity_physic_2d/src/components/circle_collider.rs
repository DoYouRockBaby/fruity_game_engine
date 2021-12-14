use fruity_any::*;
use fruity_ecs::*;
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, FruityAny)]
pub struct CircleCollider {
    pub center: Vector2d,
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self {
            center: Vector2d::new(0.0, 0.0),
            radius: 1.0,
        }
    }
}
